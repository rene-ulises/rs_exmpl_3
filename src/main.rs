use std::io::stdin;
fn main() {
    let result:i64;
    let first_val:i64 = read_integer("first");
    let second_val:i64 = read_integer("second");
    if first_val==second_val {
        result = 3*(first_val+second_val);
        print!("the triple of the sum is {}",result);
    }else {
        result = first_val+second_val;
        println!("The sum of the values is {}",result);
    }
}


fn read_integer(val:&str)->i64{
    println!("Enter the {} value: ",val);
    let mut in_str1 = String::new();
    stdin().read_line(&mut in_str1).expect("failed to read input.");
    let par_in: i64 = in_str1.trim().parse().expect("invalid input");
    par_in
}