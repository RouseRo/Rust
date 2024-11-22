//Problem 2: Complete the code below 

use std::thread;

fn main() {
    
    let handle_1 = thread::spawn(|| {
        let mut sum = 0;
        let range = 0..=1_000;
        for num in range {
            sum += num;
        }
        sum
    });
    

    let handle_2 = thread::spawn(|| {
        let mut sum = 0;
        let range = 1_001..=2_000;
        for num in range {
            sum += num;
        }
        sum
    });
    

    let handle_3 = thread::spawn(|| {
        let mut sum = 0;
        let range = 2_001..=3_000;
        for num in range {
            sum += num;
        }
        sum
    });

    // Note: The thread spawn returns a joinhandle type. If there is anything returned from 
          // closure inside the thread, it will be inside the joinhandle type. In this case, it will be Joinhandle<i32>. 
          // You can access the returned i32 value by calling .unwrap() on join. 
    

    // Todo!: Insert a code for creating another thread which will compute the summation from 1001 - 2000
    
    
    // Todo!: Insert a code for creating another thread which will compute the summation from 2001 - 3000

    // let mut sum;
    let sum = handle_1.join().unwrap() +  handle_2.join().unwrap() + handle_3.join().unwrap();
    
    // Todo!: Insert code to make sure that the summation is computed correctly. 
    // Summation will be computed correctly, if all the threads go to completion. 
    
    println!("Final Summation Result {sum}");
}
