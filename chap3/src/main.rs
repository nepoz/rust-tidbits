use std::io;
use std::io::Write;

// Challenge 1: Converting between Fahrenheit and Celsius
fn main() {
    let mut temp_f: String = String::new();

    loop {

        //Prompt user for input
        io::stdout()
            .flush()
            .expect("Couldn't flush out!");
        eprint!("Enter a temperature in F > ");

        // Read user input for fahrenheit
        io::stdin()
            .read_line(&mut temp_f)
            .expect("Couldn't read an input");

        // Change the type of temp to 64 bit int
        let temp_f: f64 = match temp_f.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        let temp_c: f64 = convert_to_celsius(temp_f);
        println!("{}F is {}C", temp_f, temp_c);

        break;
    }

    // Challenge 2 find the nth fibonacci number
    let mut n: String = String::new();

    loop {
        // Prompt user for input
        io::stdout().flush().expect("Couldn't flush stdout!");
        eprint!("Enter n for nth fibonacci number > ");

        // Read in user input
        io::stdin()
            .read_line(&mut n)
            .expect("Couldn't read stdin!");

        let n: u64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        // Get and give user output
        let nth_fib: u64 = get_nth_fibonacci(n);
        println!("Your fibonacci number is {}!", nth_fib);
        break;
    }
}



fn convert_to_celsius(temp_f: f64) -> f64 {
    (temp_f - 32.0) / 1.80
}

fn get_nth_fibonacci(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 1;
    } else {
        return get_nth_fibonacci(n - 1) + get_nth_fibonacci(n - 2)
    }
}
