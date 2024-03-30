import serial
import time

serial_port = '/dev/cu.usbmodem149464201'
baud_rate = 115200
num_fans = 81

ser = serial.Serial(serial_port, baud_rate, timeout=1)

def send_fan_speeds(speed):
    start_marker = [0x00, 0xFF]  # Start transmission marker
    end_marker = [0xFF, 0x00]  # End transmission marker
    pwmValue = int((speed / 100.0) * 4095)
    highByte = pwmValue >> 8
    lowByte = pwmValue & 0xFF
    speeds = [highByte, lowByte] * num_fans
    data_packet = start_marker + speeds + end_marker  # Concatenate lists
    ser.write(bytearray(data_packet))
    print(f"Sent Speed: {speed}")

def ramp_fan_speeds(duration):
    step_delay = duration / 40.0
    for speed in range(101):
        send_fan_speeds(speed)
        time.sleep(step_delay)
        read_timing_data()

    for speed in range(100, -1, -1):
        send_fan_speeds(speed)
        time.sleep(step_delay)
        read_timing_data()

def read_timing_data():
    if ser.in_waiting >= 9:  # Expecting 9 bytes, 1 marker + 4 bytes timing + 1 marker + 4 bytes timing
        marker = ser.read(1)
        if marker == b'\xFE':
            receive_duration = int.from_bytes(ser.read(4), 'little')
            marker = ser.read(1)
            if marker == b'\xFF':
                pwm_set_duration = int.from_bytes(ser.read(4), 'little')
                print(f"Data Receive Duration: {receive_duration} us, PWM Set Duration: {pwm_set_duration} us")

try:
    while True:
        print("Starting ramp...")
        ramp_fan_speeds(5)  # 5 seconds for ramp up and down

finally:
    ser.close()