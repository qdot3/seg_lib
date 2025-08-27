# モノイド作用

遅延伝搬セグメント木ではいくつかの要素をまとめて更新します。
区間クエリに対応するモノイドを$(Q, \cdot_Q, e_Q)$、更新クエリに対応するモノイドを$(U, \cdot_U, e_U)$とかきます。
遅延伝搬セグメント木が正しく動作するためには「いくつかの要素をまとめて更新する」ことで「各要素をそれぞれ更新した結果」を再現できなければなりません。

```admonish note title="定義（モノイド作用）"
区間クエリに対応するモノイドを$(Q, \cdot_Q, e_Q)$、更新クエリに対応するモノイドを$(U, \cdot_U, e_U)$とする。
$* \colon U \times Q \longrightarrow Q$が下記を満たすとき、これをモノイド作用という。

- $\forall u_1, u_2 \in U, \forall q \in Q, (u_1 \cdot_U u_2) * q = u_1 * (u_2 * q)$
- $\forall u \in U, \forall q_1, q_2 \in Q, (u * q_1) \cdot_Q (u * q_2) = u * (q_1 \cdot_Q q_2)$
```

~~~admonish example title="実装例（モノイド作用）"
```rust, no_run
{{ #include ../../../src/traits.rs:monoid_action_trait }}
```
~~~

## 区間幅の利用
