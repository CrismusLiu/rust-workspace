use tokio::task;

#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3];

    task::spawn(async move {
        println!("vec: {:?}", v);

        let rc = Rc::new("hello");

        task::yield_now().await;

        println!("{}", rc);
    });
}
