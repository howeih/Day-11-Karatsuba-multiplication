
use std::cmp;
fn karatsuba(n1: i64, n2 :i64) -> i64{
    if (n1 < 10) || (n2 <10){
        return n1*n2;
    }
    let n1_str = n1.to_string();
    let n2_str = n2.to_string();
    let max_length = cmp::max(n1_str.len(), n2_str.len());
    let split_pos = max_length / 2;
    let split_pos1 = n1_str.len() - split_pos;
    let split_pos2 = n2_str.len() - split_pos;
    let (high1, low1) = (n1_str[..split_pos1].parse::<i64>().unwrap(), n1_str[split_pos1..].parse::<i64>().unwrap());
    let (high2, low2) = (n2_str[..split_pos2].parse::<i64>().unwrap(), n2_str[split_pos2..].parse::<i64>().unwrap());
    let z0 = karatsuba(low1, low2);
    let z1 = karatsuba(low1 + high1, low2 + high2);
    let z2 = karatsuba(high1, high2);
    println!("z* {} {} {}",z0, z1, z2);
    (z2)*(10i64.pow(2 * split_pos as u32)) + ((z1 - z2 - z0)* (10i64.pow(split_pos as u32))) + z0
}


fn main() {
    let res = karatsuba(994759257, 928607936);
    println!("{}", res);
}
