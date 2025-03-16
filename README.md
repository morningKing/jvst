# JVST - Java类文件解析工具

JVST是一个用Rust语言编写的轻量级Java类文件(.class)解析工具，可以帮助开发者分析和查看Java类文件的内部结构。

## 功能特性

- 支持解析标准Java类文件
- 可以读取文件系统、JAR文件和ZIP文件中的类
- 显示类文件的魔数、版本号、常量池、访问标志等信息
- 解析类的字段和方法信息
- 便捷的命令行界面

## 安装

### 前提条件

- Rust环境 (1.40.0或更高版本)
- Cargo包管理器

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/yourusername/jvst.git
cd jvst

# 编译项目
cargo build --release

# 可执行文件将在target/release目录下
```

## 使用方法

JVST提供了以下命令：

```bash
# 显示版本信息
jvst version

# 显示帮助信息
jvst help

# 显示指定类文件的绝对路径和结构
jvst cp <类路径> <类名>
```

### 示例

```bash
# 查看当前目录下的HelloWorld.class文件
jvst cp ./ HelloWorld.class

# 查看JAR文件中的类
jvst cp path/to/example.jar com/example/Main.class

# 查看ZIP文件中的类
jvst cp path/to/archive.zip SomeClass.class
```

## 项目结构

```
src/
├── main.rs           # 程序入口
├── cmd.rs            # 命令处理
└── cmd/
    ├── classfile.rs  # 类文件结构定义
    ├── classpath.rs  # 类路径处理
    ├── visit.rs      # 文件访问功能
    ├── classfile/    # 类文件各组件处理模块
    └── ...
```

## 主要模块说明

- **main.rs**: 程序入口点，处理命令行参数
- **cmd.rs**: 定义和处理命令行命令
- **classpath.rs**: 处理Java类路径（包括引导路径、扩展路径和用户路径）
- **visit.rs**: 提供文件系统和压缩文件访问功能
- **classfile.rs**: 定义Java类文件的结构和解析方法

## 贡献

欢迎提交问题报告和功能请求！如果您想贡献代码：

1. Fork项目
2. 创建您的特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交您的更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启一个Pull Request

## 许可证

本项目采用MIT许可证 - 详情请参见 [LICENSE](LICENSE) 文件。

## 作者

- Jay Kim - <18672780768@163.com>

---

*这个工具仍在积极开发中，欢迎提出建议和反馈！* 