pub fn project_euler_question_one() {
    let mut sum_value = 0;

    for value in 1..999 {
        if value % 3 == 0 || value % 5 == 0 {
            sum_value += value;
        }
    }

    println!("The anser is {}", sum_value);
}