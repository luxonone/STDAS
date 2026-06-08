use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    code: i32,
    message: &'static str,
    data: T,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Self {
            code: 0_i32,
            message: "success",
            data,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiErrorResponse {
    code: i32,
    message: &'static str,
    data: Option<()>,
}

impl ApiErrorResponse {
    pub fn new(code: i32, message: &'static str) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }
}
