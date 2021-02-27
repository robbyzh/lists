fn build_heap(array: &mut Vec<i32>) {
    for i in 1..array.len() {
        let mut j = i;
        while j >= 1 {
            let p = j / 2;
            if array[j] < array[p] {
                array.swap(p, j);
            }
            j = p;
            println!("{:?}", j);
        }
    }
}

#[test]
fn test_heap() {
    let mut vec = vec![4, 3, 7, 1, 8, 5, -2, -3];
    build_heap(&mut vec);
    println!("{:?}", vec);
}
