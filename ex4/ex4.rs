fn main() {
    println!("hello world");
    println!("{} is from {}", "Brad", "Tom");   //使用{}进行格式化输出
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Tom", "Mass");  //可以给参数编号
    println!("{name} likes to play {activity}", name="Tom", activity="code" );  //可以给参数命名
    println!("10 + 10 = {}", 10+10);  //参数可以是表达式

    let name = "Tom";
    let activity = "code";
    println!("{name} likes to play {activity}");  //参数可以是基本类型变量

    const ID: i32=001;
    println!("ID: {}",ID);  //在变量名后面添加 :类型名 来指定类型，打印常量，
                            //整形用i/u + 数字(8/16/32/64/128) 表示 如u8,i64，普通整型默认为 "i32"类型，普通浮点数默认为"f64"类型

    let person : (&str, &str, i8) = ("brad", "Mass", 37);
    println!("{} like {} for {} years", person.0,person.1,person.2);  //元组是不同类型数据的集合，最多拥有十二个数据。

    println!("Bjinzhi:{:b}, Hex:{:x}, Octal{:o}", 10, 10, 10); // 格式化输出

    println!("{:?}",(12,true,"Hello"));
    //参数列表可以进行简单的数学运算，这一点与C/C++等多数语言相同



}