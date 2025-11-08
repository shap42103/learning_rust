// クレート: コンパイル単位。本ファイルは実行可能プログラム。他にはライブラリクレートもある。
use rand::Rng;
use std::{cmp::Ordering, io, os::fd::OwnedFd};

// prelude: 標準ライブラリの一部はuseせずに使うことができる

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        // letは不変。mutで可変になる。
        let mut guess = String::new();

        // &は参照を示す
        // expectはread_lineの結果がErrの時の処理。ここではクラッシュさせる
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 変数の再定義は許容される
        // u32: 符号なし32ビット整数. parseするためには型注釈が必須
        // parseはResultを返すのでmatchで処理する
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        // プレースホルダ {}
        println!("You guessed: {}", guess);

        // cmp: 比較
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
