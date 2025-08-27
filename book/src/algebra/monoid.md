# モノイド

```admonish note title="定義（モノイド）"
$S$を集合、$\cdot \colon S \times S \longrightarrow S$を二項演算とする。

- （　結合法則　）$\forall a, b, c \in S, (a \cdot b) \cdot c = a \cdot (b \cdot c)$
- （単位元の存在）$\exist e \in S, \forall a \in S, a \cdot e = e \cdot a = a$

上記の2つが成り立つとき、組$(S, \cdot, e)$をモノイドという。
```

セグメント木にはモノイドが乗りますが、これは十分条件です。
セグメント木には半群も乗ります。

```admonish note title="定義（半群）"
$S$を集合、$\cdot \colon S \times S \longrightarrow S$を二項演算とする。

- （　結合法則　）$\forall a, b, c \in S, (a \cdot b) \cdot c = a \cdot (b \cdot c)$

上記が成り立つとき、組$(S, \cdot)$を半群という。
```

セグメント木に半群$(S, \cdot)$を乗せる場合、未初期化のノードは`None`で表現することになります。
すなわち、ノードには`Option<S>`が格納されます。
`Option<S>`上の二項演算を以下で定義すると、半群をモノイドに拡張できます。

```rust, no_run
fn binary_operation<S>(lhs: Option<S>, rhs: Option<S>) -> Option<S> {
    match (lhs, rhs) {
        (Some(lhs), Some(rhs)) => unimplemented!("lhs ⋅ rhs"),
        (Some(lhs), None) => Some(lhs),
        (None, Some(rhs)) => Some(rhs),
        (None, None) => None
    }
}
```

このようにモノイドと半群の区別はあまり重要ではありません。
実装上単位元があると便利なので、`seg_lib`ではモノイドを採用しました。

~~~admonish example title="実装例（モノイド）"
```rust, no_run
{{ #include ../../../src/traits.rs:monoid_trait }}
```
~~~

## おまけ

セグメント木には半群でない代数的構造も乗ります。
問題にしている範囲で半群のように振舞えば何でもよいのです。

```admonish note title="主張（セグメント木に乗る代数的構造）"
- 隣り合う2つのノードの間に演算が定義されていること
- 連続する3つ以上のノードの内、どの隣り合うノードから計算しても、結果が変わらないこと

あるデータ列に対して上記が成り立つとき、これはセグメント木に乗る。
とくに、隣り合わないノードの間に演算が定義されていなくともよい。
```
