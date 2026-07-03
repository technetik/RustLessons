/*
    Мы можем передать структуру в функцию четырьмя способами
 */
 struct Coffee {
    price: f64, 
    name: String, 
    is_hot: bool,
}

fn main() {

    // Пример 1
    let mocha1 = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: false,  
    };

    drink_coffee1(mocha1);

    // Пример 2
    let mocha2 = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: false,  
    };

    drink_coffee2(mocha2);

    // Пример 3
    let mocha3 = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: false,  
    };

    drink_coffee3(&mocha3);  

    // Пример 4
    let mut mocha4 = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: false,  
    };

    drink_coffee4(&mut mocha4);
}

// принимаем как параметр экземпляр структуры Coffee не изменяемый
// Происходит передача владения, после выполнения, экземпляра структуры mocha больше не существует
fn drink_coffee1(coffee: Coffee) {
    println! ("Кофе {}", coffee.name);
}

// принимаем как параметр экземпляр структуры Coffee  изменяемый
// Происходит передача владения, после выполнения, экземпляра структуры mocha больше не существует
fn drink_coffee2(mut coffee: Coffee) {
    coffee.is_hot = true;
    println! ("Кофе {} / {}", coffee.name, coffee.is_hot);
    
}

// принимаем как параметр ссылку на экземпляр структуры Coffee не изменяемый
// передача владения не происходит
fn drink_coffee3(coffee: &Coffee) {
    println! ("Кофе {}", coffee.name);
}

// принимаем как параметр ссылку на экземпляр структуры Coffee не изменяемый
// передача владения не происходит
fn drink_coffee4(coffee: &mut Coffee) {
    coffee.is_hot = true;
    println! ("Кофе {} / {}", coffee.name, coffee.is_hot);
}