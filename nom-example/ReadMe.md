# nom を使ってみる

![nom](https://user-images.githubusercontent.com/49807271/216722464-e1727857-6de4-403a-a674-4e547bbb36e5.png)

rust で parser として有名な nom を使ってみます。  
コンビネーターというアプローチを取っているパーサーです。  
単純なパーサーを組み合わせて、複雑なパーサーを組み立てられるように設計されています。  
１つ１つがシンプルなパーサーになるので、各パーサーのテストが容易かつ再利用性が高くなります。

https://github.com/rust-bakery/nom
