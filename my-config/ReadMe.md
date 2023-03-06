## 個人的な Cargo のグローバルコンフィグ

以下に config.toml をコピーする。

```
C:\Users\user\.cargo
```

- プロジェクト作成時にバージョンコントロールシステムファイルが作成されないようにします

```
[cargo-new]
vcs = "none"
```

- Cargo の出力詳細を有効化します。  
  慣れたら false にする予定。

```
[term]
verbose = true        # whether cargo provides verbose output
```

尚、ローカルプロジェクトに config.xml を配置することで、config はオーバーライドされます。
