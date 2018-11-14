fn main() {
   //loop_sample();
   while_sample();
   while_with_collections();
   for_better_for_collection();
}

//fn loop_sample() {
//    loop {
//        println!("again!!");
//    }
//}
//
fn while_sample() {
    let mut number = 5;

    while number != 0 {
        println!("{}!!", number);
        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

fn while_with_collections() {
    let a = [50,40,30,20,10];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}

fn for_better_for_collection() {
    let a = [50,40,30,20,10];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
