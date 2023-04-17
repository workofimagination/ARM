
pub struct Point {
    x: f64,
    y: f64
}

pub struct Calc {
    origin: Point,
    radius: f64
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
        let theta_two = f64::asin((d-a)/self.radius);

        return (theta_one, theta_two);
    }

    pub fn temp_dist(angle: f64, goto: &Point) -> f64 {
        let x = f64::cos(angle);
        let y = f64::sin(angle);

        let point = Point {
            x,
            y
        };

        let distance = Calc::dist(&point, &goto);

        return distance
    }

    pub fn to_point(&self, angle: f64) -> Point{
        let x = f64::cos(angle);
        let y = f64::sin(angle);

        return Point { x, y }
    }
}

fn main() {
    let origin = Point {
        x: 0.0,
        y: 0.0
    };

    let calc = Calc {
        origin,
        radius: 1.0
    };

    let goto = Point {
        x: 1.0,
        y: 0.4
    };

    let n_origin = Point {
        x: 0.0,
        y: 0.0
    };

    let (theta_one, theta_two) = calc.get_angles(&goto);

    println!("Column Degree: {}", theta_one);
    println!("Beam Degree: {}", theta_two);

    println!("\nTotal distance: {}", Calc::dist(&n_origin, &goto));
    println!("\nDistance: {}", Calc::temp_dist(theta_one, &goto));
    
}
