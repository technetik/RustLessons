fn main() {
    
    //Затенения  (shadowing) переменных
    //по сути затенение переменной(сокрытие переменной), это ее новое объявление
    let test = "11.11";
    let test = 11.11;
    let mut test = 11;
    test = 111;

    println!("Hello, world! {}", test);


}
