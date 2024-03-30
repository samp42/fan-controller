// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::Arc, sync::Mutex};

struct InstantProfile {
    pub fans: Vec<u8>,
    pub delta_t: u8, // ms
}

struct Pattern {
    pub run: Mutex<bool>,
    pub profile: Mutex<Vec<InstantProfile>>,
}

// type SharedState = Arc<Mutex<Pattern>>;

#[tauri::command]
fn list_serial_ports() -> Vec<String> {
    serialport::available_ports()
        .unwrap_or(vec![])
        .iter()
        .map(|port| port.clone().port_name)
        .collect()
}

#[tauri::command]
fn run_pattern(state: tauri::State<Pattern>) {
    let mut run = state.run.lock().unwrap();

    // set run to true
    *run = true;

    println!("Pattern running: {:?}", run);

    drop(run);
}

fn run_loop(state: tauri::State<Pattern>) {
    loop {
        {
            let running = state.run.lock().unwrap();
            if !*running {
                // Break out of the loop if running is set to false
                break;
            }
        }

        // Access shared state inside the loop as needed
        // let mut pattern = state.pattern.lock().unwrap();
        // Your loop logic here

        // Sleep or perform some other action
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

fn set_fan_speeds(speed: u8) {
    let mut port = serialport::new("/dev/cu.usbmodem149464201", 115200)
        .timeout(std::time::Duration::from_millis(1))
        .open()
        .expect("Failed to open port");

    let start_marker: [u8; 2] = [0x00, 0xFF];
    let end_marker: [u8; 2] = [0xFF, 0x00];
    let mut data_packet: Vec<u8> = Vec::new();

    data_packet.extend_from_slice(&start_marker);

    for i in 0..81 {
        let pwm_value = (speed as f32 / 100.0 * 4095.0) as u16;
        let high_byte = (pwm_value >> 8) as u8;
        let low_byte = (pwm_value & 0xFF) as u8;

        data_packet.push(high_byte);
        data_packet.push(low_byte);
    }

    data_packet.extend_from_slice(&end_marker);

    port.write_all(&data_packet)
        .expect("Failed to write to port");
}

fn read_timing_data() {
    let mut port = serialport::new("/dev/cu.usbmodem149464201", 115200)
        .timeout(std::time::Duration::from_millis(1))
        .open()
        .expect("Failed to open port");

    let mut buffer: Vec<u8> = vec![0; 83];

    port.read_exact(&mut buffer)
        .expect("Failed to read from port");

    println!("{:?}", buffer);

    // if ser.in_waiting >= 9:  # Expecting 9 bytes, 1 marker + 4 bytes timing + 1 marker + 4 bytes timing
    //     marker = ser.read(1)
    //     if marker == b'\xFE':
    //         receive_duration = int.from_bytes(ser.read(4), 'little')
    //         marker = ser.read(1)
    //         if marker == b'\xFF':
    //             pwm_set_duration = int.from_bytes(ser.read(4), 'little')
    //             print(f"Data Receive Duration: {receive_duration} us, PWM Set Duration: {pwm_set_duration} us")
}

fn main() {
    // let openFile = CustomMenuItem::new("open_file".to_string(), "Open File");
    // let saveFile: CustomMenuItem::new
    // let close = CustomMenuItem::new("close".to_string(), "Close Window");
    // let fileSubmenu = Submenu::new("File", Menu::new().add_item(openFile).add_item(close));

    // let viewSubmenu = Submenu::new("View");
    // let windowSubmenu = Submenu::new("Window");

    // let menu = Menu::new()
    //     .add_native_item(MenuItem::Copy)
    //     .add_item(CustomMenuItem::new("hide", "Hide"))
    //     .add_submenu(fileSubmenu);
    //     .add_submenu(viewSubmenu)
    //     .add_submenu(windowSubmenu);

    // tauri::Builder::default()
    //     .manage(Pattern {
    //       run: Mutex::new(false),
    //       profile: Mutex::new(Vec::new()),
    //     })
    //     .invoke_handler(tauri::generate_handler![list_serial_ports, run_pattern])
    //     // .menu(menu)
    //     // .on_menu_event(|event| {
    //     //     match event.menu_item_id() {
    //     //       "quit" => {
    //     //         std::process::exit(0);
    //     //       }
    //     //       "close" => {
    //     //         event.window().close().unwrap();
    //     //       }
    //     //       _ => {}
    //     //     }
    //     // })
    //     .setup(|app| {
    //         let state: tauri::State<'_, Pattern>  = app.state();
    //         print!("Setup");
    //         run_loop(state);
    //         Ok(())
    //     })
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");

    let pattern = Pattern {
        run: Mutex::new(false),
        profile: Mutex::new(Vec::new()),
    };

    let result = tauri::Builder::default()
        .manage(pattern)
        .invoke_handler(tauri::generate_handler![list_serial_ports, run_pattern])
        .build(tauri::generate_context!());

    match result {
        // Ok(app) => app.run(|_app_handle, event| {}),
        Ok(app) => {
            // Get a handle to the Tauri manager
            // let manager: tauri::State<'_, Pattern> = app.state();

            std::thread::spawn(move || {
                // "/dev/cu.usbmodem149464201"
                // "/dev/cu.BeatsStudiodeSamuel"

                let mut port = serialport::new("/dev/cu.usbmodem149464201", 115200)
                    .timeout(std::time::Duration::from_millis(1))
                    .open()
                    .expect("Failed to open port");

                let mut counter: u8 = 0; // 0 to 100

                // loop {
                //     // println!("{:?}", cloned_manager.run.lock().unwrap().to_string());
                //     println!("running loop");

                //     let start_marker: [u8; 2] = [0x00, 0xFF];
                //     let end_marker: [u8; 2] = [0xFF, 0x00];
                //     let mut data_packet: Vec<u8> = Vec::new();

                //     data_packet.extend_from_slice(&start_marker);

                //     for i in 0..81 {
                //         let pwm_value = (0 as f32 / 100.0 * 4095.0) as u16;
                //         let high_byte = (pwm_value >> 8) as u8;
                //         let low_byte = (pwm_value & 0xFF) as u8;

                //         data_packet.push(high_byte);
                //         data_packet.push(low_byte);
                //     }

                //     data_packet.extend_from_slice(&end_marker);

                //     println!("{:?}", data_packet);

                //     port.write_all(&data_packet)
                //         .expect("Failed to write to port");

                //     // let running = pattern.run.lock().unwrap();

                //     // println!("Pattern running: {:?}", running);

                //     // drop(running);

                //     // Simulate some delay between iterations
                //     std::thread::sleep(std::time::Duration::from_millis(100));

                //     println!("Counter: {:?}", counter);

                //     if (counter == 100) {
                //         counter = 0;
                //     } else {
                //         counter += 1;
                //     }
                // }

                loop {
                    //                 tep_delay = duration / 40.0
                    // for speed in range(101):
                    //     send_fan_speeds(speed)
                    //     time.sleep(step_delay)
                    //     read_timing_data()

                    // for speed in range(100, -1, -1):
                    //     send_fan_speeds(speed)
                    //     time.sleep(step_delay)
                    //     read_timing_data()

                    let step_delay = 0.125;

                    for speed in 0..101 {
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

                        port.write_all(&data_packet)
                            .expect("Failed to write to port");
                        std::thread::sleep(std::time::Duration::from_secs_f32(step_delay));
                        let mut buffer: Vec<u8> = vec![0; 83];

                        port.read_exact(&mut buffer)
                            .expect("Failed to read from port");

                        println!("{:?}", buffer);
                    }

                    for speed in (0..101).rev() {
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

                        port.write_all(&data_packet)
                            .expect("Failed to write to port");
                        std::thread::sleep(std::time::Duration::from_secs_f32(step_delay));
                        let mut buffer: Vec<u8> = vec![0; 83];

                        port.read_exact(&mut buffer)
                            .expect("Failed to read from port");

                        println!("{:?}", buffer);
                    }
                }
            });

            // Start the Tauri application
            app.run(|_app_handle, event| {});
        }
        Err(e) => println!("error while running tauri application"),
    }
    // let manager = app.unwrap().manager();

    // thread::spawn(move || {
    //     let state = manager.state();
    //     run_loop(state);
    // });

    // app.run(|_app_handle| Ok(()));
}
