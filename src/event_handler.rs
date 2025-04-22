use aws_config::BehaviorVersion;
use aws_sdk_eventbridge::types::PutEventsRequestEntry;
use lambda_runtime::{tracing, Error, LambdaEvent};
use aws_lambda_events::event::eventbridge::EventBridgeEvent;

pub(crate)async fn function_handler(event: LambdaEvent<EventBridgeEvent>) -> Result<(), Error> {
    // Extract some useful information from the request
    let payload = event.payload;
    tracing::info!("Payload: {:?}", payload);

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = aws_sdk_eventbridge::Client::new(&config);

    let request_id = payload.detail["request_id"].as_str().unwrap();
    let event = format!(
        r#"
    {{
        "request_id": "{request_id}"
    }}"#
    );

    println!("PutEvent: {}", event);

    let input = PutEventsRequestEntry::builder()
        .detail(event)
        .detail_type("tdx_quote".to_string())
        .event_bus_name("tdx-prover".to_string())
        .source("com.magic.newton".to_string())
        .build();

    match client.put_events().entries(input).send().await {
        Ok(result) => {
            println!("Event sent: {}", result.failed_entry_count);
        }
        Err(err) => match err.into_service_error() {
            aws_sdk_eventbridge::operation::put_events::PutEventsError::InternalException(e) => {
                println!("eventbridge error: {:?}", &e.message().unwrap());
            }
            _ => {},
        },
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_runtime::{Context, LambdaEvent};

    #[tokio::test]
    async fn test_event_handler() {
        let event = LambdaEvent::new(EventBridgeEvent::default(), Context::default());
        let response = function_handler(event).await.unwrap();
        assert_eq!((), response);
    }
}
