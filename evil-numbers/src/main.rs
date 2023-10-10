use std::time::SystemTime; 
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
    println_colour("Evil Numbers Benchmark, Logan Cammish 2023\n", Color::Black);

    let mut time = 0;
    for x in 1..11 {
        let start = SystemTime::now();
        println!("Test started: {}/10", x);
        for i in 0..5000000 {
            if format!("{:b}",i).matches("1").count() % 2 == 0 {
                print!("\rProgress: ({}%)", (i/50000));
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
                time / 11, time, time / 1000,
                (time ^ 2) / 6);
}
