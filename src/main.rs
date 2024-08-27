use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    practice_input();
}

fn practice_input() {

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //use rand::Rng Rng 트레이트(trait)는 난수 생성기에 구현된 메서드를 정의한다.
    //SampleRange<T>는 ..=라는 것으로 표현되어 범위를 지정할때에 쓰임.
    //트레이트(trait)는 Java에서 Interface와 비슷한 존재
    //rand::thread_rng 함수는 우리가 사용할 난수 생성기를 리턴하고, 생성기의 gen_range 함수를 호출한다.
    println!("비밀 값은 => {}", secret_number);

    println!("원하는 숫자를 입력해 주세요.");


    // 루프문 실행
    loop {

        let mut guess = String::new();
        // mut 키워드는 값을 변경할수 있게 해줌.(입력이 가능하게)



        io::stdin().read_line(&mut guess)// 표기에 유의! &guess가 아닌 &mut guess로 표현해야함.
            .expect("입력 오류!!");

        println!("당신이 입력한 값은 {}", guess);

        // guess가 String값이기 때문에 i32으로 이해, u32 로 파싱
        //let guess: u32 = guess.trim().parse()
        //    .expect("입력한 값이 올바른 숫자가 아닙니다.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // Ok는 u32로 파싱에 매칭에 성공할 경우 그대로
            Err(_) => continue, // Err가 나면 컨틴뉴
        };

        // 값을 비교하는 부분
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("입력한 숫자가 작습니다!"),
            Ordering::Greater => println!("입력한 숫자가 큽니다!"),
            Ordering::Equal => {
                println!("정답!");
                break;
            },
        }

        //let guess: u32 = match guess.trim().parse() {
        //    Ok(num) => num,
        //    Err(_) => continue,
        //};
    }// break 종료

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
