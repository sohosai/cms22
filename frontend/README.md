# 概要

フロントエンドは Vue3 + TypeScript を用いて開発しております。

# 開発環境立ち上げ

```bash
yarn
yarn dev
```

# コードチェック

```bash
yarn lint
yarn format # 自動整形は format:fix
```

# Auditor (監査人) の有効化

提出されたコンテンツを審査するためには、auditor 権限が必要です。以下では、auditor 権限を付与する方法を説明します。

## Firestore に auditor 権限をもつユーザのメールアドレスを記録する

1. Firestore のローカルエミュレーターコンソール (http://127.0.0.1:4000/firestore) にアクセスし、`system/authority` を作成します。

2. ドキュメントに、以下のように自分のメールアドレスを追記します。  
   admins: ["my-email@example.com"]  
   auditors: ["my-email@example.com"]

3. `/reflect-authority` にブラウザでアクセスすると権限が反映されます。
