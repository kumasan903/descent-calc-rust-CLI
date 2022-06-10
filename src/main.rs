use std::io;
use std::f32::consts::PI;

fn main() {
    println!("--------------------");
    
    // 現在高度を入力
    println!("enter current altitude(FL)");
    let mut current_alt = String::new();

    // 現在高度の処理
    io::stdin()
        .read_line(&mut current_alt)
        .expect("Failed to read line");

    // 降下目標高度を入力
    println!("enter target altitude(FL)");
    let mut target_alt = String::new();

    // 降下目標高度の処理
    io::stdin()
        .read_line(&mut target_alt)
        .expect("Failed to read line");

    // 入力された値を数値に変換

    let current_alt = current_alt.trim();
    let target_alt = target_alt.trim();

    let current_alt: f32 = current_alt.parse().unwrap();
    let target_alt: f32 = target_alt.parse().unwrap();
    
    // 降下に必要な距離の計算
    let result = ((current_alt * 100.0) - (target_alt * 100.0)) / (3.0 * (PI / 180.0)).tan() / 6076.0;

    //結果の表示
    println!("--------------------");
    let result = (result * 10.0).round() / 10.0;
    println!("required {} nm", result);

    let difference = (current_alt - target_alt) * 100.0;
    println!("difference {} ft",difference);
    println!("--------------------");
}
