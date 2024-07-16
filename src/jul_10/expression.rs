use std::{
    fmt::Debug,
    fs::File,
    io::{self, stdin, Write},
    process::exit,
    thread::sleep,
    time::Duration,
};

use rand;

pub enum MyAlphatbet {
    A,
    B,
    C,
    D,
    E,
}

fn random_bool() -> bool {
    rand::random::<bool>()
}

fn match_expression() -> Result<(), io::Error> {
    let option_i32: Option<i32> = Some(1);
    let result_i32: Result<i32, ()> = Ok(1);
    let alphabet = MyAlphatbet::A;
    // match option_i32 {

    // }
    // match result_i32{

    // }

    match alphabet {
        MyAlphatbet::A => todo!(),
        MyAlphatbet::C => todo!(),
        MyAlphatbet::B => todo!(),
        MyAlphatbet::D => todo!(),
        MyAlphatbet::E => todo!(),
    }

    let unwrapped_int = match option_i32 {
        Some(some_num) => {
            println!("dfdf");
            some_num
        }
        None => 0,
    };

    let a = String::new();

    let output = match File::create("something") {
        Ok(f) => f,
        Err(err) => return Err(err),
    };

    if let Some(some_int) = option_i32 {
        println!("{:?}", some_int);
    }

    Ok(())
}

fn if_expression() {
    let arr = [1];
    let print_msg = if arr.is_empty() {
        "Empty array"
    } else {
        "Non empty array"
    };
    println!("{:?}", print_msg);
}

fn declarations() {
    let name;
    if random_bool() {
        name = "True!";
    } else {
        name = "False!";
    }
}

#[test]
fn loop_expression() {
    // let mut rand_bool;
    // let answer = loop {
    //     rand_bool = random_bool();
    //     if rand_bool {
    //         break rand_bool;
    //     } else {
    //         println!("False!!");
    //     }
    //     sleep(Duration::from_millis(5));
    // };

    // assert_eq!(answer, true);
    // println!("Got {:?}", answer);

    let res = 'first_loop: loop {
        println!("first_loop start!");
        'second_loop: loop {
            // label loop's lifetime
            println!("second_loop start!");
            break 'first_loop "break with specifying first_loop!!"; // label can be used `continue` too
            println!("second_loop end!");
            break;
        }
        println!("first_loop end!");
        break "break without specifying alias";
    };

    println!("{:?}", res);
}

fn serve_forever() {
    panic!()
    // loop {
    //     let mut a= 1;
    //     a = 2;
    //     exit(0)
    // }
}

fn function_mehtod_call() {
    #[derive(Debug)]
    struct Person {
        name: &'static str,
        age: u8,
    }

    impl Person {
        fn print_name(&self) {
            println!("{:?}", self.name);
        }
        fn print_age(self) {
            println!("{:?}", self.name);
        }
    }
    let person = Person {
        name: "고승우",
        age: 20,
    };

    person.print_name();
    println!("{:?}", person);
    // person.print_age();
    // println!("{:?}", person);
}
