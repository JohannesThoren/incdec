use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::{get, routes, State};

#[get("/increment")]

pub async fn increment(counter: &State<Counter>) -> String {
    let mut count = counter.count.load(Ordering::Relaxed);
    let max = counter.max.load(Ordering::Relaxed);
    if count == max {
        count = 0;
    } else {
        count = count + 1;
    }
    counter.count.store(count, Ordering::Relaxed);
    format!("{}", count)
}
#[get("/decrement")]
pub async fn decrement(counter: &State<Counter>) -> String{
    let mut count = counter.count.load(Ordering::Relaxed);
    let max = counter.max.load(Ordering::Relaxed);
    if count == 0 {
        count = max;
    } else {
        count = count - 1;
    }
    counter.count.store(count, Ordering::Relaxed);
    format!("{}", count)
}

pub struct Counter {
    count: AtomicUsize,
    max: AtomicUsize,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = dotenv::dotenv();
    let _rocket = rocket::build()
        .manage(Counter {
            count: AtomicUsize::new(0),
            max: AtomicUsize::new(
                std::env::var("COUNTER_MAX")
                    .expect("now 'COUNTER_MAX' environment variable. please set it")
                    .parse::<usize>()
                    .unwrap(),
            ),
        })
        .mount("/", routes![increment, decrement])
        .launch()
        .await?;

    Ok(())
}
