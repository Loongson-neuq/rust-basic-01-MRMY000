use std::cmp::Ordering;
use std::io;
use rand::Rng;
use  rand::thread_rng;

fn main() {
    

    let sec_num = thread_rng().gen_range(1..=100);//generate a num
    

    loop {

        println!("请输入一个数：");

        let mut  guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("读取失败！");//标准输入输出读取

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };//异常处理

        match guess.cmp(&sec_num) {
            Ordering::Less => println!("数字偏小！"),
            Ordering::Greater => println!("数字偏大！"),
            Ordering::Equal => {
                println!("猜对了！");
                break;
            }
        }//正误判断逻辑
    }
}
