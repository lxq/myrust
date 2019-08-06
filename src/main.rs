/**
 * My learning of rust.
 * @author Allen Lin
 * @email xqlin@qq.com
 * @date 2019/7/27
 */


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

    // String标准库字符串类型
    // ::new()叫关联函数，与类型相关
    let mut guess = String::new();
    // io::stdin()返回io::Stdin实例，代表标准输入终端的句柄
    // Stdin::read_line() 获取标准输入的字符串，返回io::Result类型——校兴（Ok/Err）
    // & 表示引用，不需要复制
    std::io::stdin().read_line(&mut guess)
        .expect("读取数字错误。");
    
    //“{}”点位符，
    println!("你猜的数字是：{}", guess);
}