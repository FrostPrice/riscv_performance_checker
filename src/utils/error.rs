use actix_web::{http::StatusCode, HttpResponse};

use crate::models::response::ResponseBody;

pub struct ServiceError {
    pub http_status: StatusCode,
    pub body: ResponseBody<String>,
}

impl ServiceError {
    pub fn new(http_status: StatusCode, message: String) -> ServiceError {
        ServiceError {
            http_status,
            body: ResponseBody {
                message,
                data: String::new(),
            },
        }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::build(self.http_status).json(&self.body)
    }
}
