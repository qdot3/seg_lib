# 定義済み演算

```admonish warning
この章は書きかけです
```

`seg_lib`を利用するたびに`Monoid`トレイトなどを実装するのは面倒です。
そこで、典型的な二項演算について、モノイドのテンプレートを用意しました。
定義済み演算の一覧は[ドキュメント](https://docs.rs/seg_lib/latest/seg_lib/ops/index.html#structs)から確認できます。

## 加法モノイドのテンプレート

~~~admonish example title="実装例（加法モノイド）"
```rust, no_run
{{ #include ../../src/ops/add.rs:def_and_impl_monoid }}
```

例えば、`Add<i32>`はモノイド`(i32, +, 0)`に対応します。

トレイト境界のとり方には他にもいくつかのパターンが考えられます。

- `T: Copy + Zero + std::ops::Add<Output = T>`
- `T: Clone + Zero + std::ops::Add<Output = T>`

これらは、`combine()`の挙動に影響を与えます。
最適な実装は型に依るため、ユーザーに定義してもらうことにしました。
~~~

```admonish tip
`i32`に`Monoid`トレイトを直接実装してはいけません。
例えば、`(i32, +, 0)`と`(i32, *, 1)`が共存できないためです。
```

## モノイド作用について
