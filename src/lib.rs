#[cfg(test)]
use rand;

pub fn sort<T>(mut vec: Vec<T>) -> Vec<T>
where
    T: Ord,
{
    for j in 0..vec.len() {
        let mut smallest = j;
        for i in j..vec.len() {
            if vec[i] < vec[smallest] {
                smallest = i
            }
        }

        vec.swap(smallest, j);
    }

    return vec;
}

#[cfg(test)]
mod tests {
    #[test]
    fn sort_4() {
        let vec: Vec<i32> = (0..2104).map(|_| rand::random()).collect();
        let mut sorted = vec.clone();
        
        sorted.sort();

        assert_eq!(super::sort(vec), sorted);
    }
}
