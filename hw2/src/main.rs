/*
3210103818
杨朗骐
assignment 2
*/

// Exercise 1
struct Buffer<T> {
    member: Vec<T>,
}

impl<T> Buffer<T> {
    fn new() -> Self {
        Buffer { member: Vec::new() }
    }
    fn push(&mut self, item: T) {
        self.member.push(item);
    }
    fn sum(&self) -> T
    where
        T: std::ops::Add<Output = T> + Default + Clone,
    {
        self.member
            .iter()
            .cloned()
            .fold(Default::default(), |temp_sum, next| temp_sum + next)
    }
}

// Exercise 2
fn compareString(x: &str, y: &str) -> bool {
    let chars1 = x.chars().collect::<Vec<_>>();
    let chars2 = y.chars().collect::<Vec<_>>();

    let len1 = x.len();
    let len2 = y.len();
    let min_len = {
        if len1 < len2 {
            len1
        } else {
            len2
        }
    };
    for i in 0..min_len {
        if chars1[i] < chars2[i] {
            return false;
        } else if chars1[i] > chars2[i] {
            return true;
        }
    }
    chars1.len() > chars2.len()
}

// Exercise 3, 见main()

fn main() {
    // Exercise 1
    let mut buffer = Buffer::new();
    buffer.push(114_000);
    buffer.push(500);
    buffer.push(14);
    let sum = buffer.sum();
    println!("Exercise 1.\nThe sum of elements in Buffer is {}", sum);

    // Exercise 2
    println!("\nExercise 2.");
    let x = "computer";
    let y = "compare";
    println!("{} 比 {} 大吗？{} !", x, y, compareString(x, y));
    let x = "company";
    let y = "compare";
    println!("{} 比 {} 大吗？{} !", x, y, compareString(x, y));

    // Exercise 3
    println!("\nExercise 3.");
    let vec: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
    let new_vec: Vec<char> = vec.iter().map(|&c| (c as u8 + 1) as char).collect();
    println!("The new vector is {:?}", new_vec);
}
