use log::info;
use once_cell::sync::OnceCell;
use time::Duration;
use tokio::signal;
use tokio::time;
use umatoi::cube::id_information::IdInformation;
use umatoi::cube::{CoreCube, CoreCubeBasicFunction, NotificationData};
use umatoi::device_interface::ble::BleInterface;
use umatoi::api::simple::Simple;
use umatoi::notification_manager::NotificationManager;
use umatoi::device_interface::DeviceInterface;
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
    let mut notification_manager = NotificationManager::<NotificationData>::new();


    let cube_for_main = cube_arc.clone();
    let cube_for_task = cube_arc.clone();

    // search and connect

    {
        let mut cube = cube_for_main.lock().unwrap();
        cube.device.scan(None, None, Duration::from_secs(3))
            .await
            .unwrap();
        cube.device.connect()
            .await
            .unwrap();
        println!("** connection established");
    }


    // register notify handler

    let handler_uuid = notification_manager
        .register(Box::new(&notify_handler))
        .unwrap();
    info!("notify handler uuid {:?}", handler_uuid);

    // start to receive notifications from cube


    let mut task: Option<tokio::task::JoinHandle<NotificationManager<NotificationData>>> = None;
    {
        if let Some(peri) = &cube_for_task.lock().unwrap().device.ble_peripheral {
            let mut stream = peri.notifications().await.unwrap();
            let notify_receiver = async move {
                while let Some(data) = stream.next().await {
                    notification_manager.invoke_all_handlers(data).unwrap();
                }
                notification_manager
            };
            task = Some(tokio::spawn(notify_receiver));
        }
    }

    let timer = async {
        signal::ctrl_c().await.expect("failed to listen for event");
        println!("received ctrl-c event");
    };

    {
        let mut cube = cube_for_main.lock().unwrap();
        // run
        cube.go(15, 15, 0).await.unwrap();

        // wait until Ctrl-C is pressed

        timer.await;
        if let Some(t) = task {
            t.abort();
            // let mut nm = t.await.unwrap();
            // if nm.unregister(handler_uuid).is_err() {
            //     panic!();
            // }
        }
        println!("** disconnecting now");

        // stop
        cube.stop().await.unwrap();

        // if notification_manager.unregister(handler_uuid).is_err() {
            //  panic!();
        // }
        if cube.device.disconnect().await.is_err() {
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

    // time::sleep(Duration::from_secs(3)).await;
    println!("Bye!");
}
