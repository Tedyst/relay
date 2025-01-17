//! Returns captured events.

use ::actix::prelude::*;
use actix_web::{http::Method, HttpResponse, Path};
use futures::future::Future;

use crate::actors::events::GetCapturedEnvelope;
use crate::envelope;
use crate::extractors::CurrentServiceState;
use crate::service::ServiceApp;

use relay_general::protocol::EventId;

fn get_captured_event(
    state: CurrentServiceState,
    event_id: Path<EventId>,
) -> ResponseFuture<HttpResponse, actix::MailboxError> {
    let future = state
        .event_manager()
        .send(GetCapturedEnvelope {
            event_id: *event_id,
        })
        .map(|captured_event| match captured_event {
            Some(Ok(envelope)) => HttpResponse::Ok()
                .content_type(envelope::CONTENT_TYPE)
                .body(envelope.to_vec().unwrap()),
            Some(Err(error)) => HttpResponse::BadRequest()
                .content_type("text/plain")
                .body(error),
            None => HttpResponse::NotFound().finish(),
        });

    Box::new(future)
}

pub fn configure_app(app: ServiceApp) -> ServiceApp {
    app.resource("/api/relay/events/{event_id}/", |r| {
        r.name("internal-events");
        r.method(Method::GET).with(get_captured_event);
    })
}
