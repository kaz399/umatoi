use log::info;
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};
use time::Duration;
use tokio::signal;
use tokio::time;
use umatoi::api::simple::Simple;
use umatoi::cube::id_information::IdInformation;
use umatoi::cube::{CoreCube, CoreCubeBasicFunction, NotificationData};
use umatoi::device_interface::ble::BleInterface;
use clap::Parser;

#[derive(Parser)]
#[clap(
    name = "id_checker",
    author = "YABE.Kazuhiro",
    version = "v0.0.1",
    about = "toio ID checker",
)]
struct AppArg {
    #[clap(short ,long)]
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
    let arg: AppArg = AppArg::parse();

    let cube_arc = Arc::new(tokio::sync::RwLock::new(CoreCube::<BleInterface>::new()));
    let notification_cube = cube_arc.clone();
    let cube = cube_arc.clone();

    // search and connect

    cube.write()
        .await
        .scan(None, None, Duration::from_secs(3))
        .await
        .unwrap()
        .connect()
        .await
        .unwrap();
    println!("** connection established");

    // register notify handler

    let handler_uuid = cube
        .write()
        .await
        .register_notify_handler(Box::new(&notify_handler))
        .await
        .unwrap();
    info!("notify handler uuid {:?}", handler_uuid);

    // start to receive notifications from cube

    let notification_receiver = async move {
        notification_cube
            .read()
            .await
            .create_notification_receiver()
            .unwrap()
            .await;
    };
    let notification_task = tokio::spawn(notification_receiver);

    // run
    if arg.run {
            cube.read().await.go(15, 15, 0).await.unwrap();
    }


    // wait until Ctrl-C is pressed
    let timer = async {
        signal::ctrl_c().await.expect("failed to listen for event");
        println!("received ctrl-c event");
    };
    timer.await;
    notification_task.abort();

    println!("** disconnecting now");

    // stop
    if arg.run {
            cube.read().await.stop().await.unwrap();
    }

    if cube
        .write()
        .await
        .unregister_notify_handler(handler_uuid)
        .await
        .is_err()
    {
        panic!();
    }
    if cube.write().await.disconnect().await.is_err() {
        panic!()
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
