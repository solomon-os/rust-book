// pub trait Draw {
//     fn draw(&self);
// }
//
// struct Screen {
//     components: Vec<Box<dyn Draw>>,
// }
//
// struct Button {}
//
// struct Input {}
//
// impl Draw for Button {
//     fn draw(&self) {
//         println!("drawing button")
//     }
// }
//
// impl Draw for Input {
//     fn draw(&self) {
//         println!("drawing Input")
//     }
// }
//
// fn main() {
//     let button = Button {};
//     let input = Input {};
//
//     let screen = Screen {
//         components: vec![Box::new(button), Box::new(input)],
//     };
//
//     for i in screen.components {
//         i.draw();
//     }
// }
//
//

use chapter_18::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
