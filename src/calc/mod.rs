use std::f32::consts::PI;
use crate::utils::{ Point, AngleSet };
use std::io::prelude::*;

pub struct Calc {
    pub origin: Point,
    pub radius: f32
}

impl Calc {
    pub fn new(origin_x: f32, origin_y: f32, radius: f32) -> Calc {
        let origin = Point {
            x: origin_x,
            y: origin_y,
            z: 0.0
        };

        return Calc { origin, radius }
    }

    pub fn dist_3d(start: &Point, end: &Point) -> f32 {
        let d = f32::sqrt((end.x - start.x).powi(2) + (end.y - start.y).powi(2) + (end.z - start.z).powi(2));

        return d
    }

    pub fn dist(start_x: f32, start_y: f32, end_x: f32, end_y: f32) -> f32 {
        let d = f32::sqrt((start_x - end_x).powi(2) + (start_y - end_y).powi(2));

        return d;
    }

    pub fn get_angles(&self, x: f32, y: f32) -> AngleSet {
        let change_x = x - self.origin.x;
        let chang_y = y - self.origin.y;

        let d = Calc::dist(self.origin.x, self.origin.y, x, y);
        let a = d/2.0;
        let h = f32::sqrt(self.radius.powi(2) - a.powi(2));
        let i = f32::atan(chang_y/change_x);

        let column_angle = f32::atan(h/a) + i;

        let point_one = Calc::get_point_2d(column_angle, &self.origin);
        let change_x2 = x-point_one.x;
        let change_y2 = y-point_one.y;
        let is_third = i32::abs(i32::signum(f32::signum(change_x2) as i32 + f32::signum(change_y2) as i32 + 2)-1);

        let beam_angle = f32::atan(change_y2/change_x2) - (PI*is_third as f32);

        return AngleSet { column_angle, beam_angle, base_angle: 0.0 };
    }

    pub fn get_angles_3d(&self, x: f32, y: f32, z: f32) -> AngleSet {
        let theta = f32::atan(z/x);

        let x_prime = x*f32::cos(-theta) + z*f32::sin(-theta);

        let x_y_angles = self.get_angles(x_prime, y);

        return AngleSet {
            column_angle: x_y_angles.column_angle,
            beam_angle: x_y_angles.beam_angle,
            base_angle: theta
        }
    }

    pub fn to_degree(angle: f32) -> f32 {
        return (180.0*angle) / PI;
    }

    pub fn to_radian(angle: f32) -> f32 {
        return (PI*angle) / (180.0);
    }

    pub fn get_point_2d(angle: f32, center: &Point) -> Point {
        let x = f32::cos(angle) + center.x;
        let y = f32::sin(angle) + center.y;

        return Point{ x, y, z: 0.0 }
     }

    pub fn snap(angle: f32, precision: f32) -> f32{
        return f32::round(angle / precision) * precision;
    }

    pub fn smooth(points: Vec<i64>) -> Vec<i64> {
        let amp = 1;
        let min = points[0] as f64;
        let max = (points[points.len() - 1] + 1) as f64;

        let mut smoothed: Vec<i64> = Vec::new();

        for i in points {
            //let k = f64::max(0.0, f64::min(1.0, (i as f64-min) / (max-min)));
            //let t = (((6.0*k) - (6.0*k*k)).powi(8) * 1000.0) as i64;

            //STEEPNESS MUST BE DIVISIBLE BY 2
            let steepness = 8.0;
            let scale = 1.0/max.powf(steepness-1.0);
            let mid_point = max/2.0; 

            let t = ((scale * (i as f64-mid_point).powf(steepness)) + max) * 1000.0;

            smoothed.push(t as i64);
        }

        return smoothed
    }

    pub fn normalize(min: i64, max: i64, start: i64, end: i64, input: i64) -> i64 {
        //this is the worst thing I have ever written this language fucking sucks
        return ((end as f64-start as f64)*((input as f64-min as f64)/(max as f64-min as f64))+start as f64) as i64
    }

    pub fn normalize_vec(start: i64, end: i64, input: Vec<i64>) -> Option<Vec<i64>> {
        if input.len() == 0 { return None }
        let mut new = input.clone();

        let min = input.iter().min().unwrap();
        let max = input.iter().max().unwrap(); 

        for i in 0..input.len() {
            //gay ass dereference here
            new[i] = Calc::normalize(*min, *max, start, end, input[i]); //TEMP CHANGED THIS TO PUT END AS START AND START AS END, FIX THE LATER
        }

        return Some(new);
    }

    pub fn test_temp() {
        let steps = 1000;
        let mut counter = 1;
        let mut times: Vec<i64> = Vec::new();

        (0..i32::abs(steps)).for_each(|_| { times.push(counter); counter += 1});

        let smoothed = Calc::smooth(times);
        let mut file = std::fs::File::create("temp").unwrap();

        let start = 1500;
        let end = 4000;
            
        let normalized = Calc::normalize_vec(start, end, smoothed.clone()).unwrap();

        for item in normalized {
            file.write_all(format!("{} ", item).as_bytes()).unwrap();
        }
    }
}
