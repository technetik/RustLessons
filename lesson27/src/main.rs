#[derive(Debug)]
struct Users {
    name: String,
    age: u32,
    year: u32,
    height: u32,
}

// Варианты создания метода альтернативный синтаксис
// fn print_user(self)
// fn print_user(self: Users) 
// fn print_user(self: Self)

impl Users{
    /*
    Чтобы объект можно было использовать многократно, в аргументах методов 
    обычно используют ссылки: fn print_user(&self) и fn year_plus(&mut self).
    Варианты, как и в функциях.
    // self
    // mut self
    // &self
    // &mut
     */
    // Пример 1
    fn print_user(self){
        println!("Имя: {}", self.name);
        println!("Возраст: {}", self.age);
        println!("Год: {}", self.year);
        println!("Рост: {}", self.height);
    }
    fn year_plus(mut self){
        self.year = self.year + 1;
        println!("{:#?}", self);
    }
    // Передача дополнтельного параметра в метод
    // Передаем еще один экземпляр стуктуры
    // Пример 2
    fn is_height_than(&self, other: &Self) -> bool{
        self.height > other.height
    }
}

fn main() {
    // Пример 1
    let user1 = Users{
        name: String::from("Вася"),
        age: 32,
        year: 1991,
        height: 178,
    };
    // Объект user1 "умер" здесь, так как print_user забрал владение.
    user1.print_user();

    let user2 = Users{
        name: String::from("Петя"),
        age: 29,
        year: 1991,
        height: 185,
    };
    // Объект user2 "умер" здесь, так как year_plus забрал владение.
    user2.year_plus();

    // Пример 2
        let user3 = Users{
        name: String::from("Андрей"),
        age: 32,
        year: 1991,
        height: 178,
    };
        let user4 = Users{
        name: String::from("Саша"),
        age: 29,
        year: 1990,
        height: 185,
    };
    
    if user3.is_height_than(&user4){
        println!("{} выше", user3.name);
    } else {
        println!("{} выше", user4.name);
    }

}