use log::info;
use once_cell::sync::OnceCell;
use std::sync::mpsc;
use time::Duration;
use tokio::signal;
use tokio::time;
use umatoi::cube::id_information::IdInformation;
use umatoi::cube::{CoreCube, CoreCubeBasicFunction, NotificationData};
use umatoi::device_interface::ble::BleInterface;
use umatoi::device_interface::CoreCubeNotifyControl;
use umatoi::api::simple::Simple;
use btleplug::platform::Peripheral;
use btleplug::api::{Peripheral as _};
use std::sync::{Arc, Mutex};
use futures::stream::StreamExt;

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
    let cube_arc = Arc::new(Mutex::new(CoreCube::<BleInterface>::new()));
    let cube = cube_arc.clone();
    let notification = cube_arc.clone();


    // search and connect

    let handler_uuid;
    let peri: Peripheral;
    {
        let mut locked_cube = cube.lock().unwrap();
        locked_cube.scan(None, None, Duration::from_secs(3))
            .await
            .unwrap()
            .connect()
            .await
            .unwrap();
        println!("** connection established");

        peri = locked_cube.device.ble_peripheral.clone().unwrap();

        // register notify hander

        handler_uuid = locked_cube
            .register_notification_handler(Box::new(&notify_handler))
            .unwrap();
        info!("notify handler uuid {:?}", handler_uuid);
    }

    // start to receive notifications from cube


    let task;
    {
        let mut notification_stream  = peri.notifications().await.unwrap();
        let notify_receiver = async move {
            while let Some(data) = notification_stream.next().await {
                let nm = notification.lock().unwrap();
                nm.notification_manager.invoke_all_handlers(data).unwrap();
            }
        };
        task = Some(tokio::spawn(notify_receiver));
    }

    let timer = async {
        signal::ctrl_c().await.expect("failed to listen for event");
        println!("received ctrl-c event");
    };

    {
        let mut locked_cube = cube.lock().unwrap();
        // run
        locked_cube.go(15, 15, 0).await.unwrap();

        // wait until Ctrl-C is pressed

        // let _ = tokio::join!(notify_receiver, timer);
        timer.await;
        task.unwrap().abort();
        println!("** disconnecting now");

        // stop
        locked_cube.stop().await.unwrap();

        if locked_cube.unregister_notification_handler(handler_uuid).is_err() {
            panic!();
        }
        if locked_cube.disconnect().await.is_err() {
            panic!()
        }
    }

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
