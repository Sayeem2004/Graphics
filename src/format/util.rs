/// Function that prints the type of the variable inputted
pub fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
