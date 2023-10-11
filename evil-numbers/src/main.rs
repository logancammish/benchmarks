use std::time::SystemTime; 
use std::env;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn println_colour(input: &str, colour: Color) {
    fn set_colour(colour: Color) {
        StandardStream::stdout(ColorChoice::Auto)
            .set_color(ColorSpec::new()
            .set_fg(Some(colour)))
            .unwrap();
    }

    set_colour(colour);
    println!("{}", input);
    set_colour(Color::White)
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut length = args[1].parse::<i32>().unwrap();
    if length < 1 {
    	length = 50000000;
    }
    let mut tests = args[2].parse::<u128>().unwrap();
    if tests < 1 {
        tests = 10;
    }
    tests += 1;
    println_colour("Evil Numbers Benchmark, Logan Cammish 2023\n", Color::Black);
    println!("Testing with preset: {} repitions (should be above 1000000 for accurate results)\n", length);

    let mut time = 0;
    for x in 1..tests {
        let start = SystemTime::now();
        println!("Test started: {}/{}", x, tests);
        for i in 0..length {
            if format!("{:b}",i).matches("1").count() % 2 == 0 {
                print!("\rProgress: ({}%)", (i/(length / 100)));
            }
        }
        print!("\rProgress: (100%)");

        let finish = SystemTime::now();
        let duration = finish
            .duration_since(start)
            .unwrap()
            .as_millis();

        println!("\nTest {} completed in: {:?}ms\n", x, duration);
        time += duration;
    }

    println_colour("\nTest completed successfully! Getting results...", Color::Green);
    println!("Average time: {} ms | Overall time: {} ms (≈{} s)\nScore: {}", 
                time / tests, time, time / 1000,
                (time ^ 2) / 6);
}
