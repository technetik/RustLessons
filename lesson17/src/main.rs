fn main() {
    /*
        s: String  Не изменяемое передача владения
        mut s: String Изменяемое передача владения
        s: &String  Не изменяемое заимствование, ссылка на строку (адрес в памяти)
        s: &mut String  Изменяемое заимствование, ссылка на строку (адрес в памяти)
     */
    // Не изменяемое передача владения
    let txt1 = String::from("Привет 1");
    up_text_1(txt1);
    // переменная txt1 больше не сущест

    // Изменяемое передача владения
    let txt2 = String::from("Привет 2");
    up_text_2(txt2);
    // переменная txt2 больше не сущест

    // не изменяемое заимствование (по ссылке)
    let txt3 = String::from("Привет 3");
    up_text_3(&txt3);
    println!("{txt3}");

    // изменяемое заимствование (по ссылке)
    let mut txt4 = String::from("Привет 4");
    up_text_4(&mut txt4);
    println!("{txt4}");
}

// s: String — неизменяемая передача владения
fn up_text_1 (s: String) {
    println!("Получено: {}", s);   
}

// mut s: String — изменяемая передача владения
fn up_text_2 (mut s: String) {
    s.push_str(", плюс 1!");
    println!("Получено: {}", s);
}

//s: &String — неизменяемое заимствование
fn up_text_3 (s: &String) {
    println!("Получено: {}", s);      
}

// s: &mut String — изменяемое заимствование
fn up_text_4 (s: &mut String) {
    s.push_str(", плюс 2!");
    println!("Получено: {}", s);    
}