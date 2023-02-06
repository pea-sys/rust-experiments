- derive ベースのパースを行いたい場合、cargo.toml で derive の指定が必要

```toml
[dependencies]
clap = {version = "4.1.4",features = ["derive"]}
```

基本的には derive の方が構造化された記述で分かりやすいし、バグも入り込みにくい。  
パース内容が複雑な場合は、何とも言えない。
