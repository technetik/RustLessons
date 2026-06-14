fn main() {
    // Заимствование и ссылки, разыменование

    let s1 = String::from("привет"); 
    let r1 = &s1; // Создаем неизменяемую ссылку 
    let r2 = &s1; // Создаем вторую неизменяемую ссылку (разрешено) 
    println!("{}, {}", *r1, *r2); // разыменование явное
    println!("{}, {}", r1, r2);  // разыменовании не явное
    println!("{}", s1); // s1 не уничтожена, к ней можно обращаться напрямую 

    println!("{:p}", r1);  // выведет адрес памяти на котрый ссылается ссылка


    // Так как тип bool реализует трейт Copy, компилятор Rust не перемещает владение,
    let is_concert: bool = true;
    let is_event: bool = is_concert; // переменная будет скопирована
    println!("{is_concert} {is_event}"); 

    // Поскольку тип &str реализует трейт Copy, компилятор просто дублирует эти 16 байт (адрес и длину) 
    // из переменной sushi в переменную dinner
    let sushi: &str = "Salmon";
    let dinner: &str = sushi; // переменная будет скопирована
    println! ("{sushi} {dinner}");

    // тип String управляет динамической памятью в куче (Heap) и не реализует трейт Copy
    // будет передано владение, переменная sushi будет не доступна 
    let sushi: String = String:: from ("Salmon");
    let dinner: String = sushi;
    println! ("{dinner}");
    //println! ("{sushi}");

    // Возвращаем тип String из функции eat_meal
    // если не возвращать meal , будет не доступна
    let fish: String = eat_meal(dinner);
    println!("{fish}");

}

fn eat_meal(mut meal: String) -> String {
    meal.clear();
    meal
}