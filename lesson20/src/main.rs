fn main() {
    // Пример 1
    // Владение: Строка action_hero остается единственным владельцем данных в куче (heap).
    let action_hero: String = String::from("Vasya Pupkin");
    // заимствуем фрагмент строки, берем безопасный срез строки (ссылка на строку) с 0 по 4 байт.
    let first_name: &str = &action_hero[0..5];
    println!("{first_name}");
    // заимствуем фрагмент строки, берем безопасный срез строки (ссылка на строку)с 6 по 11 байт..
    let last_name: &str = &action_hero[6..12] ;
    println!("{last_name}");

    // Пример 2
    // ссылка на Текст хранится прямо в скомпилированном бинарнике
    // action_hero это просто указатель на адрес памяти, где хранится эта строка
    // это срез строки и она не является владельцем.
    let action_hero = "Vasya Pupkin";   
    // срез строки &str, он ссылается на часть строки в бинарнике
    let first_name: &str = &action_hero[0..5];
    println!("{first_name}");   
    // срез строки &str, он ссылается на часть строки в бинарнике
    let last_name: &str = &action_hero[6..12] ;
    println!("{last_name}");

     // Пример 3
     // пример с альтернативным синтаксисом
     let action_hero1 = "Vasya Pupkin";
     //срез строки с 0 по 4 байт.
     let test_name: &str = &action_hero1[..5];
     println!("{test_name}");
     // срез строки с 6 по 1 байт.
     let test_name: &str = &action_hero1[6..];
     println!("{test_name}");
    // срез строки, вся строка.
     let test_name: &str = &action_hero1[..];
     println!("{test_name}");

}
