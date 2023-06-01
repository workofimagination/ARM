use std::f32::consts::PI;
use crate::utils::{ Point, AngleSet };

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

        let point_one = Calc::get_point(column_angle, &self.origin);
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

    pub fn get_point(angle: f32, center: &Point) -> Point {
        let x = f32::cos(angle) + center.x;
        let y = f32::sin(angle) + center.y;

        return Point{ x, y, z: 0.0 }
     }

    pub fn snap(angle: f32, precision: f32) -> f32{
        return f32::round(angle / precision) * precision;
    }

    pub fn smooth(points: Vec<i64>) -> Vec<i64> {
        let amp = 1;
        let mut smoothed: Vec<i64> = Vec::new();

        for i in &points {
            let t = i64::min(i64::max((i-points[0])/(points[points.len()-1]), 0), 1);

            let new_point = amp*(-(-(t*t) + t));
            smoothed.push(new_point);
        }

        return smoothed
    }

    pub fn normalize(min: i64, max: i64, start: i64, end: i64, input: i64) -> i64 {
        return (end-start)*((input-min)/(max-min))+start
    }

    pub fn normalize_vec(start: i64, end: i64, input: Vec<i64>) -> Option<Vec<i64>> {
        if input.len() == 0 { return None }
        let mut new = input.clone();
        let min = input.iter().min().unwrap();
        let max = input.iter().max().unwrap() + 1;

        for i in 0..input.len() {
            //gay ass dereference here
            new[i] = Calc::normalize(*min, max, end, start, input[i]); //TEMP CHANGED THIS TO PUT END AS START AND START AS END, FIX THE LATER
        }


        return Some(new);
    }
}
