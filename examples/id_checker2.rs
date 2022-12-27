use clap::Parser;
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};
use time::Duration;
use tokio::{spawn, time};
use umatoi::cube::id_information::IdInformation;
use umatoi::cube::NotificationData;
use umatoi::device_interface::ble::{BleCube, BleScanner, nm_task};
use umatoi::device_interface::CubeInterface;
use umatoi::notification_manager::NotificationManager;



#[derive(Parser)]
#[clap(
    name = "id_checker",
    author = "YABE.Kazuhiro",
    version = "v0.0.1",
    about = "toio ID checker"
)]
struct AppArg {
    #[clap(short, long)]
    run: bool,
}

static POSITION_ID_READ: OnceCell<Mutex<usize>> = OnceCell::new();
static POSITION_ID_MISSED: OnceCell<Mutex<usize>> = OnceCell::new();
static STANDARD_ID_READ: OnceCell<Mutex<usize>> = OnceCell::new();
static STANDARD_ID_MISSED: OnceCell<Mutex<usize>> = OnceCell::new();

fn notify_handler(data: NotificationData) {
    if let Some(id_data) = IdInformation::new(&data.value) {
        match id_data {
            IdInformation::PositionId(pos_id) => {
                let mut update = POSITION_ID_READ
                    .get_or_init(|| Mutex::new(0))
                    .lock()
                    .unwrap();
                *update += 1;
                println!("position id: {:?}", pos_id);
            }
            IdInformation::StandardId(std_id) => {
                let mut update = STANDARD_ID_READ
                    .get_or_init(|| Mutex::new(0))
                    .lock()
                    .unwrap();
                *update += 1;
                println!("standard id: {:?}", std_id);
            }
            IdInformation::PositionIdMissed => {
                let mut update = POSITION_ID_MISSED
                    .get_or_init(|| Mutex::new(0))
                    .lock()
                    .unwrap();
                *update += 1;
                println!("position id missed");
            }
            IdInformation::StandardIdMissed => {
                let mut update = STANDARD_ID_MISSED
                    .get_or_init(|| Mutex::new(0))
                    .lock()
                    .unwrap();
                *update += 1;
                println!("standard id missed");
            }
            _ => (),
        }
    } else {
        println!(
            "notify handler1: uuid: {:?} value: {:?}",
            data.uuid, data.value
        );
    }
}

#[tokio::main]
pub async fn main() {
    let _arg: AppArg = AppArg::parse();
    let scanner = BleScanner;
    let found_interfaces = scanner.scan(1, Duration::from_secs(5)).await.unwrap();
    let nf_manager = NotificationManager::<NotificationData>::new();

    assert!(!found_interfaces.is_empty());

    let mut cube =  BleCube::new(found_interfaces[0].clone(), &nf_manager);
    cube.connect().await.unwrap();

    let handler_uuid = cube.register_notification_handler(Box::new(notify_handler)).await.unwrap();

    let notification_task = tokio::spawn(async move {
        let _ = nm_task(found_interfaces[0].clone(), &nf_manager).await;
    });
    // let notification_task = tokio::spawn(async move {
    //     cube.notification_receiver().await;
    // });

    let result = cube.unregister_notification_handler(handler_uuid).await.unwrap();
    assert!(result);

    notification_task.abort();

    cube.disconnect().await.unwrap();
}
