fn main() {
    // 使用 Option<T> 來表示可能有值或沒有值的情況
    let maybe_num: Option<i32> = Some(5);
    let none_value: Option<i32> = None;

    match maybe_num {
        Some(n) => println!("有值：{n}"),
        None => println!("沒有值"),
    }

    // 使用 Result<T, E> 來表示可能成功或失敗的操作
    enum Result<T, E> {
    Ok(T),
    Err(E),
    }

    let f = File::open("not_exist.txt");

    match f {
        Ok(file) => println!("成功開啟：{:?}", file),
        Err(e) => println!("發生錯誤：{}", e),
    }
}

// 使用 ? 運算子來簡化錯誤處理
use std::fs::File;
use std::io::{self, Read};

fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?; // 錯誤自動傳出
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
