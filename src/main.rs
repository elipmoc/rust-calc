pub mod calc;

use crate::calc::expr_eval;

fn main() {
    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        match expr_eval(&s) {
            Ok(val) => println!("ok:{}", val),
            _ => println!("構文エラー")
        }
    }
}
