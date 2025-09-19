# 区間の抽象化

[`RangeBounds<T>`](https://doc.rust-lang.org/std/ops/trait.RangeBounds.html#implementors)トレイトは[`Range<T>`](https://doc.rust-lang.org/std/range/struct.Range.html)をはじめとするすべての区間に実装されています。
これを活用すると配列への柔軟なアクセスを実現できます。
とくに、`range_query(start.., /* something */)`で`start`から末尾までの区間に対する演算結果を求めるようにできます。

~~~admonish info
Rustの`impl Trait`はコンパイル時に具体的な型に置き換わります。
そして、具体的な[`Bound`](https://doc.rust-lang.org/std/ops/enum.Bound.html)に対する`match`式はゼロコストです。
結果として、区間の抽象化コストはゼロになります。
~~~

~~~admonish example collapsible=true title="Compiler Exploderでの実験"
`rustc1.89.0`で実験した。
`range`の方はコンパイル時に解決され、具体的な型に置き換えられる。
具体的な型に対して最適化ビルドすると、無意味な`match`式は削除される。

```rust
use std::ops::{Range, RangeBounds, RangeToInclusive};

pub fn convert_range(range: RangeToInclusive<usize>) -> Range<usize> {
    let start = match range.start_bound() {
        std::ops::Bound::Included(start) => *start,
        std::ops::Bound::Excluded(start) => start
            .checked_add(1)
            .expect("starting point of the given range is less than `usize::MAX`"),
        std::ops::Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        std::ops::Bound::Excluded(end) => *end,
        std::ops::Bound::Included(end) => end
            .checked_add(1)
            .expect("end point of the given range is less than `usize::MAX`"),
        std::ops::Bound::Unbounded => 0,
    };

    start..end
}
```

```asm
convert_range:
        cmp     rdi, -1
        je      .LBB0_2 // expectでerrorになると、panic処理にジャンプする
        inc     rdi
        xor     eax, eax
        mov     rdx, rdi
        ret
.LBB0_2:
        push    rax
        lea     rdi, [rip + .Lanon.d35b42e3d053be306c0ef31fbc2b70b5.0]
        lea     rdx, [rip + .Lanon.d35b42e3d053be306c0ef31fbc2b70b5.2]
        mov     esi, 54
        call    qword ptr [rip + core::option::expect_failed::hfe7afbd436ce9c45@GOTPCREL]

.Lanon.d35b42e3d053be306c0ef31fbc2b70b5.0:
        .ascii  "end point of the given range is less than `usize::MAX`"

.Lanon.d35b42e3d053be306c0ef31fbc2b70b5.1:
        .asciz  "/app/example.rs"

.Lanon.d35b42e3d053be306c0ef31fbc2b70b5.2:
        .quad   .Lanon.d35b42e3d053be306c0ef31fbc2b70b5.1
        .asciz  "\020\000\000\000\000\000\000\000\026\000\000\000\016\000\000"
```
~~~

## オーバーフロー

各セグメントに対応する区間は半開区間$[l, r)$で表現するのが自然ですが、$(l, r]$などが与えられた場合には、端点に$1$を足す必要があります。
このとき、`usize::MAX + 1`でオーバーフローが発生する可能性があります。

配列の容量の最大値は`isize::MAX`なので、implicit[^implicit]な実装の場合は`panic!()`するべきです。
`isize::MAX`よりも大きな`usize::MAX`をインデックスを用いること自体がバグだからです。

動的木の場合は上記のような理由付けはできません。
受け取る区間を`Range<usize>`に限定するか、`usize::MAX`要素目を特別扱いするなどの方法が考えられます。

[^implicit]: セグメント木を配列に格納し、インデックスを通じて**暗黙的**に親子関係を表現している、という意味。
