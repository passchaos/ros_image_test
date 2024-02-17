use std::time::Duration;

use anyhow::Result;
use iceoryx2::prelude::Subscribe;
use iceoryx2::{
    iox2::{Iox2, Iox2Event},
    payload_mut::{PayloadMut, UninitPayloadMut},
    port::publish::UninitLoan,
    service::{service_name::ServiceName, zero_copy, Service},
};

use crate::{data_size, Image};

pub async fn test(is_pub: bool) -> Result<()> {
    if is_pub {
        publish().await?;
    } else {
        subscribe().await?;
    }
    Ok(())
}

const SN: &str = "sdk/ros/image";

const CYCLE_TIME: Duration = Duration::from_micros(20);

async fn publish() -> Result<()> {
    tracing::info!("begin iceoryx2 publish");

    let service_name = ServiceName::new(SN)?;

    let service = zero_copy::Service::new(&service_name)
        .publish_subscribe()
        .open_or_create::<Image>()?;

    let publisher = service.publisher().create()?;

    while let Iox2Event::Tick = Iox2::wait(CYCLE_TIME) {
        let image = publisher.loan_uninit()?;
        let image = image.write_payload(Image {
            timestamp: crate::current_ts_micros()?,
            is_bigendian: 0,
            step: 0,
            data: [0; data_size()],
        });

        image.send()?;

        std::thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}

async fn subscribe() -> Result<()> {
    tracing::info!("begin iceoryx2 subscribe");
    let service_name = ServiceName::new(SN)?;
    let service = zero_copy::Service::new(&service_name)
        .publish_subscribe()
        .open_or_create::<Image>()?;

    let subscriber = service.subscriber().create()?;

    while let Iox2Event::Tick = Iox2::wait(CYCLE_TIME) {
        while let Some(image) = subscriber.receive()? {
            let ts = crate::current_ts_micros()?;
            tracing::info!("elapsed: {}", ts - image.timestamp);
        }
    }

    Ok(())
}
