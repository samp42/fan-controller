// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Debug, Deserialize)]
struct InstantProfile {
    pub fans: Vec<u8>,
    pub dt: u8, // ms
}

struct PatternArc {
    pub run: Arc<Mutex<bool>>,
}

fn set_fans_to_speed(port: String, speed: u8) {
    let start_marker: [u8; 2] = [0x00, 0xFF];
    let end_marker: [u8; 2] = [0xFF, 0x00];
    let mut data_packet: Vec<u8> = Vec::new();

    data_packet.extend_from_slice(&start_marker);

    for _ in 0..81 {
        let pwm_value = (speed as f32 / 100.0 * 4095.0) as u16;
        let high_byte = (pwm_value >> 8) as u8;
        let low_byte = (pwm_value & 0xFF) as u8;

        data_packet.push(high_byte);
        data_packet.push(low_byte);
    }

    data_packet.extend_from_slice(&end_marker);

    let mut port = serialport::new(port, 115200)
        .timeout(std::time::Duration::from_millis(1))
        .open()
        .expect("Failed to open port");

    port.write_all(&data_packet)
        .expect("Failed to write to port");
}

fn set_grid_to_speed(port: &str, profile: &Vec<u8>) {
    let start_marker: [u8; 2] = [0x00, 0xFF];
    let end_marker: [u8; 2] = [0xFF, 0x00];
    let mut data_packet: Vec<u8> = Vec::new();

    data_packet.extend_from_slice(&start_marker);

    for i in 0..81 {
        let pwm_value = (*profile.get(i).unwrap() as f32 / 100.0 * 4095.0) as u16;
        let high_byte = (pwm_value >> 8) as u8;
        let low_byte = (pwm_value & 0xFF) as u8;

        data_packet.push(high_byte);
        data_packet.push(low_byte);
    }

    data_packet.extend_from_slice(&end_marker);

    let mut port = serialport::new(port, 115200)
        .timeout(std::time::Duration::from_millis(1))
        .open()
        .expect("Failed to open port");

    port.write_all(&data_packet)
        .expect("Failed to write to port");

    println!("{:?}", data_packet);
}

#[tauri::command]
fn list_serial_ports() -> Vec<String> {
    serialport::available_ports()
        .unwrap_or(vec![])
        .iter()
        .map(|port| port.clone().port_name)
        .collect()
}

#[tauri::command]
fn run_pattern(
    app: tauri::AppHandle,
    port: String,
    grid_value: Option<Vec<u8>>,
    profiles: Option<Vec<InstantProfile>>,
) {
    let cloned_app = app.clone();

    println!("{:?}", grid_value);
    println!("{:?}", profiles);

    std::thread::spawn(move || {
        let state = cloned_app.state::<PatternArc>();

        let mut run = state.run.lock().unwrap();

        // set run to true
        *run = true;

        drop(run);

        let cloned_port = port.clone();
        let mut duration = Duration::from_secs(1);

        loop {
            let state = cloned_app.state::<PatternArc>();
            let mut running = state.run.lock().unwrap();

            if *running {
                // TODO: run logic

                match grid_value {
                    Some(ref grid) => {
                        println!("Running grid");
                        set_grid_to_speed(&cloned_port, &grid);
                    },
                    None => match profiles {
                        Some(ref profiles) => {
                            println!("Running profile");
                            duration = Duration::from_millis(1);

                            for profile in profiles {
                                set_grid_to_speed(&cloned_port, &profile.fans);
                                std::thread::sleep(Duration::from_millis(profile.dt as u64));
                            }
                        }
                        None => {
                            println!("No grid or profile provided, stopping pattern");
                            *running = false;
                        }
                    },
                }

                drop(running);
            } else {
                drop(running);
                break;
            }

            std::thread::sleep(duration);
        }
    });
}

#[tauri::command]
fn stop_pattern(state: tauri::State<PatternArc>, port: String) {
    let mut run = state.run.lock().unwrap();

    if *run {
        // set run to false
        *run = false;

        set_fans_to_speed(port, 0);
    }

    drop(run);
}

