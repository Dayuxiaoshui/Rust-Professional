// main.rs
mod prime_factor; // 引入 prime_factor 模块

fn main() {
    let number = 100;
    let res = prime_factor::find_max_prime_factor(number);
    println!("{number}'s max prime factor: {res}");
}

