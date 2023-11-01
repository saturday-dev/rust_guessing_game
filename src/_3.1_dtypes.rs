

fn numbers_ex(){
    let x = 5;
    let x = -5;
    let x = x+1;

    let x = x*2;

    let x1 = 2.0;

    let sum = 6 + 10;

    let difference = 95.2 - 4.3; // f64

    let y: f32 = 3.0; //f32

    println!("The value of x is : {}", x);


    // let mut spaces = "    ";
    // spaces: = spaces.len().parse();

    // println!("space len is : {}", spaeces)
}

fn boo_ex(){
    let t = true;

    let f: bool = false; // with explicit type annotation
}

fn str_ex(){
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

}

fn tuple_ex(){
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0; // tuple type에서 index를 "." 뒤에 넣으면 해당 값을 반환한다.

    let six_dot_four = x.1;

    println!("{} {}", five_hundred, six_dot_four);
}

fn list_ex(){
    let list = [1,2,3,4,5,6,];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("{}", list[1]);

    // let element_with_no_index = months[13]; // 13 index가 존재하지 않아서 에러 발생함.

    // println!("{}", element_with_no_index);


}

fn another_fn(x: i32, y: i32){
    println!("Another Function!");
    println!("The value of x is : {}", x);
    println!("The value of y is : {}", y);
}

fn fn_ex(){
    println!("Hello world!");

    another_fn(5, 6);
}


fn return_ex(){
    let x = 5;
    let y = {
        let x = 3;
        x+1 // semi colon을 넣으면 반환식이 아니게됨.
    };
    println!("The value of x is : {}", x);
    println!("The value of y is : {}", y);
}

fn five() -> i32 {
    5
}
fn main(){
    let x = five();
    println!("The value of x is : {}", x);
}