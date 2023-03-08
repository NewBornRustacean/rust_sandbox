fn main() {
    // statement는 return 값이 없음.
    // 그래서 let x = y=6 은 let x= nothing 과 같다.
    // println! 안에서 {x:?} 는 debug trait 형식으로 출력하기 위함.
    let y;
    let x = 1;
    let x = y = 6;
    println!("x: {x:?}, y: {y}");
}
