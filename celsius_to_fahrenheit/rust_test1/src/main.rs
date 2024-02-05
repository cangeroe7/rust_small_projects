use std::io;

fn main() {
    fn get_number() -> f64{
        println!("Convert Celsius to Fahrenheit\n");
        
        loop {
            let mut cs = String::new();
            println!("Input Degrees Celsius: ");
            io::stdin()
                .read_line(&mut cs)
                .expect("That is no good buddy");
            match cs.trim().parse() {
                Ok(num) => return num,
                Err(_) => continue
            };
            
        }
    }
    let cs: f64 = get_number();
    // let cs = 20.0;
    fn c_to_f(cs: f64) -> f64 {
        (cs * (9. / 5.) ) + 32.
    }
    let fh = c_to_f(cs);   
    println!("celsius: {cs}\nfahrenheit: {fh}")
}