fn main() {
    let pattern = PatternArc {
        run: Arc::new(Mutex::new(false)),
        // profile: Arc::new(Mutex::new(InstantProfile {
        //     fans: vec![0; 81],
        //     delta_t: 0,
        // })),
    };

    let result = tauri::Builder::default()
        .manage(pattern)
        .invoke_handler(tauri::generate_handler![
            list_serial_ports,
            run_pattern,
            stop_pattern
        ])
        .build(tauri::generate_context!());

    match result {
        Ok(app) => {
            // Get a handle to the Tauri manager
            // let manager: tauri::State<'_, Pattern> = app.state();

            //         std::thread::spawn(move || {
            //             // get port
            //             // let port = manager.port.lock().unwrap();

            //             let mut port = serialport::new(manager.port.lock().unwrap(), 115200)
            //                 .timeout(std::time::Duration::from_millis(1))
            //                 .open()
            //                 .expect("Failed to open port");

            //             let mut counter: u8 = 0; // 0 to 100

            //             loop {
            //                 // println!("{:?}", cloned_manager.run.lock().unwrap().to_string());
            //                 println!("running loop");

            //                 let start_marker: [u8; 2] = [0x00, 0xFF];
            //                 let end_marker: [u8; 2] = [0xFF, 0x00];
            //                 let mut data_packet: Vec<u8> = Vec::new();

            //                 data_packet.extend_from_slice(&start_marker);

            //                 for i in 0..81 {
            //                     let pwm_value = (0 as f32 / 100.0 * 4095.0) as u16;
            //                     let high_byte = (pwm_value >> 8) as u8;
            //                     let low_byte = (pwm_value & 0xFF) as u8;

            //                     data_packet.push(high_byte);
            //                     data_packet.push(low_byte);
            //                 }

            //                 data_packet.extend_from_slice(&end_marker);

            //                 println!("{:?}", data_packet);

            //                 port.write_all(&data_packet)
            //                     .expect("Failed to write to port");

            //                 // let running = pattern.run.lock().unwrap();

            //                 // println!("Pattern running: {:?}", running);

            //                 // drop(running);

            //                 // Simulate some delay between iterations
            //                 std::thread::sleep(std::time::Duration::from_millis(100));

            //                 println!("Counter: {:?}", counter);

            //                 if (counter == 100) {
            //                     counter = 0;
            //                 } else {
            //                     counter += 1;
            //                 }
            //             }

            //             // loop {
            //             //     //                 tep_delay = duration / 40.0
            //             //     // for speed in range(101):
            //             //     //     send_fan_speeds(speed)
            //             //     //     time.sleep(step_delay)
            //             //     //     read_timing_data()

            //             //     // for speed in range(100, -1, -1):
            //             //     //     send_fan_speeds(speed)
            //             //     //     time.sleep(step_delay)
            //             //     //     read_timing_data()

            //             //     let step_delay = 0.125;

            //             //     for speed in 0..101 {
            //             //         let start_marker: [u8; 2] = [0x00, 0xFF];
            //             //         let end_marker: [u8; 2] = [0xFF, 0x00];
            //             //         let mut data_packet: Vec<u8> = Vec::new();

            //             //         data_packet.extend_from_slice(&start_marker);

            //             //         for _ in 0..81 {
            //             //             let pwm_value = (speed as f32 / 100.0 * 4095.0) as u16;
            //             //             let high_byte = (pwm_value >> 8) as u8;
            //             //             let low_byte = (pwm_value & 0xFF) as u8;

            //             //             data_packet.push(high_byte);
            //             //             data_packet.push(low_byte);
            //             //         }

            //             //         data_packet.extend_from_slice(&end_marker);

            //             //         port.write_all(&data_packet)
            //             //             .expect("Failed to write to port");
            //             //         std::thread::sleep(std::time::Duration::from_secs_f32(step_delay));
            //             //         let mut buffer: Vec<u8> = vec![0; 83];

            //             //         port.read_exact(&mut buffer)
            //             //             .expect("Failed to read from port");

            //             //         println!("{:?}", buffer);
            //             //     }

            //             //     for speed in (0..101).rev() {
            //             //         let start_marker: [u8; 2] = [0x00, 0xFF];
            //             //         let end_marker: [u8; 2] = [0xFF, 0x00];
            //             //         let mut data_packet: Vec<u8> = Vec::new();

            //             //         data_packet.extend_from_slice(&start_marker);

            //             //         for _ in 0..81 {
            //             //             let pwm_value = (speed as f32 / 100.0 * 4095.0) as u16;
            //             //             let high_byte = (pwm_value >> 8) as u8;
            //             //             let low_byte = (pwm_value & 0xFF) as u8;

            //             //             data_packet.push(high_byte);
            //             //             data_packet.push(low_byte);
            //             //         }

            //             //         data_packet.extend_from_slice(&end_marker);

            //             //         port.write_all(&data_packet)
            //             //             .expect("Failed to write to port");
            //             //         std::thread::sleep(std::time::Duration::from_secs_f32(step_delay));
            //             //         let mut buffer: Vec<u8> = vec![0; 83];

            //             //         port.read_exact(&mut buffer)
            //             //             .expect("Failed to read from port");

            //             //         println!("{:?}", buffer);
            //             //     }
            //             // }
            //         });

            app.run(|_app_handle, _event| {});
        }
        Err(_) => println!("error while running tauri application"),
    }
}
