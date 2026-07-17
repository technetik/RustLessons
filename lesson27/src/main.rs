#[derive(Debug)]
struct Users {
    name: String,
    age: u32,
    year: u32,
}

// Варианты создания метода
// fn print_user(self)
// fn print_user(self: Users) 
// fn print_user(self: Self)

impl Users{
    /*
    Чтобы объект можно было использовать многократно, в аргументах методов 
    обычно используют ссылки: fn print_user(&self) и fn year_plus(&mut self).
    // self
    // mut self
    // &self
    // &mut
     */
    fn print_user(self){
        println!("Имя: {}", self.name);
        println!("Возраст: {}", self.age);
        println!("Год: {}", self.year);
    }
    fn year_plus(mut self){
        self.year = self.year + 1;
        println!("{:#?}", self);
    }
}

fn main() {
    let user1 = Users{
        name: String::from("Вася"),
        age: 32,
        year: 1991,
    };
    // Объект user1 "умер" здесь, так как print_user забрал владение.
    user1.print_user();

        let user2 = Users{
        name: String::from("Петя"),
        age: 29,
        year: 1991,
    };
    // Объект user2 "умер" здесь, так как year_plus забрал владение.
    user2.year_plus();
}