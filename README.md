
## 教材


## 記事
- [Rust未経験者が業務でRustを使えるようになるまで - CADDi Tech Blog](https://caddi.tech/archives/661)

## Rustを学習してのファーストインプレッション

- ポジティブ
  - モナド、パターンマッチなど関数型の思想を色濃く持っている。


- ネガティブ
  - いくつか直感的に好みじゃない書き方はある
    - `map(|x| x.id)`。RubyLikeなシンタックス。JSなシンタックスのほうが好み(`map(x => x.id`)
    - shadowing：letなんどもするので、キモい。すでにletしていることに気づかなくて、再letしそうな気がする。とはいえ、shadowingの代替としてはパイプラインになるけれども、ElixirWayのパイプラインも微妙だし、JSのproposalにあがってる？プレースホルダーを使ったパイプラインも複雑になりそうなので、良い代替案を知らない。
    - スペース４つ。スペースなのは良い。ただ、４つもいらんで２つで良いだろ、スペース。宗教戦争じゃー！
    - 所有権が難しそう。というか、難しい。
    - if-let-else記法はちょっと微妙な感がある。構文が直感的でなくてそこまで文がショートになってない感が。。。

- 難しい
  - 所有権。概念が新しい。ルールが難しい。デザインパターンがある。（BoxとかRCとか
