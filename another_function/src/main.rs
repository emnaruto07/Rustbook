// fn main() {
//     println!("Hello, world!");

//     another_function(5, 'h');
// }

// fn another_function(value: i32, unit_label: char){
//     println!{"The value of x is: {}{}", value, unit_label};
// }

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {}", y)

// }

fn five() -> i32 {
    5
}

fn main(){
    let x = five();

    println!("The value of x is: {}", x);
}