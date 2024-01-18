use std::{ops::DerefMut, path::Path, time::Instant};

use anyhow::Result;

use r2r::WrappedTypesupport;
use safe_drive::{error::DynError, msg::rosidl_runtime_c__String__Sequence__create};
use safe_drive_msg;

fn generate_safe_drive_msgs() {
    let dependencies = ["std_msgs", "sensor_msgs", "std_srvs", "common_msgs"];
    safe_drive_msg::depends(
        &Path::new("/nav_env/msg"),
        &dependencies,
        safe_drive_msg::SafeDrive::Version("0.2"),
    );
}

fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    // generate_safe_drive_msgs();
    // safe_drive_test().unwrap();
    r2r_test()?;

    println!("Hello, world!");
    Ok(())
}

const IMAGE_TOPIC: &str = "/sdk/ros/image";

fn safe_drive_test() -> std::result::Result<(), DynError> {
    use safe_drive::context::Context;
    use safe_drive::msg::RosString;
    use safe_drive::msg::U8Seq;
    use sensor_msgs::msg::Image;

    let ctx = Context::new()?;
    let node = ctx.create_node("safe_drive_node", None, Default::default())?;
    let publisher = node.create_publisher::<Image>(IMAGE_TOPIC, None)?;

    for i in 0..u64::MAX {
        let stamp = builtin_interfaces::msg::Time {
            sec: 10,
            nanosec: 100,
        };
        let header = std_msgs::msg::Header {
            stamp,
            frame_id: RosString::<0>::new(i.to_string().as_str()).unwrap(),
        };

        let raw_data = vec![0; 6 * 1024 * 1024];
        let mut data = U8Seq::<0>::new(6 * 1024 * 1024).unwrap();
        data.as_slice_mut().copy_from_slice(raw_data.as_slice());

        let mut image = publisher.borrow_loaned_message()?;
        image.header = header;
        image.height = 1024;
        image.width = 768;
        image.encoding = RosString::new("dd").unwrap();
        image.is_bigendian = 1;
        image.step = 1;
        image.data = data;

        // let image = Image {
        //     header,
        //     height: 1024,
        //     width: 768,
        //     encoding: RosString::new("dd").unwrap(),
        //     is_bigendian: 1,
        //     step: 1,
        //     data,
        // };

        // publisher.send(&image)?;
        publisher.send_loaned(image)?;
    }

    Ok(())
}

fn r2r_test() -> Result<()> {
    use r2r::common_msgs::msg::FixedImage;
    use r2r::{sensor_msgs::msg::Image, QosProfile};

    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "abc", "/")?;
    // let publisher = node.create_publisher::<r2r::sensor_msgs::msg::Image>(
    //     "/sdk/ros/image",
    //     QosProfile::default(),
    // )?;

    let publisher = node.create_publisher::<r2r::common_msgs::msg::FixedImage>(
        "/sdk/ros/image",
        QosProfile::default(),
    )?;

    tracing::info!("hahah");

    for i in 0..u64::MAX {
        // let stamp = r2r::builtin_interfaces::msg::Time {
        //     sec: 10,
        //     nanosec: 100,
        // };

        // let header = r2r::std_msgs::msg::Header {
        //     stamp,
        //     frame_id: i.to_string(),
        // };

        let data_size = 6220800;

        // let mut msg = publisher.borrow_loaned_message()?;

        // // msg.header.stamp.sec = header.stamp.sec;
        // // msg.header.stamp.nanosec = header.stamp.nanosec;
        // // msg.header.frame_id.assign("1");
        // // msg.height = 1024;
        // msg.width = 768;
        // // msg.encoding.assign("dd");
        // msg.is_bigendian = 1;
        // msg.step = 1;

        // // let begin = Instant::now();
        // msg.data.copy_from_slice(vec![0; data_size].as_slice());
        // // msg.data.update(vec![0; data_size].as_slice());
        // // let elapsed = begin.elapsed().as_micros();
        // // println!("copy cost: {elapsed}");

        // publisher.publish_native(&mut msg)?;

        let image = FixedImage {
            // header,
            height: 1024,
            width: 768,
            // encoding: "dd".to_string(),
            is_bigendian: 1,
            step: 1,
            data: vec![0; data_size],
        };
        publisher.publish(&image)?;

        // std::thread::sleep(std::time::Duration::from_millis(30));
    }

    Ok(())
}
