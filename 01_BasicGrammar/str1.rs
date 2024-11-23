fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert!(arr.len() == 5);

    let arr: [char; 3] = ['中', 'b', 'c'];
    let list: [i32; 100] = [1; 100];
    assert!(list[0] == 1);
    assert!(list.len() == 100);
}
