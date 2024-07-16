use std::process::abort;

fn print_padovan() {
    let str = r##""""""##;
    let mut padovan = vec![1, 1, 1]; // allocated here
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
    // dropped here
}

// #[derive(Clone, Copy)]
#[derive(Debug, Clone)]
struct Person {
    name: String,
    birth: i32,
}

fn ownership_struct_vec() {
    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });
    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
    // let name = composers[0].name;
    // let birth = composers[0].birth;

    // Q1. who owns Vector?
    // Q2. who owns Person Instance whose name is "Lully"?
    // Q3. who owns the text "Lully"?
    // Q4. What's the difference between a name and a birth?"
}

fn move_ownership() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s;
    let u = t;

    let mut person: Person;
    'some_block: {
        let mut composers = Vec::new();
        composers.push(Some(Person {
            name: "Palestrina".to_string(),
            birth: 1525,
        }));
        composers.push(Some(Person {
            name: "Dowland".to_string(),
            birth: 1563,
        }));
        composers.push(Some(Person {
            name: "Lully".to_string(),
            birth: 1632,
        }));
        // for composer in &composers {
        //     println!("{}, born {}", composer.name, composer.birth);
        // }
        person = composers.pop().unwrap().unwrap();

        let person = std::mem::take(&mut composers[2]);
        let person = composers.remove(2);
    }
    println!("{:?}", person);
    let ownership_taker = person;
    // println!("{:?}", person);    // ownership moved
}

#[test]
fn test_print_padovan() {
    print_padovan();
}

#[test]
fn test_move_ownership() {
    move_ownership();
}
