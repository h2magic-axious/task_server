# 数据库迁移命令
## 1 安装sqlx-cli工具

```shell
cargo install sqlx-cli
```

## 2 创建数据库
```shell
sqlx database create --database-url postgres://postgres:123456@192.168.1.101:5432/task_server
```

## 3 创建迁移文件
```shell
sqlx migrate add tasks --database-url postgres://postgres:123456@192.168.1.101:5432/task_server
```
会在当前目录下新建目录: migrations, 其下会生成一个sql文件, 然后在其中写入sql脚本. 其中add参数后的字段是生成迁移文件的名字

## 4 执行迁移文件
```shell
sqlx migrate run --database-url postgres://postgres:123456@192.168.1.101:5432/task_server
```