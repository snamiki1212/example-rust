- [まえがき - The Rust Programming Language](https://doc.rust-jp.rs/book/second-edition/)
- 所感：この教材を読む前に、すでに２つの記事・教材を流し読みしてるから、読めているけれども、０−＞１でこの記事は辛い気がする、、、。１つのトピックでまるっと詰め込んで教えすぎてて、わけわかんなくなりそう。ある程度、知ってる人なら大丈夫だと思うけど、うーん。

## 1

- `cargo build`よりも`cargo check` でビルド可能か？は確認したほうが低コスト。

- クレートの英語的なニュアンス；https://ja.wikipedia.org/wiki/%E3%82%AF%E3%83%AC%E3%83%BC%E3%83%88_(%E7%AE%B1)

## 2

- 関連関数。オブジェクトのインスタンスではなくて、型に対して実装されている関数のこと。（＝Staticメソッド）
- &: 参照：引数が参照であることを表し、これのおかげで、データを複数回メモリにコピーせずとも、 コードの複数箇所で同じデータにアクセスできるようになるわけです

- シャドーイング：パイプラインが無い代わりに再束縛ができるので、同じ変数名で何度もletして再束縛すりゃいいのか。うーん、パイプラインのほうが美しいかな、、、。再定義して再代入してるみたいなんで、気持ち悪い。
  ```rust
  fn main() {
    let x = "OK";
    let x = x.to_string() + "_NG";
    let x = x.to_string() + "_OK";
    println!("{}", x);
    // => OK_NG_OK
  }
  ```

## 3

- mut vs shadowing
  - mut：型の変更ができない
  - shadowing：型の変更もできる
- signed/unsigned
  - i32 / f64 がとりあえずのベターな選択
- 配列とベクタ
  - 配列：固定長のケース（const系のみ）
  - ベクタ：それ以外の可変のすべてのケース。
- 式と文
  - 関数末尾にセミコロンがないとreturnされる。
  - 式は終端にセミコロンが、ない。
  - 文は終端にセミコロンが、ある。
  - 文は値を返さない。式は値を返す

## 4

### 所有権

動機：メモリ管理をしたい。
なんで？ヒープとスタックがある。
スタックは固定長、ヒープは可変長
プログラムでのリテラル、つまりハードコードされている文字列などはすべてスタックとして処理できる。
Rust的には、リテラル（＝プログラムに書かれている実際のstring/numなど）は、ビルドされた結果にバイナリにハードコードされる。そのため、スタックメモリで、固定長だけど速度が早い。
しかし、ユーザー入力などのInputな値などの変数はどれくらいの大きさの数が入るかわからない。つまり、どれくらいメモリ領域を確保すればよいかが事前にはわからない。そのため、ヒープメモリ、つまり可変長になる。

可変長なヒープメモリはポインタを使って複数のメモリ空間を横断してデータを保持する。そして、メモリ空間は任意のタイミングで掃除しないといけない。今までの言語では、GCにて定期的にクリーンにしていた。Rustでは所有権によってスコープ単位で不要になった変数、つまり使用しているメモリ領域をdropする。

Copyトレイトに適合している i32/bool/charなどはスタックメモリなので、別の変数に束縛してもムーブしない。

### スタックとヒープで挙動が違う

```rust
fn main() {
    let x = 2;
    let y = x;
    println!("{}, {}", x, y); // => 2, 2 
}
```

```rust
fn main() {
    let x = String::from("ok");
    let y = x;
    println!("{}, {}", x, y); // => Error 
}
```
すでにxのポインタ・長さ・許容量のメタデータをｙに「ムーブ」しているので。

### 参照渡し

```rust
let x = 2;
func1(x)
// or
func2(&x)

fn func1(x: i32) {
  // ...
}
fn func2(x: &i32) {
  // ...
}
```

- 借用＝関数の引数に参照を取ること。
- 借用ではイミュータブル。変更したいなら、`&mut i32` の宣言子がになる
- 借用では、所有権を持っているわけではない。

### スライス

- スライス＝可変配列
- 文字列スライス（&str）＝Stringへの参照（部分参照もできる
- 文字列リテラルは&str で、不変な場所の参照。

## 5

### 構造体

- 構造体_初期化_省略_記法
  - `User { name: name  } = User { name }`
- 構造体_更新_記法
  - `User { ..user1 }`
- タプル構造体
  - `Point(0, 1, 2)` / `Color(255, 255, 0)`

### impl

- implで構造体に関数を生やす。（ダックプログラミング）
- 規則：implの中のfnの第一引数は`&self`にする。これで構造体自体にアクセスできる。`self` / `&mut self`でもOK。
- 関数とメソッドは違う。関数は外界にあるfn、メソッドはimpl内にあるfnを指す。
- 関連関数：impl内に実装されてるけど、selfアクセスがない。

## 6

- enum列挙子
  - enum の列挙しているのは値。ただ、特有の値でstringとかではない。あくまで、同一enum同士で比較などができるだけぽい。
    ```rust
      #[derive(Debug)]
      enum Type {
        User,
        Admin
      }

      let t = Type::User;
      println!("{:?}", t); // => User
    ```
  - 列挙子として型も付与できる。型が違っててもOK。
    ```rust
      enum Type {
        yen(i32),
        daller(f32),
      }
    ```
  - enumで列挙子つきで、多種に定義することは、イコールstructで細かく定義してるのと同義。
  - enum にもimplでメソッドを定義できる
- match
  - rustにはnullがない。けれども、その概念は必要。それがOptionモナド。
  - モナドの処理をさばくときに、matchを使うことが多いよね。
  - matchはパターンが包括されている必要がある。Optionの場合は、Some/Noneの両方をアームで書かないとコンパイルエラー。
  - これで、match不足のミスをコンパイルレベルで解決できる。
- if_let_else記法
  - 特定のmatchのパターンに対する糖衣構文。
  - １つの値にしかmatchしないケース⇒if_let記法
  - １つの値へのmatchか、それ以外のケース⇒if-let-else記法
  
## 7
- module
  - `mod` でモジュール化する。
  - `mod xxx;` だと、xxx.rsを見に行く。xxx.rsではmodでwrapしないでOK。
  - `use`で使用する。
    - `super`で１つ上の階層にあがる。
  
- ファイル
  - `mod.rs` / `main.rs` / `lib.rs`

## 8

- Vector
  - feature
    - ヒープリスト
    - 同一の型を扱う。
    - 別の型を扱いたいなら、enumで定義したものを型として扱う
  - 作成
    - `let v: Vec<i32> = Vec::from([1, 2, 3]);`
    - = `let v = vec![1, 2, 3];` ( マクロが用意されてる。型も推論される。)
  - 更新
    - `let mut v = vec![1, 2, 3];`
    - `v.push(2);`
  - 取得
    - 添字で取るか、.getで取るか
    - `let v = v[2]; // => v = 2 or panic when compiling`
    - `let v = v.get(2); // => v = Option(2) or None`
  - 複数の型
    - enumで型を定義して、その型のベクタを作る。
    - `enum T { I(i32), F(f64), S(String)};`
    - `let v = vec![T::I(2), T::F(2.2), T::S(String::from("OK"))];`
- 文字列
  - Rust界隈での文字列は、Stringと&strのことを指す（すでにややこしい・・・）
  - Rustのコアにはstr/&strしかない。
  - Rustの標準ライブラリにStringがある
  - Rustの標準ライブラリにその他のOsString、CStringなどがある。
  - 規則：string＝所有権有り、str＝借用されたバージョン。
  - idxでアクセスは良くない。人間の可読文字数ではなくて、UTFの処理区分で区切られてるから。２つの配列要素をまたぐデータもあり、そこでデータを切ってアクセスするとpanicになる。
- ハッシュマップ
  - RustにおけるHashMapは連想配列のこと。
  ```rust
  use std::collections::HashMap;
  // 
  let mut map = HashMap::new();
  map.insert(key, value);
  map.get(key); // => Option(&value)

  // keyのvalueが存在するかのチェッカーがentryで、そのモナドな型のEntry
  map.entry(key) // Entry(...)
    .or_insert(value) // 値がなければ、これでvalueが挿入
  ```

## 9

- error
  - エラー
    - panic / Resultモナドのどちらか
    - matchガードと呼ばれる、match条件のenhanceな構文もあり。
    - matchの &は参照にマッチして、refは値にマッチしつつ値を返す
  - matchのネストが深くなることがあるので、便利な関数がある
    - `unwrap`：Okならそのまま、Errならpanicする。`fun.unwrap();`
    - `expect`：`unwrap`でエラーメッセージを引数に持たせる。あれ、unwrapいらなくない？禁止したほうが良さげ。
  - エラーの委譲(delegation)
    - 自分で作った関数の返り値を値ではなくてResultにして、実際の値はOkかErrを返すようにすればよい。
    - これを便利に書くための演算子が`?`。(ex) `let mut f = File::open("hello.txt")?`
    - これで、errorならquick return。Ok(v)ならvになる。

## 10
- trait
  - interface/abstractのようなもの。
  - インターフェース：`pub trait <trait> { fn <xxx> -> () {...} }`
  - 実装：`impl <trait> for <struct> {...}`
  - fnのオーバーライドが可能
- trait境界(trait bound)
  - ジェネリクスにトレイトを指定することで、「引数がそのトレイトを実装していないといけない」という制約ができる
  - （普通のジェネリクス：`fn foo<T>(item: T) -> T { return item; }`）
  - 普通の記法
    - trait境界：`fn foo<T: ThisIsTrait>(item: T) -> T { return item; }`
    - 記法：`<型変数: トレイト名>`、`<T: Display + Clone >`
  - where記法
    - `+`でトレイトをつなげたり、トレイと境界が多いと可読性が悪いので、そういう場合はwhere記法で記載する
    - 記法：`fn foo<T, U>(t: T, u: U) -> i32 where T: Display + Clone, U: Clone + Debug {...}`
  - impl時の型にてトレイトをしているすることで、トレイトごとに参照するメソッドを振り分けることができる
    ```rust
    struct Animal {}
    impl<T: Dog + Cat> Animal {
      fn walk(){...}
     }
    impl<T: Bird> Animal {
      fn fly(){...}
    }
    ```
- ライフタイム
  - いつまで、ある変数の参照・借用の権限を持っているか？をライフタイムとして管理する。
  - Rustコンパイラには、借用チェッカーがあり、これでライフタイムの有無がわかる
  - ジェネリックなライフタイム引数
    - いつ：引数が参照で返り値も参照な関数があり、それらに関係性がある場合はライフタイム引数が必要になる
    - なぜ：それらの関係性がわからないと、ライフタイムチェッカーが所有権・借用のライフタイムを理解できない。
    - 記法：`'`。`&'a i32` / `&'a mut i32` / `'static str`（変数はaから始まる。staticでスタックメモリな内容
    - ex: `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {...`
    - 引数のライフタイム＝入力ライフタイム。戻り値のライフタイム＝出力ライフタイム。
## 11
- test
  - `#[ここに注釈]`
  - `#[test]`
  - `#[cfg](test)`: configuration
- 単体
  - 同一ファイル内にtests modを作り、そこでテストをする
  - 非公開モジュールもテストできる。
- 結合
  - `/src/tests`のディレクトリを切る。
  - 個別のクレートになる。そのため、テストしたいmodは`extern crate クレート名`で宣言も必要。
  - 普通に命名してヘルパファイルを作成するとテスト対象になってしまう。`src/tests/common.rs`
  - モジュールの規約に従ってヘルパファイルを作成すると非テスト対象になる。`src/tests/common/mod.rs`
## 12
## 13
## 14
## 15
## 16
## 17
## 18
## 19

## 
次：https://doc.rust-jp.rs/book/second-edition/ch07-03-importing-names-with-use.html