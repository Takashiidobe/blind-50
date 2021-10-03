macro_rules! tests {
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

tests! {
    ex1: (1, 1),
    ex2: (2, 2),
    ex3: (3, 3),
}

fn climbing_stairs(n: i32) -> i32 {
    if n == 1 {
        return 1;
    } else if n == 2 {
        return 2;
    }

    let mut prev_prev = 1;
    let mut prev = 2;
    for _ in 3..n {
        let temp = prev_prev + prev;
        prev_prev = prev;
        prev = temp;
    }
    prev_prev + prev
}

fn climbing_stairs_rec(n: i32) -> i32 {
    let mut v = vec![0, 1, 2];
    fn traverse(n: usize, v: &mut Vec<i32>) {
        v.push(v[n - 1] + v[n - 2]);
    }
    for i in 3..=n {
        traverse(i as usize, &mut v);
    }
    v[n as usize]
}

fn main() {
    println!("{}", climbing_stairs(10));
    println!("{}", climbing_stairs_rec(10));
}
