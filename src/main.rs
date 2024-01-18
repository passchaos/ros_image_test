use std::{path::Path, time::UNIX_EPOCH};

use anyhow::{bail, Result};
use clap::Parser;

use iceoryx_rs::marker::ShmSend;

use r2r::fixed_size_msgs::msg::Image1080p;

mod iceoryx;
mod iceoryx2;
mod ros;

#[derive(Debug)]
#[repr(C)]
pub struct Image {
    pub timestamp: i64,
    pub is_bigendian: u8,
    pub step: u32,
    pub data: [u8; data_size()],
}

impl Default for Image {
    fn default() -> Self {
        Self {
            timestamp: 0,
            is_bigendian: 0,
            step: 0,
            data: [0; data_size()],
        }
    }
}

unsafe impl ShmSend for Image {}

const fn data_size() -> usize {
    Image1080p::HEIGHT as usize * Image1080p::WIDTH as usize * Image1080p::CHANNELS as usize
}

#[derive(Parser, Debug)]
struct Args {
    transport_type: String,
    #[clap(long, short, action)]
    pub_or_sub: bool,
    #[clap(long, short, action)]
    ros_loan: bool,
}

#[allow(dead_code)]
fn generate_safe_drive_msgs() {
    let dependencies = ["std_msgs", "sensor_msgs", "std_srvs", "common_msgs"];
    safe_drive_msg::depends(
        Path::new("/nav_env/msg"),
        &dependencies,
        safe_drive_msg::SafeDrive::Version("0.2"),
    )
    .unwrap();
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let args = Args::parse();

    match args.transport_type.as_str() {
        "iceoryx-rs" => {
            iceoryx::test(args.pub_or_sub).await?;
        }
        "iceoryx2" => {
            iceoryx2::test(args.pub_or_sub).await?;
        }
        "ros2" => {
            ros::test(args.pub_or_sub, args.ros_loan).await?;
        }
        _ => {
            bail!("not supported transport type");
        }
    }

    // generate_safe_drive_msgs();
    // safe_drive_test().unwrap();
    // r2r_test().await?;

    println!("Hello, world!");
    Ok(())
}

const IMAGE_TOPIC: &str = "/sdk/ros/image";

fn current_ts_micros() -> Result<i64> {
    let ts = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_micros();
    Ok(ts as i64)
}

// fn safe_drive_test() -> std::result::Result<(), DynError> {
//     use safe_drive::context::Context;
//     use safe_drive::msg::RosString;
//     use safe_drive::msg::U8Seq;
//     use sensor_msgs::msg::Image;

//     let ctx = Context::new()?;
//     let node = ctx.create_node("safe_drive_node", None, Default::default())?;
//     let publisher = node.create_publisher::<Image>(IMAGE_TOPIC, None)?;

//     for i in 0..u64::MAX {
//         let stamp = builtin_interfaces::msg::Time {
//             sec: 10,
//             nanosec: 100,
//         };
//         let header = std_msgs::msg::Header {
//             stamp,
//             frame_id: RosString::<0>::new(i.to_string().as_str()).unwrap(),
//         };

//         let raw_data = vec![0; 6 * 1024 * 1024];
//         let mut data = U8Seq::<0>::new(6 * 1024 * 1024).unwrap();
//         data.as_slice_mut().copy_from_slice(raw_data.as_slice());

//         let mut image = publisher.borrow_loaned_message()?;
//         image.header = header;
//         image.height = 1024;
//         image.width = 768;
//         image.encoding = RosString::new("dd").unwrap();
//         image.is_bigendian = 1;
//         image.step = 1;
//         image.data = data;

//         // let image = Image {
//         //     header,
//         //     height: 1024,
//         //     width: 768,
//         //     encoding: RosString::new("dd").unwrap(),
//         //     is_bigendian: 1,
//         //     step: 1,
//         //     data,
//         // };

//         // publisher.send(&image)?;
//         publisher.send_loaned(image)?;
//     }

//     Ok(())
// }
