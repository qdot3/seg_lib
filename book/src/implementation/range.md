# 区間の取り扱い

[`RangeBounds<T>`](https://doc.rust-lang.org/std/ops/trait.RangeBounds.html#implementors)トレイトは[`Range<T>`](https://doc.rust-lang.org/std/range/struct.Range.html)をはじめとするすべての区間に実装されています。
これを活用すると配列への柔軟なアクセスを実現できます。
とくに、`range_query(start.., /* something */)`で`start`から末尾までの区間に対する演算結果を求めるようにできます。

~~~admonish info
Rustの`impl Trait`はコンパイル時に具体的な型に置き換わります。
そして、具体的な[`Bound`](https://doc.rust-lang.org/std/ops/enum.Bound.html)に対する`match`式はゼロコストです。
結果として、区間の抽象化コストはゼロになります。
~~~

## オーバーフロー

各セグメントに対応する区間は半開区間$[l, r)$で表現するのが自然ですが、$(l, r]$などが与えられた場合には、端点に$1$を足す必要があります。
このとき、オーバーフローが発生する可能性があります。

配列の容量の最大値は`isize::MAX`なので、implicit[^implicit]な実装の場合は`panic!()`するべきです。
`isize::MAX`よりも大きなインデックスを用いること自体がバグだからです。

動的木の場合は上記のような理由付けはできません。
受け取る区間を`Range<usize>`に限定するか、ドキュメントで注意を促すなどの方法が考えられます。

[^implicit]: セグメント木を配列に格納し、インデックスを通じて**暗黙的**に親子関係を表現している、という意味。
