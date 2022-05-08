#![deny(warnings)]

extern crate tree_magic;

use std::fs::File;

use bytes::BufMut;
use futures::TryStreamExt;
use warp::hyper::StatusCode;
use warp::multipart::Part;
use std::convert::Infallible;
use uuid::Uuid;
use warp::Reply;
use warp::Rejection;
use warp::multipart::FormData;
use warp::Filter;

#[tokio::main]
async fn main() {
    let port = 5551;

    let index = warp::get()
        .and(warp::path::end())
        .map(|| "Hello world!");

    let upload_route = warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(100 * 1024 * 1024))
        .and_then(upload);

    let routes = index.or(upload_route).recover(handle_rejection);
    println!("Server started at localhost:{}", port);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}

async fn upload(form: FormData) -> Result<impl Reply, Rejection> {
    let base_upload_dir = std::env::temp_dir().join("music_upload").join("files");

    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    })?;

    for p in parts {
        if p.name() != "file" {
            // We really only care about the file upload right now.
            // In the future, maybe we'll extract information about the file from
            //  request headers or something.
            continue;
        }

        let content_type = p.content_type();
        let file_ending;
        match content_type {
            Some(file_type) => match file_type {
                "application/zip" | "application/octet-stream" => {
                    file_ending = "zip";
                }
                v => {
                    eprintln!("invalid file type found: {}", v);
                    return Err(warp::reject::reject());
                }
            },
            None => {
                eprintln!("file type could not be determined");
                return Err(warp::reject::reject());
            }
        }

        let value = p.stream()
            .try_fold(Vec::new(), |mut vec, data| {
                vec.put(data);
                async move { Ok(vec) }
            })
            .await
            .map_err(|e| {
                eprintln!("reading file error: {}", e);
                warp::reject::reject()
            })?;

        let uploaded_file_mimetype = tree_magic::from_u8(&value);
        if uploaded_file_mimetype != "application/zip" {
            eprintln!("Non-zipfile upload detected");
            return Err(warp::reject::reject());
        }
        let file_uuid = Uuid::new_v4().to_string();
        let file_name = base_upload_dir.join(format!("{}.{}", file_uuid, file_ending));
        match tokio::fs::create_dir_all(&base_upload_dir).await {
            Ok(_) => {},
            Err(e) => {
                eprintln!(
                    "error creating file upload directory {}: {}",
                    base_upload_dir.to_string_lossy(),
                    e
                );
            }
        }
        match tokio::fs::write(&file_name, value).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("error writing file: {}", e);
                return Err(warp::reject::reject());
            }
        };

        println!("Uploaded zip file: {}", file_name.to_string_lossy());
        println!("Unpacking zip file {} now", file_name.to_string_lossy());
        let file = File::open(&file_name).unwrap();
        let archive = zip::ZipArchive::new(file);
        match archive {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Something is wrong with the uploaded zip file: {}", e);
                return Err(warp::reject::reject());
            }
        }
        // TOOD: we should validate the contents of the zip file!
        let target_dir = base_upload_dir.join(format!("extracted-{}", file_uuid));
        match tokio::fs::create_dir_all(&target_dir).await {
            Ok(_) => {},
            Err(e) => {
                eprintln!("failed to create extract dir {}: {}", target_dir.to_string_lossy(), e);
                return Err(warp::reject::reject());
            }
        };

        match archive.unwrap().extract(&target_dir) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("failed to extract {}: {}", file_name.to_string_lossy(), e);
                return Err(warp::reject::reject());
            }
        }
        println!("Unpacked {} to {}", file_name.to_string_lossy(), target_dir.to_string_lossy());
    }

    Ok("success")
}

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = 
        if err.is_not_found() {
            (StatusCode::NOT_FOUND, "Not Found".to_string())
        } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
            (StatusCode::BAD_REQUEST, "Payload too large".to_string())
        } else {
            eprintln!("unhandled error: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string()
            )
        };

    Ok(warp::reply::with_status(message, code))
}
