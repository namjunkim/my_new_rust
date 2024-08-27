use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    //practice_input();
    study_variable();
}


fn study_variable() {
    let x = 5;
    println!("x : {}", x);
    //x = 6; // 기본적으로 variable은 immutable

    let mut y = 9;
    println!("y : {}", y);
    y = 6; // 기본적으로 variable을 자체를 mut(mutable)로 선언해서 변경 가능
    println!("y : {}", y);

    const MIN_NUM:u32 = 0; // 상수는 const로 나타낸다.


    // Compound Type
    /// 컴파운드 타입(compound type)은 하나의 타입으로 여러 개의 값을 그룹화한 타입
    /// 러스트는 기본적으로 튜플(tuple)과 배열(array) 두 가지 컴파운드 타입을 지원
    ///
    /// 튜플은 서로 다른 타입의 여러 값을 하나의 컴파운트 타입으로 그룹화하기에 적합한 타입
    ///
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("y의 값: {}" ,y);

    let five_hundred = tup.0;
    println!("five_hundred의 값: {}" ,five_hundred);

    let six_point_four = tup.1;
    println!("six_point_four의 값: {}" ,six_point_four);


    /// 배열(array)은 튜플과 달리 같은 타입으로 이루어 져야하며
    /// 다른 언어의 배열과는 달리 고정된 길이다. 배열에 저장할 값은 대괄호(square bracket,[]) 안에 쉼표로 구분해서 나열
    let a = [1, 2, 3, 4, 5];

    //i32 타입으로 5개 원소가 존재한다고 지정
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    //5개의 원소를 3으로 초기화
    let b = [3; 5];


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
