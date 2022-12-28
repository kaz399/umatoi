use clap::Parser;
use once_cell::sync::OnceCell;
use std::sync:: Mutex;
use time::Duration;
use tokio::time;
use umatoi::cube::id_information::IdInformation;
use umatoi::cube::NotificationData;
use umatoi::device_interface::ble::{BleCube, BleScanner, ble_notification_receiver};
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

    let mut cube =  BleCube::new(found_interfaces[0].clone());
    cube.connect().await.unwrap();

    let handler_uuid = nf_manager.register(Box::new(notify_handler)).unwrap();

    let notification_task = tokio::spawn(async move {
        println!("start notification task");
        let _ = ble_notification_receiver(found_interfaces[0].clone(), &nf_manager).await;
        println!("end notification task");
        let result = nf_manager.unregister(handler_uuid).unwrap();
        assert!(result);
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    println!("disconnect");

    notification_task.abort();

    cube.disconnect().await.unwrap();
}
