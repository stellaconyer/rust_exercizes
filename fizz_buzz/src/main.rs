fn main() {
    let mut x = 1;
    while x < 101 {
       match x {
            x if x % 3 == 0 && x % 5 == 0    => { println!("FizzBuzz"); }
            x if x % 3 == 0                  => { println!("Fizz"); }
            x if x % 5 == 0                  => { println!("Buzz"); }
            _                                => { println!("{}", x) } 
       }
       x += 1
    }
}
