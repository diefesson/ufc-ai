pub fn find_classes<T: PartialEq + Clone>(data: &[T]) -> Vec<T> {
    let mut classes = vec![];
    for d in data {
        if !classes.contains(d) {
            classes.push(d.clone());
        }
    }
    classes
}
