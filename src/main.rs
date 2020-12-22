use std::collections::linked_list; // starting point

mod cache_lib;
fn main() {
    // let x = 10;
    // let mut y: Vec<linked_list::LinkedList<i32>> = Vec::with_capacity(x);
    // for _z in 0..9 {
    //     y.push(linked_list::LinkedList::new());
    //     let z = &mut y[_z];
    //     z.push_back(69);
    // }
    
    // let j = &y[4];
    // assert!(j.contains(&69), "Does not contain 69!"); //passes
    let x: u64 = 0xEEEEEEEE;
    let y: u64 = cache_lib::addr_to_tag(x, 6);
    println!("{:064b}", y);
    println!("{:064b}", x);
}
