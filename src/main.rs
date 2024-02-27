fn main() {
    let total_fibo: usize = 10;
    fibonacci_ret(total_fibo);
}

fn fibonacci_ret(total_fibo: usize) {

    let mut num_init: usize = 1; 
    let mut num_soma : usize = num_init;

    for item in 0..total_fibo 
    {
        num_soma += num_init;
        num_init = num_soma - num_init;

        println!("{}", num_soma); 
    }
}

