#![feature(pattern)]

mod day1;
mod day2;

fn main() {
    day1::solve().unwrap();
    day2::solve().unwrap();
}

#[cfg(test)]
pub mod tests {
    use env_logger::Env;

    pub fn init_logger() {
        env_logger::init_from_env(Env::default().default_filter_or("debug"));
    }
}
