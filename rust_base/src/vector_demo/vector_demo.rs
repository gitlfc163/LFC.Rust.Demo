pub fn getvector_demo1() {
    println!();
    let v = vec![1, 2, 3, 4, 5];
    //使用 & 和 [] 返回一个引用
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    //使用 get 方法以索引作为参数来返回一个 Option<&T>
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
pub fn for_vector_demo1() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("for_vector_demo1 i is {}", i);
    }
}

pub fn for_mutvector_demo() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        //原来数组对应的项的值都加上50
        *i += 50;
        //println!("for_mutvector_demo i is {}", *i);
    }

    for i in &v {
        //原来的值都加上50
        println!("for_mutvector_demo i is {}", i);
    }
}
