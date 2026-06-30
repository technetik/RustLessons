fn main() {
    // Пример 1
    let registrations = [true, false, true];
    // копирует значение, оставляя оригинал доступным.
    let first: bool = registrations[0];
    println! ("{first} and {registrations:?}");

    // Пример 2
    // В Rust тип String не реализует трейт Copy
    let languages = [String::from("Rust"), String::from("Java")];
    // Можем создать заимствование (ссылку). Массив остается владельцем.
    let first1: &String = &languages[0];
    // Можем создать явную копию строки. Массив остается владельцем.
    let first2: String = languages[0].clone();
    println!("{first1} , {first2} and {languages:?}");
}
