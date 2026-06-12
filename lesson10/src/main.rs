fn main() {
    // Strings &str and Raw Strings &str
    let _path ="C:\\Users\\name\\documents"; // экранируем слэши или пример ниже 
    let path = r"C:\Users\name\documents"; // слэши остаются обычными слэшами
    println!("{path}");

    // Тип String
    let text1 = String::new();
    let text2 = String::from("Просто текст");

    let mut text3 = String::from("Просто текст");
    println!("{text3}");
    text3.push_str(" еще текст");
    println!("{text3}");
}
