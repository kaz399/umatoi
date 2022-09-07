use log::info;
use once_cell::sync::OnceCell;
use std::sync::mpsc;
use std::sync::Mutex;
use time::Duration;
use tokio::signal;
use tokio::time;
use umatoi::cube::id_information::IdInformation;
use umatoi::cube::{CoreCube, CoreCubeBasicFunction, NotificationData};
use umatoi::device_interface::ble::BleInterface;
use umatoi::device_interface::CoreCubeNotifyControl;
//use umatoi::cube::motor::control::MotorControl;

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
    let (tx, rx) = mpsc::channel::<CoreCubeNotifyControl>();
    let mut cube = CoreCube::<BleInterface>::new();

    // search and connect

    cube.scan(None, None, Duration::from_secs(3))
        .await
        .unwrap()
        .connect()
        .await
        .unwrap();
    println!("** connection established");

    // register notify hander

    let handler_uuid = cube
        .register_notify_handler(Box::new(&notify_handler))
        .await
        .unwrap();
    info!("notify handler uuid {:?}", handler_uuid);

    // start to receive notifications from cube

    let notify_receiver = cube.run_notify_receiver(rx);
    let timer = async {
        tx.send(CoreCubeNotifyControl::Run).unwrap();
        signal::ctrl_c().await.expect("failed to listen for event");
        println!("received ctrl-c event");
        tx.send(CoreCubeNotifyControl::Quit).unwrap();
    };

    // run

    // wait until Ctrl-C is pressed

    let _ = tokio::join!(notify_receiver, timer);
    println!("** disconnecting now");

    if cube.unregister_notify_handler(handler_uuid).await.is_err() {
        panic!();
    }
    if cube.disconnect().await.is_err() {
        panic!()
    }
    drop(cube);

    {
        let pos_read = POSITION_ID_READ
            .get_or_init(|| Mutex::new(0))
            .lock()
            .unwrap();
        let pos_missed = POSITION_ID_MISSED
            .get_or_init(|| Mutex::new(0))
            .lock()
            .unwrap();

        let std_read = STANDARD_ID_READ
            .get_or_init(|| Mutex::new(0))
            .lock()
            .unwrap();
        let std_missed = STANDARD_ID_MISSED
            .get_or_init(|| Mutex::new(0))
            .lock()
            .unwrap();

        println!("position id (read/missed) = {}/{}", *pos_read, *pos_missed);
        println!("standard id (read/missed) = {}/{}", *std_read, *std_missed);
    }

    // wait to complete the disconnection process of the cube

    time::sleep(Duration::from_secs(3)).await;
    println!("Bye!");
}
