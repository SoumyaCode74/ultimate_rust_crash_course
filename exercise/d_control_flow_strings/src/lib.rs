pub fn sum() {
    let mut sum = 0;
    // 2. Use a "for loop" to iterate through integers from 7 to 23 *inclusive* using a range
    // and add them all together (increment the `sum` variable).  Hint: You should get 255
    // Run it with `cargo run sum`
    for i in 7..=23{
        sum += i;
    }
    println!("The sum is {}", sum);
}

pub fn double() {
    let mut count = 0;
    let mut x = 2;
    println!("Initial value of x is {}",x);
    // 3. Use a "while loop" to count how many times you can double the value of `x` (multiply `x`
    // by 2) until `x` is larger than 500.  Increment `count` each time through the loop. Run it
    // with `cargo run double`  Hint: The answer is 9 times.
    'main: while 1 != 0{
        x *= 2;
        if x <= 500{
            count += 1;
        }
        else{
            break 'main;
        };
        
    };

    println!("You can double x {} times until x is larger than 500", count);
}

pub fn count(arg: String) {
    // Challenge: Use an unconditional loop (`loop`) to print `arg` 8 times, and then break.
    // You will need to count your loops, somehow.  Run it with `cargo run bananas`
    //
    // print!("{} ", arg); // Execute this line 8 times, and then break. `print!` doesn't add a newline.
    let mut count: i8 = 1;
     loop {
        if count <= 8 {
            print!("{} ", arg);
            count += 1;
        }
        else {
            break;
        };
    }
    println!(); // This will output just a newline at the end for cleanliness.
}