use clap::Parser;
use once_cell::sync::OnceCell;
use std::sync::Mutex;
use time::Duration;
use tokio::time;
use umatoi::api::Simple;
use umatoi::characteristic::id_information::IdInformation;
use umatoi::characteristic::motor;
use umatoi::characteristic::NotificationData;
use umatoi::interface::ble::BleScanner;
use umatoi::interface::CubeScanner;
use umatoi::position::{CubeLocation, Point};

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

fn notify_handler1(data: NotificationData) {
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
            _ => (),
        }
    } else {
        println!(
            "notify handler1: uuid: {:?} value: {:?}",
            data.uuid, data.value
        );
    }
}

fn notify_handler2(data: NotificationData) {
    if let Some(id_data) = IdInformation::new(&data.value) {
        match id_data {
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
    }
}

fn notify_handler3(data: NotificationData) {
    if let Some(motor_response) = motor::response::MotorResponse::new(&data.value) {
        match motor_response {
            motor::response::MotorResponse::MotorControlTarget(res) => {
                println!("ResponseMotorControlTarget: {:?}", res.response_code);
            }
            motor::response::MotorResponse::MotorControlMultipleTargets(res) => {
                println!(
                    "ResponseMotorControlMultipleTargets: {:?}",
                    res.response_code
                );
            }
            _ => (),
        }
    }
}

#[tokio::main]
pub async fn main() {
    let _arg: AppArg = AppArg::parse();
    let scanner = BleScanner;
    let mut cubes = scanner.scan(1, Duration::from_secs(5)).await.unwrap();

    assert!(!cubes.is_empty());

    let cube = &mut cubes[0];
    cube.connect().await.unwrap();

    let notification_receiver = cube.create_notification_receiver(Box::new(vec![
        Box::new(notify_handler1),
        Box::new(notify_handler2),
        Box::new(notify_handler3),
    ]));
    let notification_task = tokio::spawn(notification_receiver);

    // cube.motor_control(50, 50, 2000).await.unwrap();

    let target: motor::def::TargetPosition = motor::def::TargetPosition {
        cube_location: CubeLocation {
            point: Point { x: 360, y: 170 },
            angle: 180,
        },
        ..motor::def::TargetPosition::default()
    };
    cube.motor_control_target(30, target).await.unwrap();

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    cube.motor_stop().await.unwrap();

    println!("disconnect");

    notification_task.abort();
    cube.disconnect().await.unwrap();
}
