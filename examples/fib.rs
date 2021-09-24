macro_rules! fib_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(expected, fib(input));
        }
    )*
    }
}

fib_tests! {
    fib_0: (0, 0),
    fib_1: (1, 1),
    fib_2: (2, 1),
    fib_3: (3, 2),
    fib_4: (4, 3),
    fib_5: (5, 5),
    fib_6: (6, 8),
}

fn fib(n: u16) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _ in 1..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    sum
}

fn main() {
    println!("{}", fib(10));
}
