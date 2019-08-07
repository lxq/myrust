/**
 * My learning of rust.
 * @author Allen Lin
 * @email xqlin@qq.com
 * @date 2019/7/27
 */

// 使用外部依赖，这样可以使用rand::访问
extern crate rand;

use std::io;
use std::cmp::Ordering;

// 使用Rng trait
use rand::Rng;

fn main() {
    println!("你好, Rust!");
    let b = true;
    let c = 'b';
    let s = "未高科技";
    let mut n = 123;
    println!("{}, {}, {}, {}",b, c, s, n);
    //b = false;

    n = 345;
    println!("{}", n);

    guess_number();
}

// 猜数字
fn guess_number() {
    println!("输入你要猜的数字");

    // 随机数
    let num = rand::thread_rng().gen_range(1, 101);
    println!("随机数是：{}", num);

    // String标准库字符串类型
    // ::new()叫关联函数，与类型相关
    let mut guess = String::new();
    // io::stdin()返回io::Stdin实例，代表标准输入终端的句柄
    // Stdin::read_line() 获取标准输入的字符串，返回io::Result类型——校兴（Ok/Err）
    // & 表示引用，不需要复制
    io::stdin().read_line(&mut guess)
        .expect("读取数字错误。");
    
    // 同名变量会覆盖(shadow)前面
    // 字符串parse()转换时，被赋值变量必须有明确类型
    let guess: i32 = guess.trim().parse()
        .expect("请输入整数!");

    //“{}”点位符，
    println!("你猜的数字是：{}", guess);

    // match 是一个表达式，{}后没有；
    // match后面的分布类似箭头函数
    // std::cmp::Ordering 常用于结果比较
    match guess.cmp(&num) {
        Ordering::Less => println!("小了"),
        Ordering::Greater => println!("大了"),
        Ordering::Equal => println!("猜对了！")
    }
}