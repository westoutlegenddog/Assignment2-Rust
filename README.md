# Assignment2-Rust
3210103818
杨朗骐

# Exercise 1  
代码如下：  
```rust
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
```
main()中测试代码如下：  
```rust
let mut buffer = Buffer::new();
buffer.push(114_000);
buffer.push(500);
buffer.push(14);
let sum = buffer.sum();
println!("Exercise 1.\nThe sum of elements in Buffer is {}", sum);
```
# Exercise 2
函数代码如下：  
```rust
// 根据ppt中函数名要求不修改此处
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
```
main()中测试代码如下：  
```rust
println!("\nExercise 2.");
let x = "computer";
let y = "compare";
println!("{} 比 {} 大吗？{} !", x, y, compareString(x, y));
let x = "company";
let y = "compare";
println!("{} 比 {} 大吗？{} !", x, y, compareString(x, y));
```
# Exercise 3
main()中代码如下：  
```rust
println!("\nExercise 3.");
let vec: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
let new_vec: Vec<char> = vec.iter().map(|&c| (c as u8 + 1) as char).collect();
println!("The new vector is {:?}", new_vec);
```
# 测试结果
均成功输出预期结果  
此处warning根据ppt中函数名要求不进行修改  
<img width="613" alt="截屏2023-09-05 下午11 52 13" src="https://github.com/westoutlegenddog/Assignment2-Rust/assets/103580732/1fb82df5-7f43-4b8b-824d-b8c326a8aa37">


