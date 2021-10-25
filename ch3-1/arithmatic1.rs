fn main (){
    //変数と変数を定義して表示
    let x = 10;
    let y = 3;
    let mut z;
    // 変化可用性のmut

    // 加算
    z = x + y;
    println!("{} + {} = {}", x, y, z);

    // 減算
    z = x - y;
    println!("{} - {} = {}" , x, y, z);

    //乗算
    z = x * y;
    println!("{} * {} = {}", x, y, z);

    //剰算

    z = x/y;
    println!("{} / {} = {}", x, y, z);

    //剰余算
    z = x % y;
    println!("{} % {} = {}", x, y, z);

}