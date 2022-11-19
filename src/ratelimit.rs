use actix_governor::{GovernorConfig, GovernorConfigBuilder};
use actix_web::{dev::ServiceRequest, HttpResponse, HttpResponseBuilder, ResponseError};
use governor::{
    clock::{Clock, DefaultClock, QuantaInstant},
    middleware::StateInformationMiddleware,
    NotUntil,
};

use crate::{auth::utils::extract_token, errors::TodoError as TodoErrorTrait};
use crate::{errors::Error as TodoError, schemas::errors::ErrorSchema};

#[derive(Debug, Clone)]
pub struct BearerTokenExtractor;

#[derive(Debug, Clone)]
pub struct IpAddressExtractor;

impl actix_governor::KeyExtractor for BearerTokenExtractor {
    type Key = String;
    type KeyExtractionError = TodoError;

    fn extract(&self, req: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        extract_token(req.request())
    }

    fn exceed_rate_limit_response(
        &self,
        negative: &NotUntil<QuantaInstant>,
        _: HttpResponseBuilder,
    ) -> HttpResponse {
        let wait_time = negative
            .wait_time_from(DefaultClock::default().now())
            .as_secs();
        TodoError::TooManyRequests(wait_time).error_response()
    }
}

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

/// Initializes bearer token rate limiter
pub fn init_bearer() -> GovernorConfig<BearerTokenExtractor, StateInformationMiddleware> {
    GovernorConfigBuilder::default()
        .key_extractor(BearerTokenExtractor)
        .per_second(60)
        .burst_size(100)
        .use_headers()
        .finish()
        .unwrap()
}

/// Initializes IP rate limiter
pub fn init_ip() -> GovernorConfig<IpAddressExtractor, StateInformationMiddleware> {
    GovernorConfigBuilder::default()
        .key_extractor(IpAddressExtractor)
        .per_second(60)
        .burst_size(100)
        .use_headers()
        .finish()
        .unwrap()
}
