enum Status {
    Todo,
    Doing,
    Done,
}

enum Message {
    Quit,                       // 沒有資料
    Move { x: i32, y: i32 },    // 結構形式
    Write(String),              // 單一資料
    ChangeColor(i32, i32, i32), // 多筆資料
}

fn handle_message(msg: Message) {
    match msg {
        Message::Quit => println!("Exit."),
        Message::Move { x, y } => println!("Move to ({x}, {y})"),
        Message::Write(text) => println!("Text: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: ({r}, {g}, {b})"),
    }
}

// enum Option<T> {
// //如果有值，用 Some(value)；
// // 沒有值，用 None；
//     Some(T),
//     None,
// }

fn maybe_add(x: Option<i32>, y: Option<i32>) -> Option<i32> {
    match (x, y) {
        (Some(a), Some(b)) => Some(a + b),
        _ => None,
    }
}

// 1️⃣ 定義一個 enum `Shape`，包含三種圖形：Circle(r)、Rectangle(w,h)、Triangle(a,b,c)
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

// 2️⃣ 實作一個函式 `area(shape: &Shape) -> f64`，根據形狀計算面積：
//     - 圓形：π * r²
//     - 長方形：w * h
//     - 三角形：用海龍公式
fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0; // 半周長
            (s * (s - a) * (s - b) * (s - c)).sqrt() // 海龍公式
        }
    }
}
// 3️⃣ 在 main() 建立三種圖形，印出面積



fn main() {
    
    let s = Status::Doing;
    match s {
        Status::Todo => println!("待辦事項"),
        Status::Doing => println!("進行中"),
        Status::Done => println!("已完成"),
    }

    let m1 = Message::Quit;
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::Write(String::from("Hello"));
    let m4 = Message::ChangeColor(255, 0, 0);

    println!("Handling messages using pattern matching:");
    handle_message(m1);
    handle_message(m2); 
    handle_message(m3); 
    handle_message(m4); 


    // if let example
    let maybe_value = Some(42);
    if let Some(v) = maybe_value {
        println!("有值：{}", v);
    }

    // 3️⃣ 在 main() 建立三種圖形，印出面積
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(4.0, 6.0);
    let triangle = Shape::Triangle(3.0, 4.0, 5.0);

    println!("Circle area: {}", area(&circle));
    println!("Rectangle area: {}", area(&rectangle));
    println!("Triangle area: {}", area(&triangle));

}
