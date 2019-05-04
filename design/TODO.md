# TODO
- 命名規約
  - sprite
    - background
      - 動かない

    - enemy
      - 動く

    - player
      - 動く
      - 操作可能
      
  - map を使ってあたり判定ができるものを、MapSprite とする？

- 画像サイズを 64 [px] ベースで書く

- あたり判定の実装 
  - background とのあたり判定
    - map の接触部との判定をすれば OK
    
  - enemy とのあたり判定
    - 判定ロジック
      - 4分木分割

- 重力の実装

- tsv ファイルから、background, player, enemy を作成する
  - ファイル読み込みは一回で抑える

- リソースの読み込み
  - stage の開始時に、画像と音楽を読み込むか？
  - scene の開始時に、画像と音楽を読み込むか？
  - sprite のデータ(toml)はいつ読み込むか？
    - リリースビルド時に変数の埋め込みは可能か？
  - 画像と効果音は起動時にロードしておく？
    - あと toml 設定系
    - bgm は重いから、都度ロード 

- 定数値の持ちまわし
  - セルサイズとか、プレイヤーサイズとか
  - GameContext みたいなのを作って、ggez の Context と一緒に入れる？
  - 画像、toml 設定系も持ちまわしにするか？

- Player, PlayerController, PlayerDrawer に分けるか？

- toml 設定系は開発効率を上げるために必須かもしれない
  - 他の項目は後からやれば OK
