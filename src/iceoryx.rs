use anyhow::Result;
use iceoryx_rs::{PublisherBuilder, Runtime, SubscriberBuilder};

pub async fn test(is_pub: bool) -> Result<()> {
    if is_pub {
        publish().await?;
    } else {
        subscribe().await?
    }
    Ok(())
}

async fn publish() -> Result<()> {
    tracing::info!("begin iceoryx publish");
    Runtime::init("publisher");

    let publisher = PublisherBuilder::<crate::Image>::new("sdk", "ros", "image").create()?;

    loop {
        let mut a = publisher.loan()?;
        a.timestamp = crate::current_ts_micros()?;

        publisher.publish(a);
    }
}

async fn subscribe() -> Result<()> {
    tracing::info!("begin iceoryx subscribe");
    Runtime::init("subscriber");

    let (subscriber, image_receive_token) =
        SubscriberBuilder::<crate::Image>::new("sdk", "ros", "image")
            .queue_capacity(5)
            .create()?;

    let image_receiver = subscriber.get_sample_receiver(image_receive_token);

    loop {
        if image_receiver.has_data() {
            if let Some(image) = image_receiver.take() {
                let ts = crate::current_ts_micros()?;

                tracing::info!("elapsed: {} data_size= {}", ts - image.timestamp, 0);
            }
        }
    }
}
