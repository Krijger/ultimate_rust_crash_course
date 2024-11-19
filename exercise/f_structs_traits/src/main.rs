trait Bite {
    fn bite(&mut self);
}

fn main() {
    // Once you finish #1 above, this part should work.

    let mut carrot = Carrot {
        percent_left: 100.0,
    };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes { grapes_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
}

fn bunny_nibbles<T : Bite>(bite_me: &mut T) {
    for _ in 1..4 {
        bite_me.bite();
    }
}

#[derive(Debug)]
struct Grapes {
    grapes_left: i32,
}

impl Bite for Grapes {
    fn bite(&mut self) {
        self.grapes_left -= 1;
    }
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(&mut self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}
