- [Rust ツアー - Let's go on an adventure!](https://tourofrust.com/chapter_2_ja.html)

## 1

- なんで Rust が愛されてるか、よくわかった。
- 低級レイヤー向けの言語なのに、書き方がすごいモダン。2020 年プログラミングな感じ。TS 的。

## 2

- for がモダン。イテレーターな for。
- パターンマッチがあるーーー。超いいな。
- loop が関数的に使えて、かつ変数代入できて、break の値が返り値で使える。

## 3

- ブロック記法を変数代入できる！！！if/else とか、match とか。ruby/elixir 的。
- struct = interface
- method 呼び出し。`::` = static method / `.` = instance medhod。
- データ保存場所は３つ：data memory / stack memory / heap memory

## 4

- generic は turbofish にて型宣言
- Rust には null がない。struct で None を定義はできるで。
- モナドやん。
- Option 型ってので、None か Some(T)かの maybe 的なのを作れる。
- Result 型ってので、Ok(v)か Err(e)を取れる。
- Vector は variable な Array

## 5 所有権

- Rust は GC がないけど、owner ship がある
- 代入、関数の引数で使う、などをすると ownership が drop して使えくなる
- `&mut`：所有権の借用
- `*f`：値のコピー（所有元は共有しない）
- rust は、1mutable ref か multi non-mut ref のどっちかしか許容しない。これにより data race しない
- ref は owner よりも長生きしない
