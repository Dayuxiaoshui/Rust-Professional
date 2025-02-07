// prime_factor.rs
pub fn find_max_prime_factor(number: u128) -> u128 {
    let mut num = number;
    let mut max_prime = 1;

    // 处理2，直到不能整除为止
    while num % 2 == 0 {
        max_prime = 2;
        num /= 2;
    }

    // 处理奇数因子，从3开始检查
    let mut factor = 3;
    while factor * factor <= num {
        while num % factor == 0 {
            max_prime = factor;
            num /= factor;
        }
        factor += 2;  // 只检查奇数
    }

    // 如果num大于1，说明它本身就是素数
    if num > 1 {
        max_prime = num;
    }

    max_prime
}
