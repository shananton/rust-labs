const N: usize = 10;

fn minimum(numbers: &[i32; N]) -> i32{
    let mut min = i32::MAX;
    for n in numbers {
        if *n < min {
            min = *n
        }
    }
    min
}

// Primes are 0-indexed
fn nth_prime(n: u32) -> i32 {
    fn is_prime(n: i32) -> bool {
        let mut i = 2;
        loop {
            if i * i > n {
                break true;
            }
            if n % i == 0 {
                break false;
            }
            i += 1;
        }
    }

    let mut i = 2;
    let mut n = n;
    loop {
        if is_prime(i) {
            if n == 0 {
                break i;
            }
            n -= 1;
        }
        i += 1;
    }
}

// Returns the index of the first element greater then or equal to value
// If there are no such elements, returns N
fn lower_bound(value: i32, numbers: &[i32; N]) -> usize {
    let mut i = 0;
    let mut len = N;
    while len > 0 {
        let step = len / 2;
        let mid = i + step;
        if numbers[mid] < value {
            i = mid + 1;
            len -= step + 1;
        } else {
            len = step;
        }
    }
    i
}

fn main() {
    let numbers: [i32; N] = [3, 5, 8, 1, 0, 4, -5, 8, 2, -2];
    let sorted_numbers: [i32; N] = [-3, -3, 0, 2, 4, 7, 9, 9, 12, 15];

    println!("The minimum of {:?} is {}", numbers, minimum(&numbers));

    for n in [0, 1, 2, 3, 4, 5, 10, 500, 10000] {
        println!("Prime #{} is {}", n, nth_prime(n))
    }

    println!("In {:?},", sorted_numbers);
    for value in [-10, -3, 0, 6, 10, 16] {
        println!("  lower_bound({}) = {}", value, lower_bound(value, &sorted_numbers));
    }
}
