use std::env;

fn main() {
        let args: Vec<_> = env::args().collect();
        
        if args.len() < 2 {
                    println!("Error: Please provide a number as argument.");
                            return;
                                }

            let i: i32 = args[1].parse().unwrap();
                println!("{} has the minimum {} Collatz steps", total_steps(i), i);
}

    fn total_steps(i: i32) -> i32 {
    
        let mut current_number: i32 = 1;

        loop {    
            let collatz_number  = collatz(current_number);
            if collatz_number == i {
                return current_number;
            }
            else {
                current_number += 1
            }

            }
    }


    fn collatz(n: i32) -> i32 {
        if n == 1 { return 0; }
            match n % 2 {
                        0 => { 1 + collatz(n/2) }
                        _ => { 1 + collatz(n*3+1) }
            }
}

