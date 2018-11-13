fn main() {
   //loop_sample();
   while_sample();
}

fn loop_sample() {
    loop {
        println!("again!!");
    }
}

fn while_sample() {
    let mut number = 5;

    while number != 0 {
        println!("{}!!", number);
        number = number - 1;
    }

    println!("LIFTOFF!!!");
}
