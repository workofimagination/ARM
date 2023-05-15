use std::f32::consts::PI;

#[derive(Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

pub struct Calc {
    pub origin: Point,
    pub radius: f32
}

impl Calc {
    pub fn new(origin_x: f32, origin_y: f32, radius: f32) -> Calc {
        let origin = Point {
            x: origin_x,
            y: origin_y
        };

        return Calc { origin, radius }
    }
    pub fn dist(start_x: f32, start_y: f32, end_x: f32, end_y: f32) -> f32 {
        let d = f32::sqrt((start_x - end_x).powi(2) + (start_y - end_y).powi(2));

        return d;
    }

    pub fn get_angles(&self, x: f32, y: f32) -> (f32, f32) {
        let change_x = x - self.origin.x;
        let chang_y = y - self.origin.y;

        let d = Calc::dist(self.origin.x, self.origin.y, x, y);
        let a = d/2.0;
        let h = f32::sqrt(self.radius.powi(2) - a.powi(2));
        let i = f32::atan(chang_y/change_x);

        let theta_one = f32::atan(h/a) + i;

        let point_one = Calc::get_point(theta_one, &self.origin);
        let change_x2 = x-point_one.x;
        let change_y2 = y-point_one.y;
        let is_third = i32::abs(i32::signum(f32::signum(change_x2) as i32 + f32::signum(change_y2) as i32 + 2)-1);

        let theta_two = f32::atan(change_y2/change_x2) - (PI*is_third as f32);

        return (theta_one, theta_two);
    }

    pub fn get_angles_3d(&self, x: f32, y: f32, z: f32) -> (f32, f32, f32) {
        let theta = f32::atan(z/x);

        let x_prime = x*f32::cos(-theta) + z*f32::sin(-theta);

        let (theta_column, theta_beam) = self.get_angles(x_prime, y);

        return (theta, theta_column, theta_beam)
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

        return Point{ x, y }
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

        return smoothed;
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
            new[i] = Calc::normalize(*min, max, start, end, input[i]);
        }


        return Some(new);
    }
}
