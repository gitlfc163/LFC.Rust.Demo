mod models;
mod quux;
mod vecdeque_demo;
pub mod vector_demo;

fn main() {
    quux::foo::bar();
    quux::foo::baz();

    variables_demo();

    const_demo();

    shadowing_demo();

    tup_demo();

    array_demo();

    reference_demo();

    reference_mut_demo();

    string_slice_demo();

    create_user();

    crate::vector_demo::vector_demo::for_mutvector_demo();
}

//结构体
fn create_user() {
    println!();
    let user1 = models::user::User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("The user1 is '{}' ", user1.email);
}

fn variables_demo() {
    println!();
    println!("variables_demo");
    let x = 5; //不可变变量 x
    println!("This x is {x}");
    //x = 6; //对不可变变量 x重新赋值
    //println!("This is {x}");
}

fn const_demo() {
    println!();
    println!("const_demo");
    const VERSION: &'static str = "0.1.0";
    println!("VERSION is:{}", VERSION);
}

fn shadowing_demo() {
    println!();
    //不可变变量 x
    let x = 5;
    println!("The value of x is: {x}");

    //创建了一个新变量 x，获取初始值并加 1,隐藏了第一个x
    let x = x + 1;
    println!("The value of x + 1 is: {x}");

    {
        let x = x * 2;
        println!("The value of x * 2 is: {x}");
    }

    println!("The value of x is: {x}");
}

fn tup_demo() {
    println!();
    let tup: (i32, i32, f32) = (11, 100, 1000.1);
    println!("The value of x is: {}", tup.0);

    //解构
    println!();
    let tup2 = (500, 6.4, 1);
    let (x, y, z) = tup2;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    //使用点号（.）后跟值的索引来直接访问
    println!();
    let tup3: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup3.0 is: {}", tup3.0);
    println!("The value of tup3.1 is: {}", tup3.1);
    println!("The value of tup3.2 is: {}", tup3.2);
}

fn array_demo() {
    println!();

    //我们将数组的值写成在方括号内，用逗号分隔：
    let arr = [1, 2, 3, 4, 24, 2, 14, 22];
    println!("The value of arr first is: {}", arr[0]);

    //方括号中指定初始值加分号再加元素个数[初始值; 元素个数]
    println!();
    let a = [3; 5];
    for x in a {
        print!("{x} ");
    }
    println!();

    // 通过引用进行迭代
    println!();
    let array: [i32; 8] = arr;
    for item in array.iter().enumerate() {
        let (i, x): (usize, &i32) = item;
        println!("array[{i}] = {x}");
    }
}

//引用
fn reference_demo() {
    println!();
    let str = String::from("hello");
    let len = calculate_length(&str);

    println!("The length of '{}' is {}.", str, len);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}

//可变引用
fn reference_mut_demo() {
    println!();
    let mut str = String::from("hello");
    change(&mut str);
    println!("The str is '{}' ", str);
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//字符串 slice
fn string_slice_demo() {
    println!();
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("The hello is '{}' ", hello);
    println!("The world is '{}' ", world);
}
