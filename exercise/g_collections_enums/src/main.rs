// Silence some warnings that could distract from the exercise
#![allow(unused_variables, unused_mut, dead_code)]

enum Shot {
    Bullseye,
    Hit {distance: f64},
    Miss,
}

impl Shot {
    fn points(self) -> i32 {
        match self {
            Self::Bullseye => 5,
            Self::Hit{distance: x} if x < 3.0 => 2,
            Self::Hit{distance: _} => 1,
            Self::Miss => 0,
        }
    }
}

fn main() {
    // Simulate shooting a bunch of arrows and gathering their coordinates on the target.
    let arrow_coords: Vec<Coord> = get_arrow_coords(5);
    let mut shots: Vec<Shot> = Vec::new();

    for coord in arrow_coords {
        let dist = coord.distance_from_center();
        let shot = match dist {
            _ if dist < 1.0 => Shot::Bullseye,
            _ if dist < 5.0 => Shot::Hit { distance: dist },
            _ => Shot::Miss,

        };
        shots.push(shot);
    }

    let mut total = 0;
    for shot in shots {
        total += shot.points();
    }

    println!("Final point total is: {}", total);
}

// A coordinate of where an Arrow hit
#[derive(Debug)]
struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    fn distance_from_center(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    fn print_description(&self) {
        println!(
            "coord is {:.1} away, at ({:.1}, {:.1})",
            self.distance_from_center(),
            self.x,
            self.y);
    }

}

// Generate some random coordinates
fn get_arrow_coords(num: u32) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    for _ in 0..num {
        let coord = Coord {
            x: (rand::random::<f64>() - 0.5) * 12.0,
            y: (rand::random::<f64>() - 0.5) * 12.0,
        };
        coords.push(coord);
    }
    coords
}