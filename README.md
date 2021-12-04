# rust-v8-demo

## 构建
请不要直接使用 `cargo build` 构建（Macos x86_x64），这样要编译 v8 非常慢，直接使用 `lib` 目录下已经编译好的成果就可以了

执行以下命令：
```shell
# 构建并运行
./build.sh run

# 仅构建
./build.sh
```