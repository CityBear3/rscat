use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    //コマンドライン引数を取得する
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    let len = args.len();

    while i != len {
        //failenameを取得する
        let filename = &args[i];
        //println!("In file {}", filename);

        //ファイルへの可変なハンドルを得る
        let mut f = match File::open(filename) {
            Ok(f) => f,
            Err(e) => {
                println!("error:{}", e);
                return;
            }
        };
        let mut contents = String::new();
        //ファイルハンドルを引数として可変参照に渡す
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        println!("{}", contents);

        i = i + 1;
    }
}
