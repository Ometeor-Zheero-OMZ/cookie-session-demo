# Cookie と Session の練習

## Cookie・Session とは

Cookie: データをブラウザに保存する仕組み
Session: ユーザーが行う一連の動作（ログイン～ログアウト）

Cookie を使って Session 管理を行うことで、HTTP 通信の「ステートレス」
という性質を克服して、状態を保持できるようになる（疑似的にステートフルにすることができる）

## Cookie と Session はなぜ必要か？

HTTP がステートレスという性質を持っているから

サーバーがクライアントの状態を保持するには DB にデータ保存するか、
Cookie や Session を使ってメモリ上にデータを保持する必要がある。

HTTP の「ステートレス性」を補填して状態を保持させるために生まれた技術が「Cookie」と「Session」

## Cookie と Session の仕組み

サーバーはアクセスしてきたユーザーに Session ID を付与する
クライアントはブラウザの Cookie にその Session ID を保持する
クライアントは次にリクエストを送るときに、その Session ID をサーバーに渡す
サーバーはその Session ID によって「誰からのアクセスか」を認識できる
サーバーは Session ID に紐づいた情報を Session 情報保管場所（データベースなど）に保存する

以上の手順で Session（ユーザーが行う一連の動作）管理を行うことで、HTTP のステートレス性を克服して、状態を保持することができる。
