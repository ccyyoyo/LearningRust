fn main() {
    let msg = String::from("Hi!");
    let greeting = msg;

    // This will cause a compile-time error because msg's ownership has been moved to greeting.
    // String 不是 Copy 型別，也就是說，它在被指派給別人時，會「移動所有權（move）」，而不是複製記憶體。
    // println!("msg: {}", msg);

    // 正確的使用方式是使用 greeting
    
    println!("greeting: {}", greeting);

    // 或是
    // 1. let greeting = msg.clone(); 深拷貝 各自擁有自己的記憶體
    // 2. let greeting = &msg; 浅拷贝，greeting 只是一个指向 msg 的引用

    // 借用
    // 使用引用（reference） & 來借用資料，而不是取得所有權，只能讀

    // 可變借用
    // 使用可變引用 &mut 來借用資料，可以修改資料的內容
    // example: 
    let mut s = String::from("Hello");
    let r = &mut s;
    r.push_str(", world!");
    println!("r: {}", r);
    println!("s: {}", s);

    // 一個值同一時間只能有 
    // 一個可變借用。
    // 或者有 多個不可變借用。
    // 但「不能同時」兩種並存。
}
