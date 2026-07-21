use std::{
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// #[derive(Debug)]
// enum List {
//     Cons(i32, Rc<RefCell<List>>),
//     Nil,
// }

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));
    // dbg!(list);

    // let x = 5;
    // let y = MyBox::new(x);
    //
    // assert_eq!(5, *y);
    //
    // let name = MyBox::new(String::from("Solomon"));
    // hello(&name);
    //
    // let a = Rc::new(RefCell::new(Cons(
    //     5,
    //     Rc::new(RefCell::new(Cons(10, Rc::new(RefCell::new(Nil))))),
    // )));
    // println!("count after creating a = {}", Rc::strong_count(&a));
    // let _b = Cons(3, Rc::clone(&a));
    // println!("count after creating b = {}", Rc::strong_count(&a));
    // {
    //     let _c = Cons(4, Rc::clone(&a));
    //     println!("count after creating c = {}", Rc::strong_count(&a));
    // }
    // println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

fn hello(name: &str) {
    println!("hello {name}");
}
