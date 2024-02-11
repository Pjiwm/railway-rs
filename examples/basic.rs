use railway_rs::Railway;

#[derive(Debug)]
struct Msg {
    text: String,
    number: u32,
    id: u32,
}

fn log_message(message: Msg) -> Result<Msg, String> {
    println!("Logging message: {}", message.text);
    Ok(message)
}

fn check_id_even(message: Msg) -> Result<Msg, String> {
    if message.number % 2 != 0 {
        println!("number is not even");
        Err("id is 0".to_owned())
    } else {
        println!("number is even");
        Ok(message)
    }
}

fn done(message: Msg) -> Result<Msg, String> {
    println!("DONE: {}", message.id);
    println!("====================");
    Ok(message)
}

fn patch_result(err: String) -> Result<Msg, String> {
    Ok(Msg {
        text: err,
        number: 0,
        id: 1,
    })
}

fn load_message(msg: Result<Msg, String>) -> Result<Msg, String> {
    msg
}

fn main() {
    let railway = Railway::new(load_message)
        .connect(&log_message)
        .connect(&check_id_even)
        .recover(&|mut x| {
            x.push_str(" - closure");
            Err(x)
        })
        .recover(&patch_result)
        .connect(&done);

    let result = railway.call(Ok(Msg {
        text: "Hello".to_owned(),
        number: 1,
        id: 1,
    }));
    let result2 = railway.call(Ok(Msg {
        text: "Hello".to_owned(),
        number: 2,
        id: 1,
    }));
    println!("{:?}", result);
    println!("{:?}", result2);
}
