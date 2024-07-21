# Cookie と Session の練習

## Cookie・Session とは

Cookie: データをブラウザに保存する仕組み
Session: ユーザーが行う一連の動作（ログイン～ログアウト）

Cookie を使って Session 管理を行うことで、HTTP 通信の「ステートレス」
という性質を克服して、状態を保持できるようになる（疑似的にステートフルにすることができる）
