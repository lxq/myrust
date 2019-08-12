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
    let mut str1 =  String::from("你好，");
    println!("{}", str1);
    // 可变引用调用
    push_str(&mut str1, &String::from("rust!"));
    println!("{}, len = {}", str1, calc_len(&str1));

    test_variable();

    guess_number();
}

// 可变引用：&mut 类型
fn push_str(src: &mut String, sub: &String) {
    src.push_str(sub);
}

// 引用参数
fn calc_len(s: &String) -> usize {
    // String长度是按字节，不是字符数
    s.len()
    // 表达式有返回值
}

// 变量练习
fn test_variable() {
    let b = true;
    let c = 'b';
    let s = "未高科技";
    let mut n = 123;
    println!("{}, {}, {}, {}",b, c, s, n);
    //b = false;

    n = 345;
    println!("{}", n);

    // tuple
    let t:(i32, bool, f64) = (123, true, 3.14);
    println!("tuple = ({}, {}, {})", t.0, t.1, t.2);
    // pattern match
    let (x,y,z) = t;
    println!("(t.x={}, t.y={}, t.z={})", x, y, z);

    // array
    let arr = [1,2,3,4];
    // 索引从0开始，越界会panic
    for e in arr.iter() {
        println!("{}", e);
    }
    // range revert
    for e in (5..8).rev() {
        println!("{}", e);
    }

    // 下面调用会发生所有权转移，是不允许的。
    // takes_ownership(s);
    makes_copy(n);
}

// 猜数字
fn guess_number() {
    println!("输入你要猜的数字");

    // 随机数
    let num = rand::thread_rng().gen_range(1, 101);
    println!("随机数是：{}", num);

    loop {
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
        let guess: i32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("请输入整数!");
                continue;
            }
        };

        //“{}”点位符，
        println!("你猜的数字是：{}", guess);

        // match 是一个表达式，{}后没有；
        // match后面的分布类似箭头函数
        // std::cmp::Ordering 常用于结果比较
        match guess.cmp(&num) {
            // 分支间使用逗号分隔
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("猜对了！");
                break; // for loop
            }
        }
    }
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(x: i32) {
    println!("{}", x);
}
