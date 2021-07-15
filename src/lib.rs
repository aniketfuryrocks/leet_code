use std::{cmp::Ordering, collections::HashMap};
/// 1. order has length at most 26, and no character is repeated in order.
/// 2. str has length at most 200.
/// 3. order and str consist of lowercase letters only.
pub fn custom_sort_string(order: String, str: String) -> String {
    // get chars
    let mut order_map:HashMap<char, usize> = HashMap::with_capacity(order.len());
    
    println!("{:?}", order_map);
    order.chars().enumerate().for_each(|(c , i)| { order_map.insert(i,c); });

    println!("{:?}", order_map);
    let mut str_chars:Vec<char> = str.chars().collect();

    str_chars.sort_by(| a, b | {
        println!("{} {}", a, b);
        match ( order_map.get(a), order_map.get(b) ) {
            ( Some(a), Some (b) ) => a.cmp(b) ,
            ( Some(a), None ) => Ordering::Less ,
            _ => Ordering::Greater 
        }
    });
    // convert str_chars back to string
    str_chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::custom_sort_string;

    #[test]
    fn check() {
        assert_eq!(custom_sort_string("cba".into(), "abcd".into()), "cbad");
        assert_eq!(custom_sort_string("exa".into(), "xwvee".into()), "eexvw");
    }
}

