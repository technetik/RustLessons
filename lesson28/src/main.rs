#[derive(Debug)]
struct Users {
    name: String,
    age: u32,
    year: u32,
}

impl Users{
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

    user1.print_user();
        let user2 = Users{
        name: String::from("Петя"),
        age: 29,
        year: 1991,
    };
    user2.year_plus();
}
