# 20240517【实例2：构建大型工程 layout】 

# 结构
```
[root@iZ7xv7ktt0bfa251r2jr2vZ layout]# tree
.
├── Cargo.lock
├── Cargo.toml
├── depend
│        ├── Cargo.toml
│        └── src
│            └── main.rs
├── readme.md
├── src
│        └── main.rs
└── utils
    ├── Cargo.toml
    └── src
        └── lib.rs

```

## 步骤
### 创建 layout
```
cargo new layout
cd layout
vim Cargo.toml
添加：
[workspace]
members = ["depend", "utils"]
```
### 创建依赖文件 depend
在 layout 目录下执行
```
cd layout
cargo new depend
```

### 创建库文件 utils
在 layout 目录下执行
```
cd layout
cargo new utils --lib
```

### 设置 layout 的工作空间 workspace
```
[workspace]
members = ["depend", "utils"]
```

### 添加库方法 utils
vim layout/utils/src/lib.rs
```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tools_tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

### 依赖代码 depend 中调用库文件
在 depend 的 Cargo.toml 文件中添加依赖：vim layout/depend/Cargo.toml
```
[dependencies]
utils = { path = "../utils" }
```

### 依赖代码 depand 中执行 utils 库函数 add
vim layout/depend/src/main.rs
```
use utils;

fn main() {
    println!("=== Hello, depend! ===");
    let r = utils::add(2, 3);
    println!("num = {}, r = {}", num, r);
} 
```

### 构建依赖代码 depend
```
cd layout
cargo run -p depend
```



