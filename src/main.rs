use std::io;
use std::f32::consts::PI;

fn main() {
    println!("--------------------");
    
    // 現在高度を入力
    println!("enter current altitude");
    let mut current_alt = String::new();

    // 現在高度の処理
    io::stdin()
        .read_line(&mut current_alt)
        .expect("Failed to read line");

    // 降下目標高度を入力
    println!("enter target altitude");
    let mut target_alt = String::new();

    // 降下目標高度の処理
    io::stdin()
        .read_line(&mut target_alt)
        .expect("Failed to read line");

    // 入力された値を数値に変換

    let current_alt = current_alt.trim();
    let target_alt = target_alt.trim();

    let current_alt_num: f32 = current_alt.parse().unwrap();
    let target_alt_num: f32 = target_alt.parse().unwrap();


    // デバッグ用
    println!("current altitude is {}",current_alt_num);
    println!("target altitude is {}",target_alt_num);

    
    // 降下に必要な距離の計算
    //let current_alt_num = current_alt_num * 100.0;
    //let target_alt_num = target_alt_num * 100.0;
    let pi = PI;
    let result = ((current_alt_num * 100.0) - (target_alt_num * 100.0)) / (3.0 * (pi / 180.0)).tan() / 6076.0;

    println!("required {} nm", result);
    

    //Math.round(   ( | (Number(Number(cra.value)*100-Number(toa.value)*100) | / Math.tan( Number(pas.value)*(Math.PI / 180) )))/6076   ) + " nm"; //結果を表示(必要距離)
}
