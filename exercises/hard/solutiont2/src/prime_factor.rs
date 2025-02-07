pub fn find_max_prime_factor(number: u128) -> u128 {
    let mut num = number;
    let mut max_prime = 1;

    if num == 0 {
        return 0;
    }

    // 处理所有因子2
    while num % 2 == 0 {
        max_prime = 2;
        num /= 2;
    }

    if num == 1 {
        return max_prime;
    }

    // 处理奇数因子
    let mut factor = 3;
    while factor * factor <= num {
        let mut changed = false;
        while num % factor == 0 {
            max_prime = factor;
            num /= factor;
            changed = true;
        }
        // 检查是否剩下的num是质数
        if changed {
            if num == 1 {
                return max_prime;
            }
            if is_prime(num) {
                return std::cmp::max(max_prime, num);
            }
        }
        factor += 2;

        if num == 1 {
            break;
        }
    }

    // 如果剩下的num是质数
    if num > 1 {
        max_prime = max_prime.max(num);
    }

    max_prime
}

// Miller-Rabin 质数测试
fn is_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    } else if n <= 3 {
        return true;
    } else if n % 2 == 0 {
        return false;
    }

    // 将n-1分解为 d * 2^s
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    // 使用确定性的Miller-Rabin基数
    let bases = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];

    for &a in &bases {
        if a >= n {
            continue;
        }
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut composite = true;
        for _ in 0..s - 1 {
            x = mod_pow(x, 2, n);
            if x == n - 1 {
                composite = false;
                break;
            }
        }
        if composite {
            return false;
        }
    }
    true
}

// 快速模幂运算
fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}