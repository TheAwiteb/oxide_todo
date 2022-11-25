use std::{env, future::Ready, time::Duration};

use actix_extensible_rate_limit::{
    backend::{memory::InMemoryBackend, SimpleInput, SimpleInputFunctionBuilder, SimpleOutput},
    HeaderCompatibleOutput, RateLimiter, RateLimiterBuilder,
};
use actix_web::{dev::ServiceRequest, http::StatusCode, HttpResponse};

use crate::{errors::Error as TodoError, schemas::errors::ErrorSchema};

/// The response error for rate limit exceeded
fn rate_limit_exceeded(rate_info: &SimpleOutput) -> HttpResponse {
    let rest_time = rate_info.seconds_until_reset();
    let body = ErrorSchema::from(TodoError::TooManyRequests(rest_time));

    HttpResponse::build(StatusCode::TOO_MANY_REQUESTS)
        .append_header(("x-ratelimit-limit", rate_info.limit()))
        .append_header(("x-ratelimit-remaining", rate_info.remaining()))
        .append_header(("x-ratelimit-reset", rest_time))
        .json(body)
}

/// Initializes IP rate limiter
pub fn init_ip(
    backend: InMemoryBackend,
) -> RateLimiterBuilder<
    InMemoryBackend,
    SimpleOutput,
    impl Fn(&ServiceRequest) -> Ready<Result<SimpleInput, actix_web::Error>>,
> {
    let seconds: u64 = env::var("RATE_LIMIT_PER_SECOND")
        .unwrap_or_else(|_| "60".to_string())
        .parse()
        .expect("Invalid rate limit per second");
    let burst_size: u64 = env::var("RATE_LIMIT_BURST_SIZE")
        .unwrap_or_else(|_| "30".to_string())
        .parse()
        .expect("Invalid rate limit burst size");
    // Assign a limit of `burst_size` requests per `seconds` seconds per client ip address
    let input = SimpleInputFunctionBuilder::new(Duration::from_secs(seconds), burst_size)
        .real_ip_key()
        .build();
    RateLimiter::builder(backend, input)
        .add_headers()
        .request_denied_response(rate_limit_exceeded)
}
