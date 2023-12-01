
// TODO
// bessere imports (ohne Augenkrebs)
// Namen der Funktionen anpassen

mod day1;

fn main() {
    let input = day1::input_day_1();
    println!("{}", day1::puzzle1(&input));
    println!("{}", day1::puzzle2(&input));
}
