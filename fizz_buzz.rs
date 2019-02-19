
fn main(){
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            let mut input_num: i32 = input.parse().unwrap();
            for x in 1..input_num + 1 {
                if (x % 3 != 0) && (x % 5 != 0) {
                    print!("{}", x);
                    continue;
                }
                if (x % 3 == 0) {
                    print!("Fizz");
                }
                if (x % 5 == 0) {
                    print!("Buzz");
                }
            }
        }
        Err(error) => println!("error: {}", error),
    }
}
