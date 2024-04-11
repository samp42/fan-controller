// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::time;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use serde::{Deserialize, Serialize};
use tauri::Manager;

use serialport::SerialPort;
use std::time::SystemTime;

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
                match grid_value {
                    Some(ref grid) => {
                        println!("Running grid");
                        set_grid_to_speed(&cloned_port, &grid);
                    }
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

#[tauri::command]
fn get_timing(_state: tauri::State<PatternArc>, port: String) {
    println!("Getting timing data");
    let duration = 50;
    let step_delay = duration / 40;

    // let mut run = state.run.lock().unwrap();

    let mut port = serialport::new(port, 115200)
        .timeout(std::time::Duration::from_millis(1))
        .open()
        .expect("Failed to open port");

    loop {
        for speed in 0..101 {
            let start_marker: [u8; 2] = [0x00, 0xFF];
            let end_marker: [u8; 2] = [0xFF, 0x00];
            let mut data_packet: Vec<u8> = Vec::new();

            data_packet.extend_from_slice(&start_marker);

            let duration_since_epoch = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            let timestamp_nanos = duration_since_epoch.as_micros();
            // write timestamp_nanos to the data packet
            let timestamp_bytes = timestamp_nanos.to_le_bytes();
            // println!("{:?}", timestamp_nanos);
            // println!("Timestamp bytes: {:?}", timestamp_bytes);
            data_packet.extend_from_slice(&timestamp_bytes);

            for _ in 0..81 {
                let pwm_value = (speed as f32 / 100.0 * 4095.0) as u16;
                let high_byte = (pwm_value >> 8) as u8;
                let low_byte = (pwm_value & 0xFF) as u8;

                data_packet.push(high_byte);
                data_packet.push(low_byte);
            }

            data_packet.extend_from_slice(&end_marker);

            let duration_since_epoch = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            let dt = duration_since_epoch.as_micros();

            port.write_all(&data_packet)
                .expect("Should have written data");
            let mut buffer: Vec<u8> = vec![0; 16]; // Buffer for reading 9 bytes
            let bytes_read_opt = port.read(buffer.as_mut_slice());
            // println!("Wrote Buffer: {:?}", buffer);
            let duration_since_epoch = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            println!(
                "{:?}",
                (duration_since_epoch.as_micros() - dt)
            );
            match bytes_read_opt {
                Ok(bytes_read) => {
                    if bytes_read >= 16 {
                        // if buffer contains 0xFE
                        // println!("Read Buffer: {:?}", buffer);
                        // println!("bytes read: {:?}", bytes_read);

                        if buffer[0] == 0xFE {
                            let receive_duration_bytes: &[u8] = &buffer[1..16];
                            let receive_duration =
                                u32::from_le_bytes(receive_duration_bytes.try_into().unwrap());

                            // println!("Receive Duration: {}", receive_duration);
                            // println!("Sent bytes: {:?}", timestamp_bytes);
                            // println!("Receive Duration bytes: {:?}", receive_duration_bytes);

                            // if buffer[] == 0xFF {
                            //     let pwm_set_duration_bytes = &buffer[6..];
                            //     let pwm_set_duration =
                            //         u32::from_le_bytes(pwm_set_duration_bytes.try_into().unwrap());

                            //     // Process the received data
                            //     println!("Receive Duration: {}", receive_duration);
                            //     println!("PWM Set Duration: {}", pwm_set_duration);
                            // }
                        }

                        // println!("If bytes_read >= 17");
                    }
                    // println!("bytes read: {:?}", bytes_read);
                }
                Err(_) => println!("Failed to read from port"),
            }
            std::thread::sleep(Duration::from_secs(step_delay));
            // read_timing_data(port);
        }

        // for speed in (0..101).rev() {
        //     let start_marker: [u8; 2] = [0x00, 0xFF];
        //     let end_marker: [u8; 2] = [0xFF, 0x00];
        //     let mut data_packet: Vec<u8> = Vec::new();

        //     data_packet.extend_from_slice(&start_marker);

        //     for _ in 0..81 {
        //         let pwm_value = (speed as f32 / 100.0 * 4095.0) as u16;
        //         let high_byte = (pwm_value >> 8) as u8;
        //         let low_byte = (pwm_value & 0xFF) as u8;

        //         data_packet.push(high_byte);
        //         data_packet.push(low_byte);
        //     }

        //     data_packet.extend_from_slice(&end_marker);
        //     port.write_all(&data_packet)
        //         .expect("Should have written data");

        //     let mut buffer: Vec<u8> = vec![0; 9]; // Buffer for reading 9 bytes
        //     let bytes_read_opt = port.read(buffer.as_mut_slice());
        //     match bytes_read_opt {
        //         Ok(bytes_read) => {
        //             if bytes_read >= 9 {
        //                 if buffer[0] == 0xFE {
        //                     let receive_duration_bytes = &buffer[1..5];
        //                     let receive_duration =
        //                         u32::from_le_bytes(receive_duration_bytes.try_into().unwrap());

        //                     if buffer[5] == 0xFF {
        //                         let pwm_set_duration_bytes = &buffer[6..];
        //                         let pwm_set_duration =
        //                             u32::from_le_bytes(pwm_set_duration_bytes.try_into().unwrap());

        //                         // Process the received data
        //                         println!("Receive Duration: {}", receive_duration);
        //                         println!("PWM Set Duration: {}", pwm_set_duration);
        //                     }
        //                 }
        //             }
        //             println!("bytes read: {:?}", bytes_read);
        //         }
        //         Err(_) => println!("Failed to read from port"),
        //     }
        //     std::thread::sleep(Duration::from_secs(step_delay));
        // }
    }
}

fn read_timing_data(port: Box<dyn SerialPort>) {
    // let mut buffer: Vec<u8> = vec![0; 9]; // Buffer for reading 9 bytes
    // let bytes_read_opt = port.read(buffer.as_mut_slice());

    // let bytes = port.bytes_to_read();

    // match bytes {
    //     Ok(bytes) => {
    //         // let mut buffer: Vec<u8> = vec![0; bytes];
    //         // let bytes_read = port.read(buffer.as_mut_slice()).unwrap();

    //         println!("{:?}", bytes);
    //     }
    //     Err(_) => println!("Failed to read from port"),
    // }

    // match bytes_read_opt {
    //     Ok(bytes_read) => {
    //         if bytes_read >= 9 {
    //             if buffer[0] == 0xFE {
    //                 let receive_duration_bytes = &buffer[1..5];
    //                 let receive_duration = u32::from_le_bytes(receive_duration_bytes.try_into().unwrap());

    //                 if buffer[5] == 0xFF {
    //                     let pwm_set_duration_bytes = &buffer[6..];
    //                     let pwm_set_duration = u32::from_le_bytes(pwm_set_duration_bytes.try_into().unwrap());

    //                     // Process the received data
    //                     println!("Receive Duration: {}", receive_duration);
    //                     println!("PWM Set Duration: {}", pwm_set_duration);
    //                 }
    //             }
    //         }
    //         println!("bytes read: {:?}", bytes_read);
    //     }
    //     Err(_) => println!("Failed to read from port"),
    // }

    // println!("{:?}", timing_data);
}

fn main() {
    let pattern = PatternArc {
        run: Arc::new(Mutex::new(false)),
    };

    let result = tauri::Builder::default()
        .manage(pattern)
        .invoke_handler(tauri::generate_handler![
            list_serial_ports,
            run_pattern,
            stop_pattern,
            get_timing
        ])
        .build(tauri::generate_context!());

    match result {
        Ok(app) => {
            app.run(|_app_handle, _event| {});
        }
        Err(_) => println!("error while running tauri application"),
    }
}
