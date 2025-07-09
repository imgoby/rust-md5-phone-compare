use crypto::digest::Digest;
use crypto::md5::Md5;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::error::Error;
use std::io::{BufWriter, Write};

fn main() {
    let in_file = match File::open("原始号码.txt"){
        Ok(f)=>f,
        Err(e)=>{
            println!("找不到文件：原始号码.txt");
            return;
        }
    };

    let br = BufReader::new(in_file);
 
    let out_file = File::create("md5号码.txt").unwrap();
    let mut bw = BufWriter::new(out_file);
    for line in br.lines() {
        let line = line.unwrap().trim().to_string();
        println!("{}", &line);
        if line.is_empty(){
            continue;
        }

        let mut md5 = Md5::new();
        md5.input_str(&line);
        let md5_str = md5.result_str();
        println!("md5_str:{}",md5_str);

        let new_line=format!("{}\n",md5_str);

        bw.write_all(new_line.as_bytes()).unwrap();
    }
    bw.flush().unwrap();

}
