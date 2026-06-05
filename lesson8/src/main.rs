fn main() {
    
	// Пример 1
    let user_data = ("Alice", 30, 4.5); // кортеж с именем, возрастом и рейтингом
	println!("Имя: {}", user_data.0);
	println!("Возраст: {}", user_data.1);
	println!("Рейтинг: {}", user_data.2);

    println!("{user_data:?}"); // Debug Trait содержимого

	// Пример 2
	let user_data1 = ("Белла", "Порч", 30);
	let last_name = user_data1.0;
	let first_name = user_data1.1;
	let age = user_data1.2;
	println!("Пример1 {last_name} {first_name} {age}");

	// альтернативный синтаксис, присваивания кортежа переменным
	let user_data2 = ("Белла", "Порч", 30);
	let (last_name1, first_name1, age1) = user_data2; // альтернативный синтаксис
	println!("Пример2 {last_name1} {first_name1} {age1}");

}
