use std::{
    env,
    pin::{self, Pin, pin},
    task::Poll,
    thread,
    time::Duration,
};

use trpl::{Either, Html, StreamExt};

fn main() {
    let args: Vec<String> = env::args().collect();

    trpl::block_on(async {
        // let title_fn_1 = page_title(&args[1]);
        // let title_fn_2 = page_title(&args[2]);

        //
        // let (url, title) = match trpl::select(title_fn_1, title_fn_2).await {
        //     Either::Left(left) => left,
        //     Either::Right(right) => right,
        // };
        //
        // println!("{url} returned first");
        // match title {
        //     Some(title) => println!("Its page title was: '{title}'"),
        //     None => println!("It had no title."),
        // }
        //

        // let task = trpl::spawn_task(async {
        //     for i in 1..10 {
        //         println!("hi number {i} from the first task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // });
        //
        // for i in 1..5 {
        //     println!("hi number {i} from the second task!");
        //     trpl::sleep(Duration::from_millis(500)).await;
        // }
        //
        // task.await.unwrap();

        // let task_1 = async {
        //     for i in 1..10 {
        //         println!("hi number {i} from the first task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // };
        //
        // let task_2 = async {
        //     for i in 1..5 {
        //         println!("hi number {i} from the second task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // };
        //
        // trpl::join(task_1, task_2).await;
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async move {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![rx_fut, tx1_fut, tx_fut];
        trpl::join_all(futures).await;

        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|val| val * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });

    let (tx, mut rx) = trpl::channel();

    thread::spawn(move || {
        for i in 1..=10 {
            tx.send(i).unwrap();
        }
    });

    trpl::block_on(async move {
        while let Some(i) = rx.recv().await {
            println!("{i}");
        }
    });
}

fn page_title(url: &str) -> impl Future<Output = (&str, Option<String>)> {
    async move {
        let response = trpl::get(url).await.text().await;

        let title = Html::parse(&response)
            .select_first("title")
            .map(|title| title.inner_html());

        (url, title)
    }
}
