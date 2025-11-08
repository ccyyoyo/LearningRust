enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let x = 5;
    let result = if x > 3 {
        "大於三"
    } else {
        "不大於三"
    };
    println!("x 的判斷結果：{result}");

    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    };
}
