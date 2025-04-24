use lambda_runtime::{tracing, Error, LambdaEvent};
use aws_lambda_events::event::eventbridge::EventBridgeEvent;

pub(crate)async fn function_handler(event: LambdaEvent<EventBridgeEvent>) -> Result<(), Error> {
    // Extract some useful information from the request
    let payload = event.payload;
    tracing::info!("Payload: {:?}", payload);
    println!("Payload: {:?}", payload);
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
