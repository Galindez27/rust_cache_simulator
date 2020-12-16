use std::collections::linked_list; // starting point

fn main() {
    let x = 10;
    let mut y: Vec<linked_list::LinkedList<i32>> = Vec::with_capacity(x);
    for _z in 0..9 {
        y.push(linked_list::LinkedList::new());
        let z = &mut y[_z];
        z.push_back(69);
    }
    
    let j = &y[4];
    assert!(j.contains(&69), "Does not contain 69!"); //passes
    assert!(j.contains(&60), "Does not contain 60!"); //panics
}
