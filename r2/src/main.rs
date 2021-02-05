fn main() {
    let cin = std::io::stdin();

    let mut input = String::new();
    
    cin.read_line(&mut input).unwrap();
    
    let values = input
        .split_whitespace()
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .unwrap();
    assert!(values.len() == 2);
    
    let r1 = values[0];
    let s = values[1];
    
    let r2: i32 = finding_r2(r1, s); 

//    println!("var1: {}, var2: {}", r1, s);
    println!("{}", r2);
}

// https://sciencing.com/missing-number-given-mean-5797971.html
fn finding_r2(r1: i32, s: i32) -> i32 {
    let x: i32;

    x = (s*2)-r1; 

    x
}
