use std::io;

fn main() {

    practice_input();
}

fn practice_input() {

    println!("원하는 숫자를 입력해 주세요.");

    let mut guess = String::new();
    // mut 키워드는 값을 변경할수 있게 해줌.(입력이 가능하게)

    io::stdin().read_line(&mut guess)// 표기에 유의! &guess가 아닌 &mut guess로 표현해야함.
        .expect("입력 오류!!");

    println!("당신이 입력한 값은 {}", guess);

}


// rust는 snake_case를 더 선호하는 듯?
fn practice_variable() {
    println!("변수 연습하기");
    let name = "홍길동";
    // let으로 변수 표현이 가능합니다.
    let age = 40;
    // 숫자 역시 가능합니다.

    println!("당신의 이름은 => {} 입니다.", name);
    println!("당신의 나이는 => {} 입니다.", age);

    let your_name = "Mike";
    let my_name = "Namjun";

    println!("Your name is {}, and my name is {}", your_name, my_name);
    // 이렇게 연속으로 가능합니다.

}
