// クレート: コンパイル単位。本ファイルは実行可能プログラム。他にはライブラリクレートもある。
use std::io;
// prelude: 標準ライブラリの一部はuseせずに使うことができる

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // letは不変。mutで可変になる。
    let mut guess = String::new();

    // &は参照を示す
    // expectはread_lineの結果がErrの時の処理。ここではクラッシュさせる
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // プレースホルダ {}
    println!("You guessed: {}", guess);
}
