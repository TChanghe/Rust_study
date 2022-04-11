fn main() {
    let logic:bool = true;
    println!("1 = 1 is {}", logic);  //变量绑定时用：标注类型

    let a_float: f32=1.0;
    let an_integer=5i32; //两种标注方法

    let default_float = 1.0;
    let default_integer = 5;  //未标注则使用默认的 ，前者为f64，后者i32

    let int8: i8 = 127;  //8位有符号整型，【-128，127】
    let uint8: u8 = 255;  //8位无符号整型，【0，255】

    let mut infered_type = 12;
    infered_type = 4294967296i64;  // let mut 定义的是可以变的变量

    
    //println!("lu32 - li32 = {}", lu32 - li32);  不能运行，因为不同类型不能运算

    let en: char = 'x';
    let zh: char = '中';  

    return();
    

}