extern crate custom_error;
use custom_error::custom_error;

custom_error!{pub AppError
    InvalidConfig{cause:String} = "Invalid configuration: {cause}"
}
