use std::env;

use actix_governor::{GovernorConfig, GovernorConfigBuilder};
use actix_web::{dev::ServiceRequest, HttpResponse, HttpResponseBuilder};
use governor::{
    clock::{Clock, DefaultClock, QuantaInstant},
    middleware::StateInformationMiddleware,
    NotUntil,
};

use crate::errors::TodoError as TodoErrorTrait;
use crate::{errors::Error as TodoError, schemas::errors::ErrorSchema};

#[derive(Debug, Clone)]
pub struct IpAddressExtractor;

impl actix_governor::KeyExtractor for IpAddressExtractor {
    type Key = String;
    type KeyExtractionError = TodoError;

    fn extract(&self, req: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        req.connection_info()
            .peer_addr()
            .map(Into::into)
            .server_err("Could not get IP address")
    }

    fn exceed_rate_limit_response(
        &self,
        negative: &NotUntil<QuantaInstant>,
        mut res: HttpResponseBuilder,
    ) -> HttpResponse {
        let wait_time = negative
            .wait_time_from(DefaultClock::default().now())
            .as_secs();
        res.json(ErrorSchema::from(TodoError::TooManyRequests(wait_time)))
    }
}

/// Initializes IP rate limiter
pub fn init_ip() -> GovernorConfig<IpAddressExtractor, StateInformationMiddleware> {
    GovernorConfigBuilder::default()
        .key_extractor(IpAddressExtractor)
        .per_second(
            env::var("RATE_LIMIT_PER_SECOND")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .expect("Invalid rate limit per second"),
        )
        .burst_size(
            env::var("RATE_LIMIT_BURST_SIZE")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .expect("Invalid rate limit burst size"),
        )
        .use_headers()
        .finish()
        .unwrap()
}
