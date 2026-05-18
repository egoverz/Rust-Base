fn main() {
    
    let mut args = Vec::new();

    for arg in std::env::args().skip(1) {
        args.push(arg);
    }

    sort(&mut args);
    println!("{:?}", args);

}

fn sort(input: &mut Vec<String>) {
    let l = input.len();

    for i in 0..l {
        let mut swapped: bool = false;
        for j in 0..l-i-1 {
            if input[j] > input[j + 1] {
                input.swap(j, j+1);
                swapped = true
            }
        }

        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_test_1() {
        let mut a: Vec<String> = vec![String::from("a"), String::from("A"), String::from("b")];
        let mut b: Vec<String> = vec![String::from("a"), String::from("A"), String::from("b")];

        a.sort();
        sort(&mut b);

        assert_eq!(a, b);
    }

    #[test]
    fn sort_test_2() {
        let mut a: Vec<String> = vec![String::from("a"), String::from("a"), String::from("X")];
        let mut b: Vec<String> = vec![String::from("a"), String::from("a"), String::from("X")];

        a.sort();
        sort(&mut b);

        assert_eq!(a, b);
    }


}