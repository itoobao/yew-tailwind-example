# BaseWebWasm
web服务器基于Axum, 前端页面基于Yew的基础模版
工具安装:
cargo install wasm-pack
cargo install cargo-make
cargo install simple-http-server # 提供 assets 服务的简单服务器

## web-server
默认端口:8081

## web-wasm
默认端口:8080
在Makefile.toml中修改

1、编译
cargo make build

2、启动服务
cargo make serve

## 生成css
css使用tailWindCss框架，postcss
1、安装依赖:
npm install

2、重新构建使用命令:
npm run build-css