
pub struct Point {
    pub x: f64,
    pub y: f64
}

pub struct Calc {
    pub origin: Point,
    pub radius: f64
}

impl Calc {
    pub fn dist(start: &Point, e: &Point) -> f64 {
        let d = f64::sqrt((start.x - e.x).powi(2) + (start.y - e.y).powi(2));

        return d;
    }

    pub fn get_angles(&self, e: &Point) -> (f64, f64) {
        let change_x = e.x - self.origin.x;
        let chang_y = e.y - self.origin.y;

        let d = Calc::dist(&self.origin, e);
        let a = d/2.0;
        let h = f64::sqrt(self.radius.powi(2) - a.powi(2));
        let i = f64::atan(chang_y/change_x);

        let theta_one = f64::atan(h/a) + i;
        
        let point_one = Calc::get_point(theta_one, &self.origin);
        let change_x2 = e.x - point_one.x;
        let change_y2 = e.y - point_one.y;

        let theta_two = f64::atan(change_y2/change_x2);

        return (theta_one, theta_two);
    }

    pub fn get_point(angle: f64, center: &Point) -> Point {
        let x = f64::cos(angle) + center.x;
        let y = f64::sin(angle) + center.y;

        return Point{ x, y }
    }

    pub fn test() {
        let origin = Point {
            x: 0.0,
            y: 0.0
        };

        let goto = Point {
            x: 1.0,
            y: 0.8
        };

        let calc = Calc {
            origin,
            radius: 1.0
        };

        let origin_clone = Point {
            x: 0.0,
            y: 0.0
        };

        let (theta_one, theta_two) = calc.get_angles(&goto);

        let point_o = Calc::get_point(theta_one, &origin_clone);
        let point_t = Calc::get_point(theta_two, &point_o);

        println!("Expected point: {}, {}", goto.x, goto.y);
        println!("Beam: {}, {}", point_o.x, point_o.y);
        println!("Beam tip: {}, {}", point_t.x, point_t.y);
    }
}
