fn has_dup<T: PartialEq>(slice: &[T]) -> bool {
    for i in 1..slice.len() {
        if slice[i..].contains(&slice[i - 1]) {
            return true;
        }
    }

    false
}

fn unique_index(input: &str, window_size: usize) -> Option<usize> {
    for i in window_size..input.len()+1 {
        if !has_dup(&input[(i-window_size)..i].chars().collect::<Vec<_>>()) {
            return Some(i);
        }
    }
    None
}

pub fn run1(input: &str) -> String {
    unique_index(input, 4).unwrap().to_string()
}

pub fn run2(input: &str) -> String {
    unique_index(input, 14).unwrap().to_string()
}