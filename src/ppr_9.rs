fn is_prime(n: &u32) -> bool {
    match *n {
        0 | 1 => false, // 0 і 1 не прості
        2 => true, // 2 — єдине парне просте число
        _ if n % 2 == 0 => false, // усі парні числа більше 2 не є простими
        _ => {
            let limit = (*n as f64).sqrt() as u32;
            (3..=limit).step_by(2).all(|i| n % i != 0)
        }
    }
}

#[test]
fn ppr_9test() {
    let test_data = [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (100, false),
        (10007, true),
    ];
    for (n, expected) in test_data.iter() {
        let result = is_prime(n);
        println!("Чи {} є простим числом? Отримуємо: {}", n, result);
    }
    test_data.iter().for_each(|(n, prime)| {
        assert_eq!(is_prime(n), *prime);
    });
}