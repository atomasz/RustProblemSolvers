pub fn project_euler_question_two() {
    let mut value1 = 0;
    let mut value2 = 1;
    let mut even_fib_sum = 0;

    while value1 < 4_000_000 {
        let fib_number = value1 + value2;
        if fib_number % 2 == 0 {
            even_fib_sum += fib_number;
        }
        value1 = value2;
        value2 = fib_number;
    }

    println!("The anser is {}", even_fib_sum);
}