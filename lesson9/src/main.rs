fn main() {
    // Диапазоны и итерация по диапазонам
    let range1 = 1..30; // диапазон от 1 до 29
    println!("{range1:?}");
    let range2 = 1..=30; // диапазон от 1 до 30
    println!("{range2:?}");

    for num in range1 {
        println!("{num}");
    }

    //Диапазон символов
    let letters = 'b'..'f';
    for letter in letters{
        println!("{letter}");
    }
}
