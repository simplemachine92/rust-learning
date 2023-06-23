fn permutations<T: Clone>(slice: &[T]) -> Vec<Vec<T>> {
    if slice.is_empty() {
        return vec![Vec::new()];
    }
    
    let mut result = Vec::new();
    for i in 0..slice.len() {
        let mut remaining_elements = slice.to_vec();
        let removed_element = remaining_elements.remove(i);
        
        for mut perm in permutations(&remaining_elements) {
            perm.insert(0, removed_element.clone());
            result.push(perm);
        }
    }
    result
}

fn is_sorted<T: Ord>(data: &[T]) -> bool {
    data.windows(2).all(|slice| slice[0] <= slice[1])
}

fn main() {
    let data = vec![3, 1, 2];
    let result = permutations(&data);

    for i in 0..result.len() {
        let isSorted = is_sorted(&result[i]);
        println!("{}", isSorted);
    }

    println!("{:?}", result);
}