```
rustlings watch
```
#### slice

```rust
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];
 // let nice_slice = a[1..4];  Error!  ----|  对比
    let nice_slice = &a[1..4];  -----------|
    assert_eq!([2, 3, 4], nice_slice)
}

// let nice_slice = a[1..4]; 
//                  ^^^ doesn't have a size known at compile-time
```

对比区别：`nice_slice` 有没有 `&`
- `a[1..4]` 表达式表示从数组 `a` 中获取一个切片（slice），不是一个数组。
- 切片是对数组的引用 Ref，它允许您引用数组的一部分而不复制数据。因此，`a[1..4]` 表达式返回的类型是 `&[i32]`，即对 `i32` 类型的切片的引用

原因：
 - 在 Rust 中，局部变量 local variable 的大小必须在编译时是已知的，这就意味着变量必须实现 `Sized` trait。数组切片（slice）是一个动态大小类型（DST），它不实现 `Sized` trait，因为它的大小在编译时是不固定的
 - `a[1..4];` 试图创建一个指向数组 `a` 中索引 `1` 到 `3` 的元素的切片。但由于切片是动态大小的，编译器无法确定 `nice_slice` 的大小，从而引发了错误。
修改：
 - 考虑借用切片: borrowing the slice instead of owning it directly
 - 这样，`nice_slice` 的类型将会是一个切片的引用 `&[{integer}]`，这是一个固定大小的类型，编译器将不会报错

> `[i32, 16]` 这是一个分配在栈上的定长数组
 
#### tuple

```rust
# define tuple 
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

```rust
# tuple destructuring...
let cat = ("Furry McFurson", 3.5);
let (name, age)  = cat;      // Attention
```

#### Vec

Create vec :
```rust
# method 1: Vec::new 后 不断 push()
let mut v = Vec::new();
v.push(10);  v.push(20);  v.push(30);    v.push(40);

# method 2 - use macro vec! 
let v = vec![10, 20, 30, 40]
```

```rust
# 将 v 这个 vec 里的每一个元素都 *2 
// v = vec![2, 4, 6, 8, 10];
for ele in v.iter_mut() {
 // *ele * 2   // Error ~~ expected `()`, found `i32`
	*ele *= 2
}
```

注意，
 - `v.iter_mut()` 返回一个可变 Iterator，会逐个产生向量 `v` 中元素的可变引用 (`&mut i32`)
 - 注释中  `*ele * 2`  的Error 是因为这个表达式计算了 `ele * 2` ，但没有将计算的结果赋回元素，`for` 循环的每次迭代都有一个表达式，Rust 要求这个表达式的类型必须是 `()`（单元类型），我们这里产生的是 `i32` 类型

```rust
v.iter().map(|ele| {
	ele * 2  // ✅
}).collect()
```
 - 这段代码使用的是 map 函数，而不是上面的 for 循环
 - 闭包 `|ele| ele * 2` 接收一个元素引用 `ele`( &i32 )，并计算它的两倍
#### move semantics

move semantics [^1]

```rust
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);
    vec
}
```
 - 观察参数，如果要改变 vec ，则需要在传参时指定其 mut
 - 为什么不在调用时写 mut ? 比如 `fill_vec(&mut vec0)` ? 
	 - 函数参数的可变性是在**函数签名**中定义的，而不是在函数调用时确定的。换句话说，当你传递一个变量给函数时，是否可以在函数内部修改这个变量取决于函数参数的定义
	 - `mut` 关键字是用于变量声明的，它定义了变量的可变性，而不是变量的使用方式

```rust
let mut vec1 = fill_vec(vec0);
let     vec1 = fill_vec(vec0);
```
 - 如上所述，2 种定义方法都是正确的，只是变量定义的可变性不同
#### match

有些 match 代码挺难写的，下面的例子做错，要仔细看

如下代码：
 - enum Message 代表了消息类型，比如 `移动`, `发声`, `变色`, `退出` ..
 - struct State 代表(一个角色) 目前的状态，比如 `颜色, 位置, 消息 Message`
 - 然后为 struct State 实现针对状态的各种函数 change_color() , quit() , echo() , ... 
```rust
enum Message {
  Move {x: u8, y: u8},
  Echo(String),
  ChangeColor(u8, u8, u8),
  Quit
}

struct Point { x: u8, y: u8, }

struct State {
  color: (u8, u8, u8),
  position: Point,
  quit: bool,
  message: String,
}

impl State {
  fn change_color(&mut self, color: (u8, u8, u8)) { self.color = color; }
  fn quit(&mut self) { self.quit = true; }
  fn echo(&mut self, s: String) { self.message = s }
  fn move_position(&mut self, p: Point) { self.position = p; }

  fn process(&mut self, message: Message) {
    // TODO: create a match expression to process the different message
    match message {
      Message::ChangeColor(r, g, b) => self.change_color((r, g, b)),
      Message::Quit => self.quit(),
      Message::Echo(s) => self.echo(s),
      Message::Move { x, y } => self.move_position(Point { x, y }),     } } }

