use crate::{
    debug_root_span, error_root_span, info_root_span, root_span, trace_root_span, warn_root_span,
};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::StatusCode;
use actix_web::{Error, ResponseError};
use tracing::Span;

/// `RootSpanBuilder` allows you to customize the root span attached by
/// [`TracingLogger`] to incoming requests.
///
/// [`TracingLogger`]: crate::TracingLogger
pub trait RootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span;
    fn on_request_end<B>(span: Span, outcome: &Result<ServiceResponse<B>, Error>);
}

/// The default [`RootSpanBuilder`] for [`TracingLogger`].
///
/// It logs all captures at the info level.
///
/// It captures:
/// - HTTP method (`http.method`);
/// - HTTP route (`http.route`), with templated parameters;
/// - HTTP version (`http.flavor`);
/// - HTTP host (`http.host`);
/// - Client IP (`http.client_ip`);
/// - User agent (`http.user_agent`);
/// - Request path (`http.target`);
/// - Status code (`http.status_code`);
/// - [Request id](crate::RequestId) (`request_id`);
/// - `Display` (`exception.message`) and `Debug` (`exception.details`) representations of the error, if there was an error;
/// - [Request id](crate::RequestId) (`request_id`);
/// - [OpenTelemetry trace identifier](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/overview.md#spancontext) (`trace_id`). Empty if the feature is not enabled;
/// - OpenTelemetry span kind, set to `server` (`otel.kind`).
///
/// All field names follow [OpenTelemetry's semantic convention](https://github.com/open-telemetry/opentelemetry-specification/tree/main/specification/trace/semantic_conventions).
///
/// [`TracingLogger`]: crate::TracingLogger
pub struct DefaultRootSpanBuilder;

impl RootSpanBuilder for DefaultRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        root_span!(request)
    }

    fn on_request_end<B>(span: Span, outcome: &Result<ServiceResponse<B>, Error>) {
        match &outcome {
            Ok(response) => {
                if let Some(error) = response.response().error() {
                    // use the status code already constructed for the outgoing HTTP response
                    handle_error(span, response.status(), error.as_response_error());
                } else {
                    let code: i32 = response.response().status().as_u16().into();
                    span.record("http.status_code", &code);
                    span.record("otel.status_code", &"OK");
                }
            }
            Err(error) => {
                let response_error = error.as_response_error();
                handle_error(span, response_error.status_code(), response_error);
            }
        };
    }
}

/// A [`RootSpanBuilder`] for [`TracingLogger`] that logs at the trace level.
///
/// Besides the log level, this span builder is equivalent to [`DefaultRootSpanBuilder`].
///
/// To use this span builder, use it as the type argument to [`TracingLogger`].
///
/// ```rust
/// # use tracing_actix_web::{TracingLogger, TraceRootSpanBuilder};
/// let logger = TracingLogger::<TraceRootSpanBuilder>::new();
/// ```
///
/// [`TracingLogger`]: crate::TracingLogger
pub struct TraceRootSpanBuilder;

impl RootSpanBuilder for TraceRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        trace_root_span!(request)
    }

    fn on_request_end<B>(span: Span, outcome: &Result<ServiceResponse<B>, Error>) {
        DefaultRootSpanBuilder::on_request_end(span, outcome)
    }
}

/// A [`RootSpanBuilder`] for [`TracingLogger`] that logs at the debug level.
///
/// Besides the log level, this span builder is equivalent to [`DefaultRootSpanBuilder`].
///
/// To use this span builder, use it as the type argument to [`TracingLogger`].
///
/// ```rust
/// # use tracing_actix_web::{TracingLogger, DebugRootSpanBuilder};
/// let logger = TracingLogger::<DebugRootSpanBuilder>::new();
/// ```
///
/// [`TracingLogger`]: crate::TracingLogger
pub struct DebugRootSpanBuilder;

impl RootSpanBuilder for DebugRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        debug_root_span!(request)
    }

    fn on_request_end<B>(span: Span, outcome: &Result<ServiceResponse<B>, Error>) {
        DefaultRootSpanBuilder::on_request_end(span, outcome)
    }
}

/// A [`RootSpanBuilder`] for [`TracingLogger`] that logs at the info level.
///
/// Besides the log level, this span builder is equivalent to [`DefaultRootSpanBuilder`].
///
/// To use this span builder, use it as the type argument to [`TracingLogger`].
///
/// ```rust
/// # use tracing_actix_web::{TracingLogger, InfoRootSpanBuilder};
/// let logger = TracingLogger::<InfoRootSpanBuilder>::new();
/// ```
///
/// [`TracingLogger`]: crate::TracingLogger
pub struct InfoRootSpanBuilder;

impl RootSpanBuilder for InfoRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        info_root_span!(request)
    }

    fn on_request_end<B>(span: Span, outcome: &Result<ServiceResponse<B>, Error>) {
        DefaultRootSpanBuilder::on_request_end(span, outcome)
    }
}

/// A [`RootSpanBuilder`] for [`TracingLogger`] that logs at the warn level.
///
/// Besides the log level, this span builder is equivalent to [`DefaultRootSpanBuilder`].
///
/// To use this span builder, use it as the type argument to [`TracingLogger`].
///
/// ```rust
/// # use tracing_actix_web::{TracingLogger, WarnRootSpanBuilder};
/// let logger = TracingLogger::<WarnRootSpanBuilder>::new();
/// ```
///
/// [`TracingLogger`]: crate::TracingLogger
pub struct WarnRootSpanBuilder;

impl RootSpanBuilder for WarnRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        warn_root_span!(request)
    }

    fn on_request_end<B>(span: Span, outcome: &Result<ServiceResponse<B>, Error>) {
        DefaultRootSpanBuilder::on_request_end(span, outcome)
    }
}

/// A [`RootSpanBuilder`] for [`TracingLogger`] that logs at the error level.
///
/// Besides the log level, this span builder is equivalent to [`DefaultRootSpanBuilder`].
///
/// To use this span builder, use it as the type argument to [`TracingLogger`].
///
/// ```rust
/// # use tracing_actix_web::{TracingLogger, ErrorRootSpanBuilder};
/// let logger = TracingLogger::<ErrorRootSpanBuilder>::new();
/// ```
///
/// [`TracingLogger`]: crate::TracingLogger
pub struct ErrorRootSpanBuilder;

impl RootSpanBuilder for ErrorRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        error_root_span!(request)
    }

    fn on_request_end<B>(span: Span, outcome: &Result<ServiceResponse<B>, Error>) {
        DefaultRootSpanBuilder::on_request_end(span, outcome)
    }
}

fn handle_error(span: Span, status_code: StatusCode, response_error: &dyn ResponseError) {
    // pre-formatting errors is a workaround for https://github.com/tokio-rs/tracing/issues/1565
    let display = format!("{}", response_error);
    let debug = format!("{:?}", response_error);
    span.record("exception.message", &tracing::field::display(display));
    span.record("exception.details", &tracing::field::display(debug));
    let code: i32 = status_code.as_u16().into();

    span.record("http.status_code", &code);

    if status_code.is_client_error() {
        span.record("otel.status_code", &"OK");
    } else {
        span.record("otel.status_code", &"ERROR");
    }
}
