use indicatif::ProgressBar;
use std::env;
use std::f64;



use std::thread;
use std::time::Instant;
use sysinfo::System;

mod benchmark;

fn stress_cpu() {
    let mut handles = vec![];

    // launch 10 functions in parallel
    for i in 0..10 {
        let handle = thread::spawn(move || {
            match i {
                0 => {
                    
                    benchmark::factorial::factorial(10);
                }
                1 => {
                    
                    benchmark::fibonacci::fibonacci(40);
                }
                2 => {
                    
                    benchmark::sum_of_squares::sum_of_squares(10000);
                }
                3 => {
                   
                    let _primes = benchmark::prime_numbers::prime_numbers(10000);
                }
                4 => {
                    
                    let _matrix = benchmark::matrix::matrix_multiplication(20);
                }
                5 => {
                    
                    benchmark::pi::pi_approximation(1000000);
                }
                6 => {
                    
                    let _sorted = benchmark::bubble_sort::bubble_sort(10000);
                }
                7 => {
                   
                    benchmark::gcd::gcd(123456, 789012);
                }
                8 => {
                    
                    let _primes = benchmark::sieve::sieve_of_eratosthenes(10000);
                }
                9 => {
                   
                    benchmark::factorial_sum::factorial_sum(10);
                }
                _ => {}
            }
        });
        handles.push(handle);
    }

   
    for handle in handles {
        handle.join().unwrap();
    }
}




fn main() {
    let args: Vec<String> = env::args().collect();
    let num_calcs_arg: Option<&String> = args.get(1);
    let num_calcs: u64 = match num_calcs_arg {
        Some(num_calcs_arg) => num_calcs_arg.trim().parse::<u64>().unwrap(),
        None => 400000000, 
    };
    let num_iters: u64 = 1000;
    let total_calc: u64 = num_calcs * num_iters;


    println!(
        "Running {} calculations over {} iterations each with a total of {} calculations.",
        &num_calcs, &num_iters, &total_calc,
    );


    let mut results = Vec::new();
    let mut test_number = 1;

    loop {
        
        let mut sys = System::new_all();
        sys.refresh_all();
        println!("System name:                 {:?}", sysinfo::System::name());
        println!("System kernel version:       {:?}", sysinfo::System::kernel_version());
        println!("System OS version:           {:?}", sysinfo::System::os_version());
        println!("System host name:            {:?}", sysinfo::System::host_name());
        println!("Number of available threads: {}", sys.cpus().len());
        println!("CPU’s architecture:          {:?}", sysinfo::System::cpu_arch());
        println!("Global CPUs usage:           {:?}", sys.global_cpu_usage());

        let available_cores: u64 = thread::available_parallelism().unwrap().get() as u64;
        let _iter_per_core: u64 = num_calcs / available_cores;

        let now = Instant::now();

        let bar = ProgressBar::new(num_iters);
        for _i in 0..num_iters {
            stress_cpu(); 
            bar.inc(1);
        }
        bar.finish();

        let elapsed = now.elapsed();
        let calc_per_sec: f64 = (total_calc as f64) / (elapsed.as_secs() as f64);

        

        println!("Test #{} completed", test_number);
        println!("Total runtime: {:.2?}", elapsed);
        println!("Calculations per second: {:.2?} seconds.", calc_per_sec);

        results.push((test_number, calc_per_sec));

        let mut input = String::new();
        println!("Would you like to repeat test? (y/n): ");
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim().to_lowercase() != "y" {
            break;
        }

        test_number += 1; 
    }

    let best_performance = results.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap();
    
    println!(
        "\nBest performance (calculations per second): Test #{} with {:.2} calculations/sec",
        best_performance.0, best_performance.1
    );
    
}