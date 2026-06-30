fn main() {
    // Пример 1
    let registrations: (bool, bool, bool) = (true, false, true);
    // копирует значение, оставляя оригинал доступным.
    let first: bool = registrations.0;
    println! ("{first} and {registrations:?}");

    // Пример 2
    // В Rust тип String не реализует трейт Copy
    let languages: (String, String) = (String::from("Rust"), String::from("Java"));
    // Можем создать заимствование (ссылку). Кортеж остается владельцем.
    let first1: &String = &languages.0;
    // Можем создать явную копию строки. Кортеж остается владельцем.
    let first2: String = languages.0.clone();
    println!("{first1} , {first2} and {languages:?}");
    
}
