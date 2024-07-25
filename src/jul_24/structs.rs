use std::{cell::Cell, fmt::{Debug, Display}, i32, rc::Rc, str::FromStr};

use actix_web::cookie::time::parsing::Parsable;

/// A rectangle of eight-bit grayscale pixels.
/// Named-Field struct
struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}

fn short_hand_expr() {
    let pix1 = Pixel {
        red: 0,
        green: 0,
        blue: 0,
    };
    let pix2 = Pixel {
        // red: pix1.red,
        ..pix1
    };
}

fn compare_type_tulpe_struct() {
    type Email1 = String;
    struct Email2(pub String);
    let email_1: Email1 = "test@example.com".to_string();
    let email_2 = Email2("test@example.com".to_string());

    // email_1.
    // email_2.
}

#[derive(Debug)]
struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

impl Pixel {
    fn rev(&mut self) {
        self.red = u8::MAX - self.red;
        self.green = u8::MAX - self.green;
        self.blue = u8::MAX - self.blue;
    }

    fn show(&self) {
        println!("{:?}", &self);
    }
}

#[test]
fn test_impl() {
    let mut pix = Pixel {
        red: 130,
        green: 40,
        blue: 210,
    };
    pix.show(); // Pixel { red: 130, green: 40, blue: 210 }
    pix.rev();
    pix.show(); // Pixel { red: 125, green: 215, blue: 45 }
}

#[derive(Debug)]
struct Node<'a> {
    tag: &'a str,
    children: Vec<Rc<Node<'a>>>,
}

impl<'a> Node<'a> {
    fn new(tag: &str) -> Node {
        Node {
            tag: tag,
            children: vec![],
        }
    }
    fn append_to(self: Rc<Self>, parent: &mut Node<'a>) {
        parent.children.push(self);
    }
    const ROOT: Node<'a> = Node {
        tag: "Root",
        children: vec![],
    };
}

#[test]
fn test_append_to() {
    let mut root = Node::ROOT;
    let owned = Node::new("owned directly");
    let node_rc = Rc::new(owned);
    node_rc.clone().append_to(&mut root);
    node_rc.append_to(&mut root);
    // node_rc.append_to(&mut root);
}

pub struct MyQueue<'a, T> {
    queue: Vec<&'a T>,
}

impl<'a, T: ToString> MyQueue<'a, T> {
    pub fn new() -> MyQueue<'a, T> {
        MyQueue { queue: vec![] }
    }
    pub fn push(&mut self, t: &'a T) {
        self.queue.push(t);
    }
    pub fn to_string_vec(&mut self) {
        self.queue.iter_mut().map(|element| {
            element.to_string();
        });
    }
}

fn test_interior_mutability() {
    struct MySystem {
        systems: Vec<&'static str>,
        systems_with_cell: Cell<Vec<&'static str>>,
    }

    let my_system = MySystem {
        systems: vec![],
        systems_with_cell: Cell::new(vec![]),
    };
    // my_system.systems = vec!["Notification"];
    my_system.systems_with_cell.set(vec!["Notification"]);
}
#[test]
fn mut_ref_string() {
    use std::cell::RefCell;
    let ref_cell: RefCell<String> = RefCell::new("hello".to_string());
    let r = ref_cell.borrow(); // ok, returns a Ref<String>
    let count = r.len(); // ok, returns "hello".len()
    assert_eq!(count, 5);
    drop(r);
    let mut w = ref_cell.borrow_mut(); // panic: already borrowed, but compiler can't detect
    w.push_str(" world");
}