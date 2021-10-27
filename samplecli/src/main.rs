use clap::{App, Arg};

fn main() {
    // インスタンス作成
    let matches = App::new("My Rpn Programming")
    // バージョン/作者/アプリケーン概要/コマンドライン引数の概要
    .version("1.0.0")
    .author("Your name")
    .about("Super awsome sample RPN caluculator")
    .arg(
        Arg::new("formula_file")
        // formula_fileは数式を記載するファイル
        .about("Formulas written iin in RPN")
        .value_name("FILE")
        .index(1)
        .required(false),
    )
    .arg(
        Arg::new("verbose")
        .about("Sets the level of verbosity")
        .short('V')
        .long("verbose")
        .required(false),
    )
    .get_matches();
    // get_matchesで実行時に指定されたコマンドライン引数と照合している。その結果をmatch以降で取り出している。
match matches.value_of("formula_file") {
    Some(file) => println!("File specified : {}", file),
    None => println!("No file specified.!"),
}
let verbose = matches.is_present("verbose");
println!("Is verbose specifiesd?: {}", verbose);
}

