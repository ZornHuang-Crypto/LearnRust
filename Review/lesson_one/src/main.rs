
/********测试 html to markdown*******/
/* 
use std::fs;

fn main() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");;
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}", output);
}
*/

/********测试函数语法********/
/* 
fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

fn main() {
    println!("apply square: {}", apply(2, square));
    println!("applye cube: {}", apply(2, cube));
}
*/

/********测试返回值，如果没有";" 直接返回该值，否则返回unit，即"()"********/
/* 
fn pi() -> f64 {
    3.1415926
}

fn not_pi() {
    3.1415926;
}

fn main() {
    let is_pi = pi();
    let is_unit1 = not_pi();
    let is_unit2 = {
        pi();
    };

    println!("is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}", is_pi, is_unit1, is_unit2);
}
*/


/********控制流程********/
/*
fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2;
    
    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        
        println!("next val is {}", b);
        
        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);
    
    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        
        println!("next val is {}", b);
    }
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);
    
    for _ in 2..n {
        let c = a + b;
        a = b;
        b = c;
        println!("next val is {}", b);
    }
}

fn main() {
    let n = 10;
    // fib_loop(n);
    fib_for(n);
}
*/

/* assert_eq
fn main() {
    let arr = [1, 2, 3];
    assert_eq!(arr[..], [1, 2, 3]);   
    assert_eq!(arr[0..1], [1]);   
}
*/


/*
/********测试枚举、标签联合********/
#[derive(Debug)]
enum Gender {
    Female = 1,
    Male = 2
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId
}

// 定义聊天室中可能发生的事件
#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String))
}

fn main() {
    
    let alice = User { id: UserId(1), name: "Alice".into(), gender: Gender::Female };
    let bob = User { id: UserId(2), name: "Bob".into(), gender: Gender::Male };
    
    let topic = Topic{ id: TopicId(1), name: "rust".into(), owner: UserId(1) };
    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "Hello world!".into()));
    
    //println!("event1: {:?}, event2: {:?}, event3: {:?}", event1, event2, event3);
    
    process_message(&event1);
    process_message(&event2);
    process_message(&event3);
}

fn process_event(event: &Event) {
    match event {
        Event::Join((uid, _)) => println!("user {:?} joined", uid),
        Event::Leave((uid, tid)) => println!("user {:?} left {:?}", uid, tid),
        Event::Message((_, _, msg)) => println!("broadcast: {}", msg)
    }
}

fn process_message(event: &Event) {
    if let Event::Message((_, _, msg)) = event {
        println!("broadcast: {}", msg);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/

fn main() {
    for arg in std::env::args() {
        println!("CR: {}", arg);
    }
}
