// 個人的な復習
// 型宣言できるけど、しなくてもrustは型を予測してくれる
// fn main() {
//     let x:i32 = 20;
//     println!("x = {}", x);

// }

fn main() {
    let x = 20;

    if x > 10{
        println!("x = {}", x);
        println!("xの値は10より大きいです");
    }

    if (x + 30) >= 35 {
        println!("x = {}", x);
        println!("x + 30の値は35より大きいです");


    if true {
        println!("条件が真なので必ず実行されます");
    }
    }
}