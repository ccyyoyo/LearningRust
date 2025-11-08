// 1. 定義一個 struct `Rectangle`，包含 width, height（整數）
// 2. 為它實作：
//    - 方法 area() 回傳面積
//    - 方法 is_square() 判斷是否為正方形
// 3. 讓它支援 {:?} 除錯輸出與 {} 格式化輸出
// 4. 在 main() 建立幾個矩形並印出結果

// 使用 `derive` 屬性，自動為 Rectangle 實作 Debug trait，使其可以使用 `{:?}` 格式化輸出
#[derive(Debug)]
// 定義一個名為 `Rectangle` 的結構體 (struct)
struct Rectangle{
    width: u32,
    height: u32,
}

// 為 `Rectangle` 結構體實作方法 (methods)
impl Rectangle {

    // 定義一個名為 `area` 的方法
    // `&self` 表示這個方法借用了目前實例的不可變參考
    // -> u32 表示這個方法會回傳一個 32 位元無號整數
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 定義一個名為 `is_square` 的方法，判斷是否為正方形
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

// 定義一個名為 `Print` 的 trait (特徵)，類似其他語言的介面 (interface)
trait Print {
    // 定義一個 `info` 方法，回傳一個字串
    fn info(&self) -> String;
}

// 為 `Rectangle` 實作 `Print` trait
impl Print for Rectangle {
    // 實作 `info` 方法
    fn info(&self) -> String {
        // 使用 `format!` 宏來建立一個格式化的字串
        format!("Rectangle {{ width: {}, height: {} }}", self.width, self.height)
    }
}

impl std::fmt::Display for Rectangle {
    // 實作 `fmt` 方法，定義如何使用 `{}` 格式化輸出
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 使用 `write!` 宏將格式化的字串寫入到格式化器中
        write!(f, "I am a Rectangle: width = {}, height = {}", self.width, self.height)
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 40, height: 40 };

    // 使用 `{}` (Display trait) 來印出 rect1 的一般資訊
    println!("Display: {}", rect1);
    // 使用 `{:?}` (Debug trait) 來印出 rect1 的除錯資訊
    println!("Debug: {:?}", rect1);
    // 使用 `{:#?}` 來印出更易讀的除錯資訊 (pretty print)
    println!("Pretty Debug:\n{:#?}", rect1);

    println!("rect1: {}", rect1.info());  
    println!("Area of rect1: {}", rect1.area());
    println!("Is rect1 a square? {}", rect1.is_square());   

    println!("rect2: {}", rect2.info());  
    println!("Area of rect2: {}", rect2.area());
    println!("Is rect2 a square? {}", rect2.is_square());   
}
