/*
fn main() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;
    
    // 值得地址是什么？引用的地址又是什么？
    println!("addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}", &data, data1, &&data, &data);
    println!("sum of data1: {}", sum(data1));

    // 堆上数据的地址是什么？
    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]  
    );
}

fn sum(data: &Vec<u32>) -> u32 {
    // 值的地址会改变么？引用的地址会改变么？
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(1, |a, x| a + x)
}
*/

/* 生命周期更长的main()函数变量r，引用了生命周期更短的local_ref()函数里的局部变量，编译不被允许
fn main() {
    let r = local_ref();
    println!("r: {:p}", r);
} 

fn local_ref<'a>() -> &'a i32 {
    let a = 42;
    &a
}
*/

/*
fn main() {
    let mut data: Vec<&u32> = Vec::new();
    let v = 42;
    data.push(&v);
    println!("data: {:?}", data);
}
*/

/*
fn main() {
    let mut arr = vec![1, 2, 3];
    // cache the last item
    let last = arr.last();
    arr.push(4);
    // consume previously stored last item
    println!("last: {:?}", last);
}

fn main() {
    let mut arr = vec![1, 2, 3];
    // cache the last item
    let last = arr.last();
    // consume previously stored last item
    println!("last: {:?}", last);
    arr.push(4);
}

*/