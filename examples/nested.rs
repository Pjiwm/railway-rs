use std::io::Error;

use railway_rs::Railway;

#[derive(Debug)]
struct State {
    count: i32,
    history: Vec<i32>,
}

impl State {
    fn new(num: i32) -> State {
        State {
            count: num,
            history: vec![num],
        }
    }
}

fn add_one(state: State) -> Result<State, Error> {
    let mut new_state = state;
    new_state.count += 1;
    new_state.history.push(new_state.count);
    Ok(new_state)
}

fn log_history(state: State) -> Result<State, Error> {
    println!("{:?}", state.history);
    Ok(state)
}

fn history_length(state: State) -> Result<State, Error> {
    println!("{}", state.history.len());
    Ok(state)
}

fn check_history(state: Result<State, Error>) -> Result<State, Error> {
    match state {
        Ok(s) => {
            if s.history.len() > 10 {
                Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "History too long",
                ))
            } else {
                Ok(s)
            }
        }
        Err(e) => Err(e),
    }
}

fn add(state: State, num: i32) -> Result<State, Error> {
    let mut new_state = state;
    new_state.count += num;
    new_state.history.push(new_state.count);
    Ok(new_state)
}

fn multiply(state: State, num: i32) -> Result<State, Error> {
    let mut new_state = state;
    new_state.count *= num;
    new_state.history.push(new_state.count);
    Ok(new_state)
}

fn state_too_large(state: State) -> Result<State, Error> {
    if state.count > 100 {
        Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Number too large",
        ))
    } else {
        Ok(state)
    }
}

fn log(state: Result<State, Error>) -> Result<State, Error> {
    match state {
        Ok(s) => {
            println!("{}", s.count);
            Ok(s)
        }
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    }
}

fn fix_state(_: Error) -> Result<State, Error> {
    println!("Brought back state to 0");
    Ok(State::new(0))
}

fn is_valid_num(state: Result<State, Error>) -> Result<State, Error> {
    match state {
        Ok(s) => {
            if s.count < 0 {
                Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid number",
                ))
            } else {
                Ok(s)
            }
        }
        Err(e) => Err(e),
    }
}

fn main() {
    let railway_sub = Railway::new(is_valid_num)
        .connect(&add_one)
        .connect(&add_one)
        .connect(&history_length)
        .connect(&|x| add(x, 10))
        .connect(&|x| add(x, 5))
        .connect(&log_history)
        .connect(&history_length);

    let railway_sub_two = Railway::new(&check_history)
        .connect(&|x| multiply(x, 10))
        .connect(&log_history)
        .connect(&state_too_large)
        .recover(&fix_state);

    let railway = Railway::new(&log)
        .extend(railway_sub)
        .extend(railway_sub_two);

    let result = railway.call(Ok(State::new(30)));
    println!("{:?}", result);
}
