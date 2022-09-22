use clap::Parser;
use log::{error, info};
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};
use time::Duration;
use pixels::{Error, Pixels, SurfaceTexture};
use tokio::signal;
use tokio::time;
use umatoi::api::simple::Simple;
use umatoi::cube::id_information::IdInformation;
use umatoi::cube::{CoreCube, CoreCubeBasicFunction, NotificationData};
use umatoi::device_interface::ble::BleInterface;
use winit::dpi::{LogicalSize, PhysicalPosition};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit::platform::run_return::EventLoopExtRunReturn;
use winit_input_helper::WinitInputHelper;



#[derive(Parser)]
#[clap(
    name = "id_checker",
    author = "YABE.Kazuhiro",
    version = "v0.0.1",
    about = "toio ID checker (GUI version)"
)]
struct AppArg {
    #[clap(short, long)]
    run: bool,
}

const MAP_START_X: u16 = 3787;
const MAP_START_Y: u16 = 778;
const MAP_SIZE_X: u16 = 190;
const MAP_SIZE_Y: u16 = 190;

static POSITION_ID_READ: OnceCell<Mutex<usize>> = OnceCell::new();
static POSITION_ID_MISSED: OnceCell<Mutex<usize>> = OnceCell::new();
static STANDARD_ID_READ: OnceCell<Mutex<usize>> = OnceCell::new();
static STANDARD_ID_MISSED: OnceCell<Mutex<usize>> = OnceCell::new();
static POSITION_ID_MAP: OnceCell<Mutex<Box<[isize; MAP_SIZE_X as usize * MAP_SIZE_Y as usize]>>> = OnceCell::new();

fn notify_handler(data: NotificationData) {
    if let Some(id_data) = IdInformation::new(&data.value) {
        match id_data {
            IdInformation::PositionId(pos_id) => {
                let mut update = POSITION_ID_READ
                    .get_or_init(|| Mutex::new(0))
                    .lock()
                    .unwrap();
                *update += 1;
                if (MAP_START_X < pos_id.x) && (pos_id.x < (MAP_START_X + MAP_SIZE_X)) &&
                   (MAP_START_Y < pos_id.y) && (pos_id.y < (MAP_START_Y + MAP_SIZE_Y)) {
                    let pos_x: usize = (pos_id.x - MAP_START_X).into();
                    let pos_y: usize = (pos_id.y - MAP_START_Y).into();
                    let mut position_map = POSITION_ID_MAP
                        .get_or_init(|| Mutex::new(Box::new([0; (MAP_SIZE_X * MAP_SIZE_Y) as usize])))
                        .lock()
                        .unwrap();
                    print!("({}, {}) ", pos_x, pos_y);
                    position_map[(pos_y * MAP_SIZE_X as usize) + pos_x] += 1;
                }
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

struct MapVisualizer {
}

impl MapVisualizer {
    fn new() -> Self {
        Self {
        }
    }

    fn draw(&mut self, frame: &mut [u8]) {
        let position_map = POSITION_ID_MAP
            .get_or_init(|| Mutex::new(Box::new([0; (MAP_SIZE_X * MAP_SIZE_Y) as usize])))
            .lock()
            .unwrap();
        for (index, pixel) in frame.chunks_exact_mut(4).enumerate() {
            //let value: u8 = std::cmp::min(position_map[index], 255) as u8;
            let value = match position_map[index] {
                0 => [32u8, 32u8, 32u8, 255u8],
                _ => [32u8, 255u8, 32u8, 255u8],
            };
            pixel.copy_from_slice(&value);
        }
    }

    fn clear(&mut self) {
        let mut position_map = POSITION_ID_MAP
            .get_or_init(|| Mutex::new(Box::new([0; (MAP_SIZE_X * MAP_SIZE_Y) as usize])))
            .lock()
            .unwrap();
        for i in 0..(MAP_SIZE_X * MAP_SIZE_Y) as usize {
            position_map[i] = 0;
        }
    }

    fn update(&mut self) {
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let arg: AppArg = AppArg::parse();

    let cube_arc = Arc::new(tokio::sync::RwLock::new(CoreCube::<BleInterface>::new()));
    let notification_cube = cube_arc.clone();
    let cube = cube_arc.clone();

    // init graphics

    let mut input = WinitInputHelper::new();
    let mut event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(MAP_SIZE_X as f64, MAP_SIZE_Y as f64);
        let shown_size = LogicalSize::new(MAP_SIZE_X as f64 * 2.0, MAP_SIZE_Y as f64 * 2.0);
        WindowBuilder::new()
            .with_title("toio ID visualizer")
            .with_inner_size(shown_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(MAP_SIZE_X as u32, MAP_SIZE_Y as u32, surface_texture)?
    };

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

    // event loop (GUI)
    let mut visualizer = MapVisualizer::new();
    event_loop.run_return(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            visualizer.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Clear Screen
            if input.key_pressed(VirtualKeyCode::C) || input.quit() {
                visualizer.clear();
                return;
            }

            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            visualizer.update();
            window.request_redraw();
        }
    });


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

    // {
    //     let position_map = POSITION_ID_MAP
    //         .get_or_init(|| Mutex::new(Box::new([0; MAP_SIZE_X * MAP_SIZE_Y])))
    //         .lock()
    //         .unwrap();
    //     for y in 0..MAP_SIZE_Y {
    //         for x in 0..MAP_SIZE_X {
    //             print!("{}, ", position_map[(y * MAP_SIZE_X) + x]);
    //         }
    //         println!();
    //     }
    // }
    println!("Bye!");

    Ok(())
}
