#[macro_use]
extern crate lazy_static;

use bytes::BufMut;
use futures::TryStreamExt;
use warp::hyper::StatusCode;
use warp::multipart::Part;
use std::convert::Infallible;
use std::net::SocketAddr;
use uuid::Uuid;
use warp::Reply;
use warp::Rejection;
use warp::multipart::FormData;
use warp::Filter;

mod appconfig;
mod audioutils;
mod errors;
mod file_namer;
mod ziputils;

#[tokio::main]
async fn main() {
    let config = &appconfig::CONFIG;

    let port = config.port;
    let interface = &config.interface;
    let address = format!("{}:{}", interface, port);
    let socket_addr = match address.parse::<SocketAddr>() {
        Ok(a) => a,
        Err(_) => { panic!("Invalid inteface/port format, check your config file: {}", address); }
    };

    let index = warp::get()
        .and(warp::path::end())
        .map(|| "Hello world!");

    let upload_route = warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(100 * 1024 * 1024))
        .and_then(upload);

    let routes = index.or(upload_route).recover(handle_rejection);
    println!("Server started at {}", address);
    warp::serve(routes).run(socket_addr).await;
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

        if !(ziputils::is_zipfile(&value)) {
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
        let target_dir = base_upload_dir.join(format!("extracted-{}", file_uuid));
        match ziputils::unpack_zipfile(&file_name, &target_dir, &appconfig::CONFIG.output.filename_pattern).await {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Something went wrong extracting the zipfile: {}", e);
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
