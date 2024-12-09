#[allow(dead_code, unused_variables)]
fn example_0() {
    let r: &i32;
    
        let x: i32 = 5;
        r = &x;
    

    println!("r: {}", r);
}

#[allow(dead_code, unused_variables)]
fn example_1() {

    // Allocate space in memory
    let highest_age: i32;

    // Initialize vars
    let alice_age: i32 = 20;
    let bob_age: i32 = 21;

    // Call function

    highest_age = largest(&alice_age, &bob_age);

    // Print output
    println!("Highest age is {}", highest_age);

    fn largest(compare_1: &i32, compare_2: &i32) -> i32 {
        if compare_1 > compare_2 {
            *compare_1
        }
        else    {
            *compare_2
        }
    }
}

#[allow(dead_code, unused_variables)]
fn example_2() {

    // Allocate space in memory
    let highest_age: &i32;

    // Initialize vars
    let alice_age: i32 = 20;  // 'a
    let new_value: i32;

    {
        let bob_age: i32 = 21;      // 'b: 'a

        // Call function
        highest_age = largest(&alice_age, &bob_age);
        new_value = *highest_age;
    }
    // Print output
    println!("Highest age is {}", new_value);

    fn largest<'a, 'b: 'a>(compare_1: &'a i32, compare_2: &'b i32) -> &'a i32 {
        if compare_1 > compare_2 {
            compare_1
        }
        else    {
            compare_2
        }
    }
}
