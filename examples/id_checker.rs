use log::info;
use std::sync::mpsc;
use time::Duration;
use tokio::signal;
use tokio::time;
use umatoi::ble::BleInterface;
use umatoi::cube::connection::{search_cube, CoreCube, CoreCubeNotifyControl, NotificationData};
use umatoi::cube::id_information::IdInformation;
use uuid::Uuid;

fn notify_handler(data: NotificationData) {
    if let Some(id_data) = IdInformation::new(&data.value) {
        match id_data {
            IdInformation::PositionId(pos_id) => {
                println!("position id: {:?}", pos_id);
            }
            IdInformation::StandardId(std_id) => {
                println!("standard id: {:?}", std_id);
            }
            IdInformation::PositionIdMissed => {
                println!("position id missed");
            }
            IdInformation::StandardIdMissed => {
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
    let mut cube = CoreCube::new();

    search_cube(&mut cube, Duration::from_secs(3))
        .await
        .unwrap();

    cube.connect().await.unwrap();
    let handler_uuid = cube
        .register_notify_handler(Box::new(&notify_handler))
        .await
        .unwrap();
    info!("notify handler uuid {:?}", handler_uuid);
    println!("** connection established");

    let data: NotificationData = NotificationData {
        uuid: Uuid::new_v4(),
        value: [1, 2, 3].to_vec(),
    };
    cube.root_notify_manager.invoke_all_handlers(data).unwrap();

    //cube.receive_notify().await.unwrap();

    let notify_receiver = cube.run_notify_receiver(rx);
    let timer = async {
        tx.send(CoreCubeNotifyControl::Run).unwrap();
        signal::ctrl_c().await.expect("failed to listen for event");
        println!("received ctrl-c event");
        tx.send(CoreCubeNotifyControl::Quit).unwrap();
    };

    let _ = tokio::join!(notify_receiver, timer);
    println!("** disconnecting now");

    if cube.unregister_notify_handler(handler_uuid).await.is_err() {
        panic!();
    }
    if cube.disconnect().await.is_err() {
        panic!()
    }
    drop(cube);

    // wait to complete the disconnection process of the cube
    time::sleep(Duration::from_secs(3)).await;
    println!("Bye!");
}
