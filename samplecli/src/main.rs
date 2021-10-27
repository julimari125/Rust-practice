use std::env;
// std::env::varで環境変数の値を取得

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
}
