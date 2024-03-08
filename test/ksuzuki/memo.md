## 仕様

### 概要

n-puzzleを解くプログラム。

n-puzzleの例

https://n-puzzle.baletskyi.me/

### input

- 解くパズル
    - .txtのテキストファイル
    - 数字のみ（ランダムにパズルを生成して解く）
- heuristic function
    - Manhattan-distance heuristic
    - other 2 functions
- アルゴリズム
    - A*探索アルゴリズム
    - 均一コスト探索
    - 貪欲法

テキストファイルの要件

- `#`に続く文字はコメント
- 以下コメントと空白行（空白のみの行も含む）を除外して考える
- 1行目はn x nのnが書かれている
    - 現実的な数字出ない場合アウトにする
- 2行目以降はパズルの内容が書かれている
- パズルは空白区切り（改行含む任意の空白。パズルの前後も可）
- 0の位置は何もない状態
- 値は0から始まる連番になっている

例

```
3
3 2 6
1 4 0
8 7 5
```

パズルの要件

- サイズは2以上
- コード内で定められている値より大きなサイズは受け付けない

### output

解くパズルを数字のみで与えられた場合、最初に解くパズルを表示する。

プログラムの終了時に以下を出力する

- パズルが解けない場合は解けないという
- パズルが解ける場合は以下を伝える
    - complexity in time: 考慮された状態の総数
    - complexity in size: 同時に探索された状態の総数
    - 探索によって導かれた、最初の状態から最終状態までに必要な移動回数
    - 探索によって導かれた、最初の状態から最終状態までに移動する状態のシーケンス
        - 移動内容のみ？
        - 状態全体？

### 完成形例

```
1 2 3
4 5 6
7 8 0
```

## 参考

- n-puzzleについて
    - https://manabitimes.jp/math/979
    - https://y-uti.hatenablog.jp/entry/2015/04/29/103422
- A-star
    - https://ja.wikipedia.org/wiki/A*
    - http://tsumulab.org/papers/pdf/degree/2009_bachelor_oozaki.pdf
- 均一コスト探索
    - https://ja.wikipedia.org/wiki/%E5%9D%87%E4%B8%80%E3%82%B3%E3%82%B9%E3%83%88%E6%8E%A2%E7%B4%A2
- 貪欲法
    - https://ja.wikipedia.org/wiki/%E8%B2%AA%E6%AC%B2%E6%B3%95
    - https://algodaily.com/lessons/getting-to-know-greedy-algorithms-through-examples

## n-puzzleの不可能な配置の証明

以下を示す。

```
8パズルが完成できる  
⟺  
s を t にする置換のパリティ（偶奇）と「空き」の最短距離の偶奇が等しい。  
```

```
8パズルが完成できる  
ならば  
s を t にする置換のパリティ（偶奇）と「空き」の最短距離の偶奇が等しい
```
はわかる。

※ 置換のパリティは変わらないことは証明略

```
s を t にする置換のパリティ（偶奇）と「空き」の最短距離の偶奇が等しい
ならば、
8パズルが完成できる
```

については、nパズルがこの場合に実際に解けることをもって、証明とする。
