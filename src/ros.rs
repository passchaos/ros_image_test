use std::time::Duration;

use anyhow::Result;
use futures_util::StreamExt;
use r2r::{Node, QosProfile};

use crate::{current_ts_micros, data_size, IMAGE_TOPIC};

type Image = r2r::fixed_size_msgs::msg::Image1080p;

pub async fn test(is_pub: bool, loan: bool) -> Result<()> {
    let ctx = r2r::Context::create()?;

    let node = r2r::Node::create(ctx, "abc", "/")?;

    if is_pub {
        publish(node, loan).await?;
    } else {
        subscribe(node).await?;
    }
    Ok(())
}

async fn subscribe(mut node: Node) -> Result<()> {
    let mut subscriber = node.subscribe::<Image>(IMAGE_TOPIC, QosProfile::default())?;
    tracing::info!("begin ros2 subscribe");

    tokio::task::spawn_blocking(move || loop {
        node.spin_once(Duration::from_millis(1));
    });

    while let Some(data) = subscriber.next().await {
        let ts = current_ts_micros()?;

        let elapsed = ts - data.timestamp;
        tracing::info!("elapsed: {}", elapsed);
    }

    Ok(())
}

async fn publish(mut node: Node, loan: bool) -> Result<()> {
    let publisher = node.create_publisher::<Image>(IMAGE_TOPIC, QosProfile::default())?;

    tracing::info!("begin ros2 publish: loan= {loan}");

    loop {
        let ts = current_ts_micros()?;

        if loan {
            let mut msg = publisher.borrow_loaned_message()?;
            msg.timestamp = ts;
            msg.is_bigendian = 0;
            msg.step = 0;
            msg.data.copy_from_slice(&[0; data_size()]);

            publisher.publish_native(&mut msg)?;
        } else {
            let image = Image {
                timestamp: ts,
                is_bigendian: 0,
                step: 0,
                data: vec![0; data_size()],
            };
            publisher.publish(&image)?;
        }

        std::thread::sleep(Duration::from_millis(10));
    }
}
