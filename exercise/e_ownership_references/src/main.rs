// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    inspect(&arg);

    change(&mut arg);
    println!("I have many {}", arg);

    if eat(arg) {
       println!("Might be bananas");
    } else {
       println!("Not bananas");
    }

    // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

    // Challenge: Write a function "bedazzle" that takes a mutable reference to a String and
    // ignores what is in the string and replaces the contents of the string with the String
    // "sparkly". Then uncomment the code below.
    //
    // Hint: You will need to dereference the mutable reference in order to assign it a
    // new value.
    //
    // let mut material = "mud".to_string();
    // println!("This material is just `{}`.", material);
    // bedazzle(&mut material);
    // println!("Wow! Now the material is `{}`!", material);
}

fn inspect(s: &String) {
    let count = if s.ends_with("s")  {
        "plural"
    } else {
        "singular"
    };
    println!("{} is {}", s, count);
}

fn change(s: &mut String) {
    if !s.ends_with("s")  {
        s.push_str("s")
    }
}

fn eat(s: String) -> bool {
    s.starts_with("b") && s.contains("a")
}