// 调用逻辑：
  #[test]
  fn test_match_message_call() {
    let mut state = State {
      quit: false,
      position: Point { x: 0, y: 0 },
      color: (0, 0, 0),
      message: "hello world".to_string(),
    };
    state.process(Message::ChangeColor(255, 0, 255));
    state.process(Message::Echo(String::from("Hello world!")));
    state.process(Message::Move { x: 10, y: 15 });
    state.process(Message::Quit);
```

**match 匹配模式讲解：** `match message` 将 `message` 与四个模式进行匹配：
1. `Message::ChangeColor(r, g, b)`：如果 `message` 是 `ChangeColor` 变量，将其三个字段绑定到 `r`、`g` 和 `b`，然后执行 `self.change_color((r as u8, g as u8, b as u8))`
2. `Message::Quit`：如果 `message` 是 `Quit` 变量，执行 `self.quit()`
3. `Message::Echo(s)`：如果 `message` 是 `Echo` 变量，将其字段绑定到 `s`，然后执行 `self.echo(s)`
4. `Message::Move { x, y }`：如果 `message` 是 `Move` 变量，将其字段 `x` 和 `y` 绑定到变量 `x` 和 `y`，然后执行 `self.move_position(Point { x, y })`

```rust
state.process(Message::Move { x: 10, y: 15 });
  // Message::Move{x, y} => self.move_position(Point { x, y })

state.process(Message::ChangeColor(255, 0, 255)); 
  // Message::ChangeColor(r, g, b) => self.change_color((r, g, b)),

state.process(Message::Echo(String::from("Hello world!")));
  //  Message::Echo(s) => self.echo(s),

state.process(Message::Quit);
  // Message::Quit => self.quit(),
```

- 为什么 `change_color((` 用了 2 个括号？
	- 因为传入的是元组 tuple 作为参数  ( When passing a tuple as a function argument, you'll need extra parentheses: fn function((t, u, p, l, e))

match 和所有权转移:  
 - 试 fix 如下代码，使得 y 在 match 模式匹配后仍然可以继续使用
```rust
match y {
	Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
	_ => panic!("no match!"),
}
y;
```
 - Solution: 
	 - 只需把 `match y` 改成 `match &y` 即可
	 - 模式匹配时，Rust 会自动进行`解引用`，让我们能够匹配引用指向的内容
#### String

简易方法创建一个多行字符串：

```rust
    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }
```

------

现在，对 `&str` 有 3 个需求：分别是 `trim`, `拼接` 和 `替换` 
 - 观察下面函数的实现：
```rust
fn trim_me(input: &str) -> String {
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    format!("{} world!", input)  // hello world! ; GM world!
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    input.replace("cars", "balloons")
}
```

**为什么 `trim()` 后要 `to_string()` ?**
- 函数定义要求：将` trim()` 后的 `&str` 转换成 `String` 类型

**为什么不直接把 `trim()` 定义到 String 上面？**
 - ①  `String` 和 `&str`（字符串切片）是两种不同的字符串表示方式。`String` 是一个可增长、可修改、拥有所有权的字符串类型，而 `&str` 是一个不可变引用到字符串的切片
 - ② `trim()` 方法定义在 `&str` 上是因为该方法不需要修改原始字符串的内容或改变字符串的长度，它仅仅是返回原字符串中的一个子切片（不包含前后的空白字符）。由于 `&str` 是不可变的，这使得使用 `trim()` 方法非常安全和高效，不涉及额外的字符串分配或复制。
 - ③ 如果 `trim()` 方法定义在 `String` 上，那么它可能会给人一种错误的印象：即该方法会修改 `String` 的内容
 - ④ 将 `trim()` 方法定义在 `&str` 上使得它可以作用于任何字符串切片，无论它是从 `String` 还是字面量字符串 `&'static str` 获得的，这增加了该方法的通用性和灵活性。如果你有一个 `String` 对象并想使用 `trim()`，你可以轻松地通过调用 `as_str()` 方法或者使用 `&` 引用 或者 Deref 来获得一个 `&str` 引用，然后再调用 `trim()` 方法，如下代码

```rust
let s = String::from("  Hello, World!  ");
let trimmed = s.trim();  // 这里 String 会自动 Deref 到 &str 类型
```

------

请你说说下面这些表达式的类型，`&str`   or   `String` ？(后面有答案)

```rust
"red".to_string()
String::from("hi")
"rust is fun!".to_owned()
"nice weather".into()
format!("Interpolation {}", "Station") // Error
&String::from("abc")[0..1]
"  hello there ".trim()
"Happy Monday!".to_string().replace("Mon", "Tues") // Error
"mY sHiFt KeY iS sTiCkY".to_lowercase() // Error

```

Answer : 
```rust
&str slice:
  "blue"
  &String::from("abc")[0..1]
  "  hello there ".trim()

String :
  String::from("hi")
  "rust is fun!".to_owned()
  "nice weather".into()
  format!("Interpolation {}", "Station")
  "Happy Monday!".to_string().replace("Mon", "Tues")
  "mY sHiFt KeY iS sTiCkY".to_lowercase()
```
 - `"blue"` 是字符串字面量，类型为 `&str`
 - `"nice weather".into()` 利用 `into` 方法将 `&str` 转换为 `String`
 - `format!` 宏返回一个 `String`
 - `.trim()` 返回一个 `&str`
 - `replace` 方法返回一个新的 `String`
 - `to_lowercase` 方法返回一个新的 `String`

`to_owned`：
 - `to_owned()` 是 `ToOwned` trait 的一部分。这个 trait 通常用于从 `借用类型` 创建 Owned 类型，例如从 `&str` 创建 `String`，或从切片 `&[T]` 创建 `Vec<T>`
 - "owned 类型" 通常指的是拥有其内容的值的类型。例如，`String` 是 `&str` 的 owned 版本，`Vec<T>` 是切片 `&[T]` 的 owned 版本。Owned 类型的值在堆上分配内存，当它们离开作用域时，它们的析构函数会被调用，内存会被释放
 - 这个方法在 `&str` 上调用时会创建一个新的堆分配的字符串（`String`）

#### HashMap

```rust
let fruit_kinds = vec![
	Fruit::Apple,
	Fruit::Banana, // ...
];

for fruit in fruit_kinds {
	if !basket.contains_key(&fruit) {
		basket.insert(fruit, 1);
	}
}
```

为什么 `contains_key(&fruit)` 用 `&` ， 但是 `basket.insert(fruit, 1);` 不用 `&`  ？

**`contains_key` 方法**：
```rust
fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
where
    K: Borrow<Q>,     
    Q: Hash + Eq,
```

`contains_key` 只需要一个对键的引用（`&Q`），因为它不会更改键或哈希映射的状态。由于它仅需要引用，我们使用 `&fruit` 来传递对键的引用

**`insert` 方法**：
```rust
fn insert(&mut self, k: K, v: V) -> Option<V>
```

`insert` 方法需要键的所有权，因为哈希映射需要存储该键。由于 `Fruit` 枚举实现了 `Copy` trait，当我们调用 `insert(fruit, 1)` 时，实际上是复制了一个 `fruit` 的副本传递给 `insert` 方法，因此我们不需要使用引用 `&` 

-------

需求：每次循环，都会给出球队 和 得分情况，比如：`德国,英格兰,3,1` , 意思就是比分： `德国: 英国= 3: 1` ， 即
 - 德国得分： 3，失分： 1
 - 英国得分： 1，失分： 3
 - 不断循环，给出最后每个球队的总得分和总失分

```rust
struct Team {
    goals_scored: u8,    // 得分
    goals_conceded: u8,  // 失分
}

let mut scores: HashMap<String, Team> = HashMap::new();
for r in results.lines() {
   // ... 
   // .. team_1_name,team_1_score,  team_2_name,team_2_score,
   let team_1 = scores
      .entry(team_1_name)
      .or_insert_with(|| Team { goals_scored: 0, goals_conceded: 0 });
    team_1.goals_scored += team_1_score;
    team_1.goals_conceded += team_2_score;

    let team_2 = scores
       .entry(team_2_name)
       .or_insert_with(|| Team { goals_scored: 0, goals_conceded: 0 });
	team_2.goals_scored += team_2_score;
	team_2.goals_conceded += team_1_score;
```

`scores.entry` : 查找 HashMap `scores` 中是否存在 `team_1_name` 这个 key ：
- If so，返回这个 key 对应的 mut ref，即 `team_1` 对应的 `Team` struct 实例的引用
- If not，它会使用 `or_insert_with` 方法中的闭包 `|| Team { goals_scored: 0, goals_conceded: 0 }` 创建一个新的 `Team` 实例，并将其插入 HashMap，然后返回这个新值的可变引用

#### Exercise 2 (中期 Exercise)

给定一组`[字符串]`和`[命令]`，返回一个 `vec![]` ,  里面存放着操作后的字符串。如：
- input:` ("hello".into(), Command::Uppercase),`
- output: `HELLO`
- 注：字符串是 "hello"， 命令是 Uppercase，结果是 "HELLO"

```rust
pub enum Command {
  Uppercase,
  Trim,
  Append(usize), // 添加 n 次 'bar' 到字符串屁股后面.
}

mod my_module {
  use super::Command;

  pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    // 由于我们要消耗 input，因此使用 into_iter 而不是 iter
    for (string, command) in input.into_iter() { 
      match command {
        Command::Uppercase => output.push(string.to_uppercase()),
        Command::Trim => output.push(string.trim().to_string()),
        Command::Append(n) => {
          let mut s = string;
          for _ in 0..n { s.push_str("bar"); }
          output.push(s);
        }
      }
    }
    output
  }
}

#[cfg(test)]
mod tests {
  // TODO: What do we need to import to have `transformer` in scope?
  // use my_module::transformer;
  use super::my_module::transformer;
  use super::Command;

  #[test]
  fn it_works() {
    let output = transformer(vec![
      ("hello".into(), Command::Uppercase),
      (" all roads lead to rome! ".into(), Command::Trim),
      ("foo".into(), Command::Append(1)),
      ("bar".into(), Command::Append(5)),
    ]);
    assert_eq!(output[0], "HELLO");
    assert_eq!(output[1], "all roads lead to rome!");
    assert_eq!(output[2], "foobar");
    assert_eq!(output[3], "barbarbarbarbarbar");
  }
}
```

 - `use super::my_module::transformer;`: 
	 - 仔细体会，` mod xxxx` 创建的是一个一个命名空间
	 - 使用 `super` 来回溯上一级
 - `Vec<(String, Command)` :  `Vec<>` 里存放的是 `()` tuple
 - `into_iter()` instead of  `iter()` : 
	 - `iter()` 返回一个不可变引用的迭代器  `Iterator<Item=&T>`
	 - `into_iter()` 会获取集合的所有权并返回一个值的迭代器`Iterator<Item=T>`
 - `"hello".into()` : 
	 - "hello" 是一个 string literal，是个 `&str` 类型，为了匹配 transformer 函数的参数，需调用 `.into` 将其转为 `String`

#### Control flow

如下代码使用 `if let Some(word)` 试探究使用 `Some` 的合理性
```rust
let target = "rustlings";

// optional_target 是个 Optional 类型
if let Some(word) = optional_target {
	// 代码块，如果 expression 匹配 Some(pattern)，则执行这里的代码
	assert_eq!(word, target);
}
```
 - `Some(pattern)` 是我们要匹配的模式，其中 `pattern` 是我们想要从 `Some` variant 中提取的值
	 - 如果 `optional_target` 是 `Some()` variant（即不是 `None`），则将其中包含的值绑定到变量 `word` 上，所以 `word` 的类型是 `Option` 中包含的类型，即 `&str` 
	 - 所以 `if let Some(xx)` 就相当于是个解包，把原本包在 `Some(xx)` 里的 xx 给 **脱壳**

如下代码使用 `Some(Some(integer))` , 试探究 :
```rust
#[test]
fn layered_option() {
	let range = 10;
	let mut optional_integers: Vec<Option<i8>> = vec![None];

	for i in 1..(range + 1) {
		optional_integers.push(Some(i));
	}

	let mut cursor = range;

	// TODO: make this a while let statement - remember that vector.pop also
	// adds another layer of Option<T>. You can stack `Option<T>`s into
	// while let and if let.
	while let Some(Some(integer)) = optional_integers.pop() {
		assert_eq!(integer, cursor);
		cursor -= 1;
	}

	assert_eq!(cursor, 0);
}
```
 - `Some(Some(integer)) : `
	 - `pop()` 方法本身会返回一个 `Option<T>` 类型

#### error_handling

##### 使用 Result

Result 处理字符串为空 `""` 时的情况，我们报一个带信息的 Error !

```rust
// 我一开始写的代码 ❌：
pub fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Result<"Error!"> // 这什么东西？？
    } else {
        Some(format!("Hi! My name is {}", name))
    }
}

// Solution ✅
pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}
```
 - 返回值由 `Result<T, E>`  instead of  `Option<T>`
 - `Err("xxx".into())` 
	 - `Err()` 里面要包含一个 String 来指示具体的错误
	 - why `.into()` ? 
		 - 因为 "xxx" 是一个 String lateral 是 `&str` 类型，所以需要 `.into()` 转成 String
- 正常的值是用 `Ok()` wrap 起来的，使用的时候需要解包，如：

```rust
fn main() {
    let result = generate_nametag_text("John".into());
    
    if let Ok(name_tag) = result { // use Ok to deconstruct
        println!("Generated nametag: {}", name_tag);
    }
    // ....
```


------ 

##### 使用 ?

如下函数：用户输入一个字符串数字，我们将其解析为数字，如果用户输入的东西不能被解析为数字，比如他输入了 “beep beep”，我们需要 raise 一个 `ParseError` !

```rust
// unfinished wrong code ❌:
pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let qty = item_quantity.parse::<i32>();
    let processing_fee = 1;
    let cost_per_item = 5;

    Ok(qty * cost_per_item + processing_fee)
}

// Solution ✅
pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let qty = item_quantity.parse::<i32>()?;
    // ...
}

// 调用
if let Ok(cost) = total_cost(user_input) {
    // 对绑定到 cost variant 的处理代码 ....
}
```

 - 只在 parse 后面加了一个  `?` 就解决了
 - `?`  用于简化 `Result` Error handle, 

Below is a trivial solution ✅ ，虽然正确，但还要 Pattern match `parse()` 的 return value，不美观： 
```rust
pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    
    // 使用 match 表达式显式处理 Result
    let qty = match item_quantity.parse::<i32>() {
        Ok(value) => value, // 如果是 Ok 变体，取出内部的值
        Err(e) => return Err(e), // 如果是 Err 变体，提前返回 Err
    };
    
    Ok(qty * cost_per_item + processing_fee)
}
```

##### impl error::Error

需求：让 `main()` 函数可以处理任何实现了 `error:Error Trait` 的类型

背景：让用户输入一个数，必须是正整数，否则就报错，该数用 struct `PositiveNonzeroInteger`  wrap 起来

思路：必然要使用 Trait Object:  `fn main() -> Result<(), Box<dyn error::Error>>`

Tips : 
 - Rust 中，为了使一个类型能被用作错误类型，它必须要实现 `std::error::Error` trait。这个 trait 本身没有强制要求实现任何方法 :
```rust
impl error::Error for CreationError {}
```

 - 但唯一需要注意的 ： `std::error::Error` trait 依赖于 `std::fmt::Display` trait。这是因为错误类型需要能被友好地展示给用户，所以任何想实现 `Error` trait 的类型也必须实现 `Display` trait，来定义如何格式化和展示错误信息，
 - 实现 `Display` trait 是实现自定义错误类型的常见做法，且是实现 `Error` trait 的前提:

```rust
// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}
```

实现思路：现在思路很清晰了
1. 定义一个错误类型 struct : `CreationError` 
2. 为这个 struct 实现 `error::Error` Trait 
3. 因为 `std::error::Error`  依赖于 `std::fmt::Display`, 顺便要把 `impl fmt::Display for CreationError`  实现了
4. 使用 Trait Object 处理任何实现了 `error::Error` Trait  的 struct

```rust
use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
fn main() -> Result<(), Box<dyn error::Error>> {
    let user_input_ = "42";
    let x: i64 = user_input_.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,  // 处理负数错误
    Zero,      // 处理 0 值错误
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
```


最后一个问题：在 Pattern match 里，`match value {  x if x < 0 => Err`  里第一个 `x` 是干嘛用的？
 - 这里的 `x if x < 0` 可以解读为：将`value`绑定到`x`，然后检查`x`是否小于0。如果是，则执行对应的分支
 - 在这个例子里，通配符 `_` 作为模式，也可以不绑定 `value` 到新变量，而是直接在 guard 中使用 `value` ： 

```rust
match value {
    _ if value < 0 => Err(CreationError::Negative),
    _ if value == 0 => Err(CreationError::Zero),
    _ => Ok(PositiveNonzeroInteger(value as u64)),
}
```
 - 👆🏻 也是完全合理的



##### 错误类型自动转换

如上的 `<(), Box<dyn error::Error>>` 错误处理方式是 don't recommended 的： 不建议在库代码中使用像 `Box<dyn error::Error> `这样的包罗万象的错误类型，因为调用者可能希望根据错误内容做出决策，而不是将其打印出来或进一步传播

Error Type 自动转换 - 核心流程
1. **自定义错误类型**:
- `ParsePosNonzeroError`是一个枚举，它有 2 个 variants ，代表了两种可能的错误类型: `Creation(CreationError)`和`ParseInt(ParseIntError)`
	- `CreationError` handle illegal input like 0, -99, -1 ..
	- `ParseIntError` handle illegal input like "buzz buzz"
- `CreationError`枚举，代表了在创建`PositiveNonzeroInteger`时可能会出现的错误:
	- Negative 
	- Zero
```rust
enum ParsePosNonzeroError {
    Creation(CreationError), { // handle illegal input like 0, -99, -1 ..
        Negative, Zero
    }
    ParseInt(ParseIntError), // handle illegal input like "buzz buzz" 
}
```

2. **实现 From trait**:
- 对于`ParsePosNonzeroError`来说，其包含的 2 种错误：`ParseIntError`（当解析字符串到数字时）和 `CreationError`（当数字是零或者负数时），这两种错误都需要被转换为 `ParsePosNonzeroError` 类型，以便返回给函数调用者
- 为什么要让两种错误可以被自动地转换成`ParsePosNonzeroError` 呢？
	- 为了简化错误处理和提供统一的错误接口
- 如何实现 ? —— 为 2 种错误实现 `From` Trait
	- `From` trait是用于类型之间的转换的。当你为一个类型实现了`From` trait，这个类型就可以被自动地转换到目标类型。在这个情况下，实现了`From<CreationError> for ParsePosNonzeroError` 和 `From<ParseIntError> for ParsePosNonzeroError`，意味着`CreationError`和`ParseIntError`类型的值可以被自动转换成`ParsePosNonzeroError`类型
	- 具体来说，当你在函数中使用 `?` 操作符时，如果发生了错误，Rust会查找是否存在从实际错误类型到函数返回错误类型的 `From` trait实现。如果存在，Rust会自动调用这个实现来转换错误类型。所以，在这个例子中，通过实现 `From<CreationError> for ParsePosNonzeroError` 和 `From<ParseIntError> for ParsePosNonzeroError`，你可以在函数中方便地使用 `?` 来处理错误，并将它们转换为 `ParsePosNonzeroError` 类型

3. **错误转换与传播**:
- 在`parse_pos_nonzero`函数中，字符串解析的错误（`ParseIntError`）和创建`PositiveNonzeroInteger`的错误（`CreationError`）都通过`.map_err(ParsePosNonzeroError::from)`的方式被转换成`ParsePosNonzeroError` 
- 使用`?`操作符，如果`Result`是`Err`，函数会提前返回错误；如果是`Ok`，则继续执行。

```rust
use std::num::ParseIntError;

// This is a custom error type that we will be using in `parse_pos_nonzero()`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError), // handle illegal input like 0, -99, -1 ..
    ParseInt(ParseIntError), // handle illegal input like "buzz buzz" 
}

// CreationError 转换-> ParsePosNonzeroError
impl From<CreationError> for ParsePosNonzeroError {
    fn from(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
}

// ParseIntError 转换-> ParsePosNonzeroError
impl From<ParseIntError> for ParsePosNonzeroError {
    fn from(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)
    }
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    // let x: i64 = s.parse()?;
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::from)?;
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from)
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}
```


#### generics

`练习 1 :` emmm
```rust
fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
```

`练习 2` : 
 - 注意 `<T>`  的位置
```rust
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}
```

#### Trait

为 `Vec<String>` 实现往屁股后面追加一个 "Bar",  例如：
 - 调用前： `vec!["Foo"]`
 - 调用后： `vec!["Foo", "Bar"]`

```rust
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar")); // 向向量末尾追加一个新的String，内容为"Bar"
        self // 返回修改后的向量
    }
}
```

观察到： trait 中 `fn append_bar(self)` 和 impl 里 `fn append_bar(mut self)` 并不完全一致： impl 里多了 `mut` : 
 - 这是完全允许的，因为 `mut` 关键字只影响函数体内部对 `self` 的可变性
 - 当我们在 trait 定义中声明 `fn append_bar(self) -> Self;` 时，我们仅仅是在表明这个方法会取得 `self` 的所有权，并且没有指定 `self` 是否是可变的
 - 在实现 trait 的时候，我们可以决定是否需要 `self` 是可变的 : 
	 - 如果我们需要修改 `self`，那么我们在 impl 时可以加上 `mut` 来获得 `self` 的可变引用
	 - 这样的设计给予了我们很大的灵活性，允许我们在不同的实现中选择是否要 `mut self`

##### Trait bounds: impl xxTrait

背景：比较 2 个软件(software) 的 license 是否一致
```rust
fn compare_license_types(sf1: impl Licensed, sf2: impl Licensed) -> bool {
    sf1.licensing_info() == sf2.licensing_info()
}
```

这是 Trait bounds 语法，用来表明这个函数可以接受任何实现了 `Licensed` trait 的类型作为参数

这个函数签名中，`sf1` 和 `sf2` 需要实现 `Licensed` trait  ，函数体中，我们调用了这两个对象的 `licensing_info` 方法（由于他们都实现了 `Licensed` trait，所以我们可以确保这个方法是存在的），然后比较了这两个字符串是否相等，最后返回了这个比较的结果

-----

Trait bound - 2 来袭 !!
现在我们要求： 函数 `some_func` 能够接受任何**同时实现了** `SomeTrait` 和 `OtherTrait` 的类型作为参数：

```rust
fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
    item.some_function() && item.other_function()
}

或者：

fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool {
    item.some_function() && item.other_function()
}
```

这 2 种写法都是可以的。
 - some_func`(item: impl SomeTrait + OtherTrait)`
 - some_func`<T: SomeTrait + OtherTrait>(item: T)`

#### Exercise 3 (泛型+Trait)

背景：打印 2 种格式的成绩单，分别支持 numerically (e.g. 1.0 -> 5.5) 和  alphabetical grades (A+ -> F-) 

思路：泛型

现有代码：
 - ReportCard 成绩单，可以看到目前的成绩只支持 `f32` 
 - 当 grade 改成泛型后，必须确保类型能使用 format 输出，即支持 `std::fmt::Display` Trait
```rust
pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}
```

实现代码：

```rust
pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: std::fmt::Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}
```

#### lifetime

```rust
# 待解题代码
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

# Solution ✅
fn longest<'a>(x : &'a str, y: &'a str) -> &'a str {
```

在 `longest` 参数中，引入了生命周期 `'a`，并将它应用到参数 `x` 和 `y` 以及返回值类型 `&'a str` 上。这告诉编译器，`result` 的生命周期与输入参数的生命周期一致，这样就不会出现生命周期不匹配的错误了

------

场景：仍是上面的 longest 函数定义，这次我们在调用的时候做了手脚：
```rust
fn longest<'a>(x : &'a str, y: &'a str) -> &'a str { ... } 

fn main() {
    let s1 = String::from("long string is long");
    let result;
    {
        let s2 = String::from("xyz");
        res = longest(s1.as_str(), s2.as_str());
    }
    println!("The longest string is '{}'", result);
}

res = longest(s1.as_str(), s2.as_str());  ❌  
                           ^^^^^^^^ borrowed value does not live long enough
```

错误分析：尝试在 `s2` 的生命周期结束后继续使用 `res` ，❌ ，不能让 `result` 引用 `string2`，因为它的生命周期已经结束了

这道题无法通过修改 longest 的函数定义来解决，因为生命周期标注只是一个标注，它不改变任何实际的生命周期

```rust
// Solution ✅
fn main() {
    let s1 = String::from("long string is long");
    let s2 = String::from("xyz");
    let res = longest(s1.as_str(), s2.as_str());
```


------

结构体的生命周期：

```rust
// ❌ 代码：
struct Book {
    author: &str,
    title: &str,
}
```
  
在 `struct Book` 的定义中，我们引入了生命周期参数 `'a`，并将其应用于 `author` 和 `title` 字段，以指示这两个字段的引用与结构体的生命周期相绑定。

在 `main` 函数中，我们创建了 `name` 和 `title` 字符串，然后使用它们来初始化 `Book` 结构体的字段。由于 `Book` 结构体的生命周期参数是 `'a`，因此 `author` 和 `title` 字段的引用必须与 `'a` 生命周期相匹配。这确保了在 `Book` 结构体中存储的引用在其所有权者（`name` 和 `title`）有效的情况下才能使用

```rust
// ✅  Solution:
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };
    println!("{} by {}", book.title, book.author);
}
```

#### Tests

常用的 assert :
- `assert!`, `assert_eq!`, `assert_ne!`, 它们会在所有模式下运行
- `debug_assert!`, `debug_assert_eq!`, `debug_assert_ne!`, 它们只会在 `Debug` 模式下运行

##### assert_eq

```rust
# constrain 2 == second.
assert_eq!(2, second, "This is not the 2nd number in the tuple!")
```

##### should_panic

写了几个必须 `panic` 的错误测试用例，如何测试？

```rust
    #[test]
    fn correct_width_and_height() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // check width
        assert_eq!(rect.height, 20); // check height
    }

    #[test]
    #[should_panic]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }
    #[test]
    #[should_panic]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
```
 - 如上代码 `#[should_panic]` , 在传入长度、宽度为 negative  的 rectangle 时，函数应该 panic，测试通过也就说明确实 panic 了。

#### Iterator

- `into_iter` 会**夺走所有权**
- `iter` 是借用
- `iter_mut` 是可变借用

-----

##### iter vs. into_iter : 

 - 如下 2 个 `main()` 都是正确的, 区别在 2 个地方：
	 - `fruits.into_iter();`  and    `fruits.iter();`
	 - `Some("banana"))`   and    `Some(&"banana"))`
```rust
fn main() {
    let fruits = vec!["banana", "peach",];
    let mut iter_fruits = fruits.into_iter();

    assert_eq!(iter_fruits.next(), Some("banana"));
    assert_eq!(iter_fruits.next(), Some("peach"));    
    assert_eq!(iter_fruits.next(), None);
}
// --------------------------------  //
fn main() {
    let fruits = vec!["banana", "peach",];
    let mut iter_fruits = fruits.iter();

    assert_eq!(iter_fruits.next(), Some(&"banana"));
    assert_eq!(iter_fruits.next(), Some(&"peach"));    
    assert_eq!(iter_fruits.next(), None);
}
```
 - `fruits` 实现了`IntoIterator` Trait，因此可以通过 `.iter()` /   `.into_iter()` 将其转换为迭代器
 - `"banana"` 是 String lateral，其类型是 `&str` ;
 - `Some("banana")` 的类型是 `Option<&&str>` ;
	 - 在 `.iter()` 的实现中，每个迭代器元素都是原来 `ele` 的 reference
 - `Some(&"banana")` 的类型是 `Option<&str>` ;
	 - 在 `.into_iter()` 实现中，每个迭代器元素都是原来 `ele` itself

```rust
impl<I: Iterator> IntoIterator for I {
    type Item = I::Item;
    type IntoIter = I;
    #[inline]
    fn into_iter(self) -> I {
        self     
    } 
}
```
##### map

 - 实现一系列首字母大写的函数，比如： 
	 - `"hello" `->` "Hello"`
	 - `["hello", "world"]`  ->  `["Hello", "World"]`
	 - `["hello", " ", "world"]`  ->  `"Hello World"`
 - 根据上述需求，我们需要实现 3 个函数：
	 - 1. `capitalize_first` —— 将 "hello" 首字母大写
	 - 2. `capitalize_words_vector` —— 利用 `.map()` ，将 vec 里的所有 word 都 apply `capitalize_first`
	 - 3. `capitalize_words_string` , 利用 `.join` 将 vec 里的元素 concat 成 String


具体实现：Part-1 ， Part-2， Part-3：

```rust
// Part - 1
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars(); // 创建字符迭代器 Chars(['h','e','l','l','o'])
    match c.next() {  // 获取迭代器的第一个字符
        None => String::new(),   // 如果没有字符，则返回空字符串
        Some(first) => {
            let capitalized = first.to_uppercase(); // 将第一个字符大写
            capitalized.to_string() + c.as_str() // 添加剩余字符并转换为字符串
        }
    }
}
```
 - `&str.chars()` 方法返回一个包含 `char` 值的迭代器
 - 这里 only call   `c.next()`  once ，所以是获取了首字母（第一个字母）
 - `c.as_str()` 返回了一个字符串切片，指向了迭代器 `c` 中剩余未处理的字符部分。这个切片没有进行额外的内存分配或者字符串复制，而只是提供了对原始字符串中字符的引用
 - `capitalized.to_string() + c.as_str()` ： 
	 - 一个`String`和一个`&str`相加时，Rust会自动将`&str`转换为`String`，然后执行拼接操作。因为`String`实现了`Add` trait，其中有一个与`&str`相加的实现，允许你直相加


```rust
// Part - 2
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String>{
    words.iter() // 创建字符串切片的迭代器
        .map(|&word| capitalize_first(word)) //处理每个 word
        .collect::<Vec<String>>() // 收集处理后的结果并返回为向量
        // .collect() 不写类型也可以
}

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }
```
 - 本部分代码处理 `["hello", "world"] -> ["Hello", "World"]`
 - 参数部分： `&[&str]` 代表对`words` 切片的引用，而不是移动 `words` 到函数中。这可以帮助读者理解代码的行为
 - `.collect() ` 也可以不写具体类型让他自动推断为函数返回值  `Vec<String>`

```rust
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter() // 创建字符串切片的迭代器
        .map(|&word| capitalize_first(word)) // map 处理每个单词
        .collect::<Vec<_>>() // 收集处理后的结果并返回为向量
        .join("") 
```
- `join` 将 Vec 转化为 String

-------

##### bigger exercise 

 - 背景：一个整除问题，要对 `除 0`  及 `无法整除` define Error，然后用迭代器和 map 实现数组的计算

错误设计：
```rust
#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),  --------|
    DivideByZero,                             |
}                                             |
                                              |
#[derive(Debug, PartialEq, Eq)]               |
pub struct NotDivisibleError {    <-----------|  
    dividend: i32,
    divisor: i32,            }
```


`divide` 除法函数设计 ：

```rust
// Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 { return Err(DivisionError::DivideByZero) }
    if a % b == 0 {
        return Ok(a / b)
    }else{
        return Err(DivisionError::NotDivisible(NotDivisibleError{
            dividend: a,
            divisor: b,
        }))
    }
}
```
 - 注意，NotDivisibleError 是个 struct，所以构建这个错误时需要传递相应的参数。


2 个 list 的 `.map()` 处理：
 - 返回值不同:  `Ok([1, 11, 20, 100])`    and   `[Ok(1), Ok(11), Ok(20), Ok(100)]`
 - 看到 variant 类型 notation 对于 `.collect()` 类型推导的影响
```rust
// Complete the function and return a value of the correct type so the test
// passes.
// Desired output:   Ok([1, 11, 20, 100])
fn result_with_list() -> Result<Vec<i32>, DivisionError> {
    let numbers = vec![7, 77, 140, 700];
    let division_results: Result<Vec<i32>, DivisionError> = numbers
        .into_iter()
        .map(|n| divide(n, 7))
        .collect();
    division_results
}

// Complete the function and return a value of the correct type so the test
// passes.
// Desired output:   [Ok(1), Ok(11), Ok(20), Ok(100)]
fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![7, 77, 140, 700];
    let division_results = numbers
        .into_iter()
        .map(|n| divide(n, 7))
        .collect();
    division_results
}

#[test]
fn test_result_with_list() {
	assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 20, 100])");
}

#[test]
fn test_list_of_results() {
	assert_eq!(format!("{:?}", list_of_results()),"[Ok(1), Ok(11), Ok(20), Ok(100)]");
}
```

##### 阶乘 factorial

计算 num 的阶乘 （注意，num = 0 时，阶乘为 1 ）：
```rust
// ✅ Solution 
pub fn factorial(num: u64) -> u64 {
    (1..=num).fold(1, |acc, x| acc * x)
}
```
 - `(1..=num)` 和 `(1..num+1)` 是等价的

[fold 函数:](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
```rust
fn fold<B, F>(self, init: B, f: F) -> B
   where Self: Sized,
		 F: FnMut(B, Self::Item) -> B { ... }
```

`fold` 方法接受两个参数：
1. `init`：初始累加器的值，是累加过程的起点
2. `f`：一个闭包，该闭包接受两个参数，① 当前的累加器值，② 迭代器产生的下一个元素，闭包返回一个新的累加器值

`fold` 方法的工作流程如下：
1. 初始化累加器为 `init`
2. 对于迭代器中的每个元素，调用闭包 `f`，并将当前累加器值和元素作为参数传递给闭包
3. 闭包返回一个新的累加器值，它将在下一次迭代中被用作当前的累加器值
4. 迭代继续，直到遍历完所有元素。
5. 最后，`fold` 方法返回最终的累加器值

下面是一个简单的示例，演示了如何使用 `fold` 计算一个数字向量的总和：
```rust
let numbers = vec![1, 2, 3, 4, 5]; 
let sum = numbers.into_iter().fold(0, |acc, x| acc + x);
```

##### count iterator

现在有这样一个 Table，里面存放着我们的解题的 progress , 如下: 
 - 题目有 3 种状态，完成(Complete)，解题中(Some) 和 未解答(None)
```rust
map.insert(String::from("variables1"), Complete);
map.insert(String::from("functions1"), Complete);
map.insert(String::from("hashmap1"), Complete);
map.insert(String::from("arc1"), Some);
map.insert(String::from("as_ref_mut"), None);
map.insert(String::from("from_str"), None);
```

现在的需求是，写几个函数，要根据**状态**做筛选然后计算个数，比如 Complete 的题目有几个, Some 状态的题目有几个 ...

思路：**filter** 

Task-1 : 给一个 `&HashMap<String, Progress>` ，计算处在某状态的题目数
```rust
fn count_iterator(maps: &HashMap<String, Progress>, prog: Progress) -> usize {
    // maps is a hashmap with String keys and Progress values.
    // maps = { "variables1": Complete, "from_str": None, ... }
    maps.iter().filter(|(_, &item)| item == prog).count()
}
```
 - `HashMap<String, Progress>` 里的 key 和 value 可以使用 `tuple` 解包 deconstruct
 - 注意，这里直接对 maps 这个 HashMap 调用了 `iter()` ， 因为`HashMap` 是一个实现了 `IntoIterator` trait
	 - solidity 里的 mapping 无法迭代，不要带跑偏了印象流

filter 详解：
```rust
vec.iter().filter(|closure_variable| closure_condition).collect();

// 例子:
let week_days = vec!["Monday", "Tuesday", "Wednesday", ..]
let filtered_week_days: Vec<_> = week_days
        .iter()
        .filter(|days| days.len() < 8)
        .collect();
```

-----

Task-2 : 给一组 `&HashMap<String, Progress>` 的数组切片，计算所有处在某状态的题目数

```rust
fn cc(collection: &[HashMap<String, Progress>], prog: Progress) -> usize {
    // collection is a slice of hashmaps.
    // collection = [{ "variables1": Complete, "from_str": None, ... },
    //     { "variables2": Complete, ... }, ... ]
    collection
        .iter()
        .flat_map(|map| map.iter())
        .filter(|&(_, &ele)| ele == prog)
        .count()
}
```

 - `flat_map` 的作用：用于将每个 HashMap 转换成一个迭代器
 - 问题：为什么不能直接对 HashMap 调用 `.iter()` ？ 就是直接 `collection.iter().iter()` 不行吗？
	 - 为了遍历切片中的每个 `HashMap`，你可以使用 `.iter()` 方法来遍历切片本身，然后再使用 `.iter()` 方法来遍历每个 `HashMap`。这就是代码中使用 `flat_map()` 的原因，它允许你将两层迭代器（切片和哈希映射内部的迭代器）"平铺"成一个单层迭代器，以便于后续的操作。如果直接使用 `collection.iter().iter()`，会导致类型不匹配错误，因为 `collection.iter()` 返回的是一个切片迭代器，而不是 `HashMap` 的迭代器。

```rust
let words = ["alpha", "beta", "gamma"];

// chars() 返回一个迭代器
let merged: String = words.iter()  // get "alpha"
                          .flat_map(|s| s.chars())  
                          .collect();
assert_eq!(merged, "alphabetagamma");
```
 - 这边看似也是一个连续 `.iter()` 的例子，先 `.iter()` 获取第一个值 "alpha"
 - `"alpha"` 再取 `.chars()` , .`chars()` 也返回一个迭代器

或者这么写（但是推荐 `flat_map`, 它可以更清楚地传达意图）：
```rust
let merged: String = words.iter()
                          .map(|s| s.chars())
                          .flatten()
                          .collect();
```


-----

map + ? : 
如下的需求是将一个 定长数组 `[i16;3]` 转换成一个 Color struct 类型
 - 因为 定长数组 `[i16;3]`是在栈上分配的，所以函数拥有所有权（直接复制一份原值过来的）
 - `arr.iter()` 后，获取了迭代器里每个值的 Ref，所以仍需要用 &d 匹配解包，获取实际值
 - 在 map 中我们做如下处理：
	 - 如果值是 0~255 则不做什么，直接转成 u8 类型
	 - 如果值不在这个范围内，则返回一个错误
	 - 因为错误类型 和 u8 不一样，所以考虑用 Result Wrap 起来
 - 因为我们不关系错误类型，且想把 map 处理后的数组返回给新数组，所以最后使用  `?`  解包 

```rust
// [255, 1, -1]
fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
	let arr_ = arr.iter().map(|&d| {
		if d < 0 || d > 255 {
			return Err(IntoColorError::IntConversion);
		} else {
			return Ok(d as u8);
		}
	}).collect::<Result<Vec<u8>, _>>()?;
	
	Ok(Color {
		red: arr_[0],
		green: arr_[1],
		blue: arr_[2],
	})
```

#### smart pointers

##### Box
- At compile time, Rust needs to know how much space a type takes up. This becomes problematic for recursive types, where a value can have as part of itself another value of the same type. 
- To get around the issue, we can use a `Box` - a smart pointer used to store data on the heap, which also allows us to wrap a recursive type.

```rust
Solution ✅
#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

```rust
pub fn create_empty_list() -> List {
    List::Nil // 空列表
}

pub fn create_non_empty_list() -> List {
    let tail = List::Cons(42, Box::new(List::Nil)); // 非空 List
    List::Cons(23, Box::new(tail)) // 创建头部节点，并连接到尾部
}
```

##### Arc
 - 使用 Arc（原子引用计数）在多个线程之间安全共享一个数字向量
```rust
fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}

```
 - `let child_numbers = ...`: 在循环内部，创建新的 Arc（`child_numbers`），它是 `shared_numbers` Arc 的 clone , 每个线程需要自己的 Arc，因为不安全的是在多个可能并发修改的线程之间共享单个 Arc , `Arc::clone` 方法增加了引用计数，允许多个线程安全地共享对相同数据的只读访问
 - `joinhandles.push(thread::spawn(...))`: 这一行生成一个新线程，并且传递给` thread::spawn`的闭包包含将在该线程中运行的代码。闭包捕获了`child_numbers` Arc，确保每个线程都在自己的向量克隆上操作

#### Thread 

使用`thread::spawn`来创建线程时，它返回一个`JoinHandle`，你可以使用它来等待线程完成并获取其返回值。在你的代码中，你可以修改`for`循环来收集线程的返回值，然后使用`join`方法等待它们完成。

##### Basic 

```rust
fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        let result = handle.join().expect("Thread panicked!");
        results.push(result);
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
```

##### Mutex

Rust中如何在多个线程间正确共享和更新数据:
- 使用`Mutex`把共享的`JobStatus`包起来,以协调多个线程之间的访问。
- 每个线程在更新值之前获取锁。
- 主线程用`join()`等待所有线程完成。
- 最后在所有线程都结束后打印更新后的最终值。

这样可以确保从多个线程正确地更新共享的值。线程之间通过 mutex 的锁进行同步。

如果没有mutex,线程会竞争去更新这个值,可能导致更新不一致。通过在更新之前获取锁,可以避免这个竞争条件。

通过 mutex 的锁机制可以协调多个线程之间对共享数据的访问,确保线程安全。

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
  jobs_completed: u32,
}

fn main() {
  let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
  
  let mut handles = vec![];
  for _ in 0..10 {
    let status_shared = Arc::clone(&status);
    let handle = thread::spawn(move || {
      thread::sleep(Duration::from_millis(250));
      
      // Acquire lock before updating shared status
      let mut status_mutex = status_shared.lock().unwrap();
      status_mutex.jobs_completed += 1;
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  // Print final updated value
  let status = status.lock().unwrap();
  println!("jobs completed {}", status.jobs_completed);
}
```

##### Mutex mpsc..?

使用多线程和通道来处理一个队列并进行数据传输，最终验证是否成功接收了所有数据

- `Queue` 结构体表示一个队列，其中包含一个长度字段和两个包含整数的向量 `first_half` 和 `second_half`
- `send_tx` 函数接受一个 `Queue` 和一个 `Arc<Mutex<mpsc::Sender<u32>>>`，它用于将数据发送到通道。该函数将 `Queue` 的两个向量中的数据发送到通道中。
	- `Arc<Mutex<mpsc::Sender<u32>>>`：这是一个包装了发送者通道的 `Arc`，它允许多个线程共享并安全地访问发送者通道


#### Macro 

```rust
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
```



#### Other small tips

**as ref:**
```rust
fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len()
}
❌ ^ no implementation for `f64 / usize`
```
- Error reason: `values.len()` is usize
- ✅ fix:   total / values.len() **as f64**


**&str.split()**
```rust
fn from(s: &str) -> Person {
	if s.is_empty(){ return Person::default()}
	let parts = s.split(",");
	let (name, age) = parts;

❌ Split<'_, &str> , found `(_, _)`
✅ Solution ： 
  let parts: Vec<&str> = s.split(",").collect();
```
 - Error reason: `split` 返回的是一个 Iterator
 - ✅ fix:  


**&str.parse()**
 - 需求：将 "26"  这个 `&str` 转化为 `uszie` number
```rust
let age = age_str.parse::<usize>()?;
❌ ^ cannot use the `?` operator in a method that returns `Person`
```
 - Error reason: `parse::<usize>()` 返回的是一个 `Result` , 需要对 `Ok()` 进行处理
 - fix 
	 - 法 1:  使用 match 的` Ok(age) , Err(_)` 分支处理
	 - 法 1:  使用 `.ok().unwrap_or_else`

```rust
✅ Solution-1
	let age = match age_str.parse::<usize>() {
		Ok(age) => age,
		Err(_) => {
			return Person::default();
		}
	};

✅ Solution-1
let age = age_str.parse::<usize>().ok().unwrap_or_else(|| Person::default().age);
```

元组解包 `let (a, b) = (..)` : 
```rust
let name = parts[0];
let age_str = parts[1];


// ✅ 语法优化：
let (name, age_str) = (parts[0].trim(), parts[1].trim());
```



```rust
#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    let z = &mut x;
    *y += 100;
    *z += 1000;
    assert_eq!(x, 1200);
}

============================>

#[test]
fn main() {
    let mut x = 100;
    {   
        let y = &mut x;
        *y += 100;
    }
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
```

------

```rust

fn main() {
    let data = "Rust is great!".to_string();
    get_char(&data);
    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
```
- 仔细观察 `&` 和 `mut` 所在的位置
- 犯了低级错误：(mut data: &String) ❌
- 犯了低级错误：(&data: String) ❌


-----

```rust
# enum
#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
// Output:
// ====================
// Quit
// Echo
// Move
// ChangeColor
```
 - enum 里的东西仔细看，Quit、Echo 、Move.. 现在都不需要明确定义。


```rust
# 复杂枚举类型：
enum Message {
    Quit,
    Move { x: i32, y: i32 },  // 移动
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
 - `Quit` 是一个不携带任何额外数据的 variant
 - Move 结构体， 注意不是 `Move : {x, ..}`  <-  没有 `Move :` 的那个冒号！！
 - `Write` variant 携带了一个 `String` 类型的数据
 - `ChangeColor` 是一个元组结构体




[^1]: move 语义
