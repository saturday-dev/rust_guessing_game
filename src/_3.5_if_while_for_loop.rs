fn if_ex(){
    let number = 3;

    // 조건문 1
    if number <5 {
        println!("condition was true");

    }else{
        println!("condition was false");
    }
    // 조건문 2
    if number != 0{
        println!("number was something other than zero")
    }

    // 조건문 3
    let condition = true;
    let number = if condition {
        5
    }else{
        6 //"six" number변수에 int와 str이 동시에 들어갈 수 없음
    };
    println!("The value of number is : {}",number);



}


fn while_ex1(){
    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5 {
        println!("the value is : {}", a[index]);

        index +=1;
    }
}

fn for_loop_ex(){
    let a = [10, 20, 30, 40, 50];

    for element in a.iter(){
        println!("the value is : {}", element);
    }
}

fn main(){
    for number in (1..=4).rev(){ // .rev()는 거꾸로 순환하는 명령어.
        println!("{}!", number);

    }
    println!("LIFTOFF!!!!");
}