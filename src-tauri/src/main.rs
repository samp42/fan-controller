// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::Arc, sync::Mutex};

use tauri::Manager;

struct InstantProfile {
    pub fans: Vec<u8>,
    pub delta_t: u8, // ms
}

#[derive(Clone)]
struct PatternArc {
    pub run: Arc<Mutex<bool>>,
    pub profile: Arc<Mutex<Vec<InstantProfile>>>,
}

struct Pattern {
    pub run: bool,
    pub profile: Vec<InstantProfile>,
}

fn set_fans_to_zero(port: String) {
    let start_marker: [u8; 2] = [0x00, 0xFF];
    let end_marker: [u8; 2] = [0xFF, 0x00];
    let mut data_packet: Vec<u8> = Vec::new();

    data_packet.extend_from_slice(&start_marker);

    for i in 0..81 {
        data_packet.push(0 as u8);
        data_packet.push(0 as u8);
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
fn run_pattern(app: tauri::AppHandle, port: String) {
    let cloned_app = app.clone();

    println!("Running pattern on port: {:?}", port);

    std::thread::spawn(move || {
        let state = cloned_app.state::<PatternArc>();

        let mut run = state.run.lock().unwrap();

        // set run to true
        *run = true;

        drop(run);

        loop {
            let state = cloned_app.state::<PatternArc>();
            let running = state.run.lock().unwrap();

            if *running {
                println!("Running: {:?}", *running);
                drop(running);
            } else {
                drop(running);
                break;
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}

#[tauri::command]
fn stop_pattern(state: tauri::State<PatternArc>, port: String) {
    let mut run = state.run.lock().unwrap();

    println!("Stopping pattern on port: {:?}", port);

    if *run {
        // set run to false
        *run = false;

        println!("Pattern running: {:?}", run);

        set_fans_to_zero(port);
    }

    drop(run);
}

fn main() {
    let pattern = PatternArc {
        run: Arc::new(Mutex::new(false)),
        profile: Arc::new(Mutex::new(Vec::new())),
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
