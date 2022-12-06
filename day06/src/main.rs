use std::collections::{BTreeSet, VecDeque};

fn main() {
    let line = std::fs::read_to_string("tests/myinput.txt").unwrap();
    println!("Result: {}", find_start_of_packet(&line, 4));
    println!("Result (part 2): {}", find_start_of_packet(&line, 14));
}

fn has_unique_elements<T>(iter: T) -> bool where T: IntoIterator,T::Item: Eq + Ord, {
    let mut uniq = BTreeSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn find_start_of_packet<T: Into<String>>(input: T, distinct: usize) -> usize {
    let str: String = input.into();
    let tbl = VecDeque::from(str.as_bytes().to_vec());

    let mut search = 0;
    loop {
        if search + distinct < tbl.len() {
            if has_unique_elements(tbl.range(search..(search+1+distinct))) {
                return search + distinct;
            } else {
                search += 1;
            }
        } else {
            return usize::MAX;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::find_start_of_packet;

    #[test]
    fn example() {
        assert_eq!(7, find_start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
        assert_eq!(5, find_start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
        assert_eq!(6, find_start_of_packet("nppdvjthqldpwncqszvftbrmjlhg", 4));
        assert_eq!(10, find_start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
        assert_eq!(11, find_start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
    }

    #[test]
    fn example_pt2() {
        assert_eq!(25, find_start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
        assert_eq!(23, find_start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
        assert_eq!(23, find_start_of_packet("nppdvjthqldpwncqszvftbrmjlhg", 14));
        assert_eq!(29, find_start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
        assert_eq!(26, find_start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14));
    }
}
