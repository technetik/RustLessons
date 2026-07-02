fn main() {
    let values = [4, 8, 15, 16, 23, 42];

    // срез массива на первые 3 элемента
    let my_arr= &values[..3];
    println! ("{my_arr:?}");

    // срез массива на 3 и 4 элемент
    let my_arr= &values[2..4];
    println! ("{my_arr:?}");

    // срез массива с 3 до конца
    let my_arr= &values[2..];
    println! ("{my_arr:?}");

    // срез массива все элементы
    let my_arr= &values[..];
    println! ("{my_arr:?}");

    // полная ссылка, это не срез, тут другой тип &[i32; 6]
    let my_arr = &values;
    println! ("{my_arr:?}");
}
