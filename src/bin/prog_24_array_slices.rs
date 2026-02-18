
fn main() {
    let a : [u32; 7] = [2, 3, 4, 5, 1, 7, 9];

    // A slice of the array above has type &[u32]:
    let s_a : &[u32] = &a[2..6];

    assert_eq!(s_a, &[4, 5, 1, 7]);
    println!("s_a = {:?}", s_a);

}