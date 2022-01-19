use rand::{Rng, rngs::ThreadRng};

mod problems { pub mod sum_two; }

mod euler_one;
mod euler_two;

fn main() {
    euler_one::project_euler_question_one();
    euler_two::project_euler_question_two();

    test_sum_two();
}

fn test_sum_two() {
    let mut rng = rand::thread_rng();
    let length_one: u8 = rng.gen_range(5..20);
    let rng_vector = create_random_vec_of_int(length_one, &mut rng);
    println!("{:?}", rng_vector);
    let sorted_rng_vector = create_random_vec_of_sorted_int(length_one, &mut rng);
    println!("{:?}", sorted_rng_vector);
}

fn create_random_vec_of_int(length: u8, rng: &mut ThreadRng) -> Vec<i32> {
    (0..length).map(|_| rng.gen_range(0..1000)).collect()
}

fn create_random_vec_of_sorted_int(length: u8, rng: &mut ThreadRng) -> Vec<i32> {
    let mut rng_vector = create_random_vec_of_int(length, rng);
    rng_vector.sort();
    rng_vector
}

