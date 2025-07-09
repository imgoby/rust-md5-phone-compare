use crypto::digest::Digest;
use crypto::md5::Md5;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::error::Error;
use std::io::{BufWriter, Write};
use std::env;
use std::collections::HashMap;



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len()<2{
        println!("请输入参数;");
        return;
    }
    let action=args.get(1).unwrap();

    if action=="md5"{
        if args.len()<4{
            println!("如：md5 in.txt out.txt");
          return;
        }

        let in_path=args.get(2).unwrap();
        let out_path=args.get(3).unwrap();

        println!("生成md5库文件,输入文件：{}，输出文件：{}",in_path,out_path);
        create_md5(in_path,out_path);
    }else if action=="cmp"{
        if args.len()<5{
            println!("如：cmp md5.txt in.txt out.txt");
            return;
        }

        let md5_path=args.get(2).unwrap();
        let in_path=args.get(3).unwrap();
        let out_path=args.get(4).unwrap();

        println!("比较文件,本地md5库文件：{}，输入文件：{}，输出文件：{}",md5_path,in_path,out_path);
        cmp_md5(md5_path,in_path,out_path);
    }else{
        println!("不支持的命令:{}，仅支持：md5、cmp",action);
    }
}



fn create_md5(in_path:&String,out_path:&String){
    let in_file = match File::open(in_path){
        Ok(f)=>f,
        Err(e)=>{
            println!("找不到文件：{}",in_path);
            return;
        }
    };

    let br = BufReader::new(in_file);
 
    let out_file = File::create(out_path).unwrap();
    let mut bw = BufWriter::new(out_file);
    for line in br.lines() {
        let line = line.unwrap().trim().to_string();
        // println!("{}", &line);
        if line.is_empty(){
            continue;
        }

        let mut md5 = Md5::new();
        md5.input_str(&line);
        let md5_str = md5.result_str();
        // println!("md5_str:{}",md5_str);

        let new_line=format!("{}\n",md5_str);

        bw.write_all(new_line.as_bytes()).unwrap();
    }
    bw.flush().unwrap();
}


/**
 * 比较md5，把存着的保存下来
 */
fn cmp_md5(md5_path:&String,in_path:&String,out_path:&String){
    let md5_file = match File::open(md5_path){
        Ok(f)=>f,
        Err(e)=>{
            println!("找不到文件：{}",md5_path);
            return;
        }
    };

    let in_file = match File::open(in_path){
        Ok(f)=>f,
        Err(e)=>{
            println!("找不到文件：{}",in_path);
            return;
        }
    };

    let mut md5_phones = HashMap::new();
    let md5_br = BufReader::new(md5_file);
    for line in md5_br.lines() {
        let line = line.unwrap().trim().to_string();
        // println!("md5_file:{}", &line);
        if line.is_empty(){
            continue;
        }

        md5_phones.insert(line, 1);
    }

    let br = BufReader::new(in_file);
    let out_file = File::create(out_path).unwrap();
    let mut bw = BufWriter::new(out_file);
    for line in br.lines() {
        let line = line.unwrap().trim().to_string();
        // println!("in:{}", &line);
        if line.is_empty(){
            continue;
        }

        if let Some(v) = md5_phones.get(&line) {
            let new_line=format!("{}\n",line);
            bw.write_all(new_line.as_bytes()).unwrap();
        }else{
            // print!("not exist:{}",&line);
        }
    }
    bw.flush().unwrap();
}