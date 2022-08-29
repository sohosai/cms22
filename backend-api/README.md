# CMS22 Backend API

Strapiとフロントエンドの間で機能するゲートウェイです。

## Requirements
- Rust

## Run
- 実行時に、雙峰祭オンラインシステムからのエクスポートデータを指定する必要があります
	- 企画や権限の照会のために使用されます。

設定は以下のように記述します。

```.envrc
export STRAPI_BASE=http://localhost:1337
export SOS_USERS_CSV=data/sos-users.csv
export SOS_PROJECTS_CSV=data/sos-projects.csv
export STRAPI_TOKEN=paste your token here
export RUST_LOG=info
```


