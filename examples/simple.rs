use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use fusion_imu::{FusionAhrs, Vector};
use plotpy::{Curve, Legend, Plot};

fn main() {
    let file = File::open("./examples/sensor_data.csv").expect("sensor_data.csv to exist");
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut fusion = FusionAhrs::new();
    let mut prev_time = 0.0;

    let mut ts = Vec::new();
    let mut gyro_x = Vec::new();
    let mut gyro_y = Vec::new();
    let mut gyro_z = Vec::new();
    let mut acc_x = Vec::new();
    let mut acc_y = Vec::new();
    let mut acc_z = Vec::new();
    let mut euler_roll = Vec::new();
    let mut euler_pitch = Vec::new();
    let mut euler_yaw = Vec::new();

    for line in lines.skip(1) {
        let line = line.unwrap();
        let mut elements = line.split(',').map(|s| s.parse::<f32>().unwrap());

        let time = elements.next().unwrap();
        let delta_time = time - prev_time;
        prev_time = time;

        let gyroscope = Vector {
            x: elements.next().unwrap(),
            y: elements.next().unwrap(),
            z: elements.next().unwrap(),
        };
        let accelerometer = Vector {
            x: elements.next().unwrap(),
            y: elements.next().unwrap(),
            z: elements.next().unwrap(),
        };
        let _magnetometer = Vector {
            x: elements.next().unwrap(),
            y: elements.next().unwrap(),
            z: elements.next().unwrap(),
        };
        fusion.update_no_magnetometer(gyroscope, accelerometer, delta_time);

        let quat = fusion.get_quaternion().to_euler();

        ts.push(time);
        gyro_x.push(gyroscope.x);
        gyro_y.push(gyroscope.y);
        gyro_z.push(gyroscope.z);
        acc_x.push(accelerometer.x);
        acc_y.push(accelerometer.y);
        acc_z.push(accelerometer.z);
        euler_roll.push(quat.roll);
        euler_pitch.push(quat.pitch);
        euler_yaw.push(quat.yaw);
    }

    let mut euler_roll_curve = Curve::new();
    euler_roll_curve
        .set_line_color("tab:red")
        .set_label("Roll")
        .draw(&ts, &euler_roll);
    let mut euler_pitch_curve = Curve::new();
    euler_pitch_curve
        .set_line_color("tab:green")
        .set_label("Pitch")
        .draw(&ts, &euler_pitch);
    let mut euler_yaw_curve = Curve::new();
    euler_yaw_curve
        .set_line_color("tab:blue")
        .set_label("Yaw")
        .draw(&ts, &euler_yaw);
    let mut gyro_x_curve = Curve::new();
    gyro_x_curve
        .set_line_color("tab:red")
        .set_label("Gyro X")
        .draw(&ts, &gyro_x);
    let mut gyro_y_curve = Curve::new();
    gyro_y_curve
        .set_line_color("tab:green")
        .set_label("Gyro Y")
        .draw(&ts, &gyro_y);
    let mut gyro_z_curve = Curve::new();
    gyro_z_curve
        .set_line_color("tab:blue")
        .set_label("Gyro Z")
        .draw(&ts, &gyro_z);
    let mut acc_x_curve = Curve::new();
    acc_x_curve
        .set_line_color("tab:red")
        .set_label("Acc X")
        .draw(&ts, &acc_x);
    let mut acc_y_curve = Curve::new();
    acc_y_curve
        .set_line_color("tab:green")
        .set_label("Acc Y")
        .draw(&ts, &acc_y);
    let mut acc_z_curve = Curve::new();
    acc_z_curve
        .set_line_color("tab:blue")
        .set_label("Acc Z")
        .draw(&ts, &acc_z);

    let mut legend = Legend::new();
    legend.set_location("right");
    legend.draw();

    let mut plot = Plot::new();
    plot.set_gridspec("grid", 3, 1, "wspace=0, hspace=0.7");
    plot.set_subplot_grid("grid", "0", "0")
        // plot.set_subplot(3, 1, 1)
        .add(&gyro_x_curve)
        .add(&gyro_y_curve)
        .add(&gyro_z_curve)
        .set_title("Gyroscope")
        .set_label_y("Degrees/s")
        .add(&legend);

    plot.set_subplot_grid("grid", "1", "0")
        // .set_subplot(3, 1, 2)
        .add(&acc_x_curve)
        .add(&acc_y_curve)
        .add(&acc_z_curve)
        .set_title("Accelerometer")
        .set_label_y("g")
        .add(&legend);

    plot.set_subplot_grid("grid", "2", "0")
        .add(&euler_roll_curve)
        .add(&euler_pitch_curve)
        .add(&euler_yaw_curve)
        .set_title("Euler angles")
        .set_label_x("Seconds")
        .set_label_y("Degrees")
        .add(&legend);

    let path = Path::new("/tmp/fusion/examples_plots").join("plot.svg");
    plot.save_and_show(&path).unwrap();
}
