fn main() {
    let x = 5;
    println!("x = {x}");

    let mut y = 10;
    println!("y = {y}");
    y = 15;
    println!("y = {y}");

    let name = "Annie";
    println!("Hello, {name}!");
    println!("Hello, {}!", name);

    let mut count = 0;
    // 多層巢狀時能控制跳出哪一層 loop
    'counting_up: loop {
        println!("count = {count}");
        count += 1;
        if count == 3 {
            break 'counting_up;
        }
    }
}
