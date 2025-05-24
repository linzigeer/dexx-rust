#!/bin/bash

echo "🚀 Dexx Rust 项目快速启动脚本"
echo "=================================="

# 检查当前目录
if [ ! -f "Cargo.toml" ]; then
    echo "❌ 错误: 请在项目根目录运行此脚本"
    echo "正确路径: /home/oscar/projects/dexx-rust"
    exit 1
fi

echo "📁 当前目录: $(pwd)"

# 🚨 关键路径验证
echo ""
echo "🚨 路径验证检查:"
echo "----------------"

# 验证Rust项目路径
if [ "$(pwd)" = "/home/oscar/projects/dexx-rust" ]; then
    echo "✅ Rust项目路径正确: $(pwd)"
else
    echo "❌ 警告: 当前路径不是标准Rust项目路径"
    echo "   当前: $(pwd)"
    echo "   期望: /home/oscar/projects/dexx-rust"
fi

# 验证Go项目存在
if [ -d "/home/oscar/go-projects/src/goldenshovel-server" ]; then
    echo "✅ Go参考项目存在: /home/oscar/go-projects/src/goldenshovel-server"
else
    echo "❌ 警告: Go参考项目不存在"
fi

echo ""

# 显示项目状态
echo "📋 项目状态检查:"
echo "----------------"

# 检查Rust环境
if command -v cargo &> /dev/null; then
    echo "✅ Cargo: $(cargo --version)"
else
    echo "❌ Cargo 未安装"
    exit 1
fi

# 检查编译状态
echo "🔧 编译检查..."
if cargo check --quiet; then
    echo "✅ 编译成功"
else
    echo "❌ 编译失败，请检查代码"
    exit 1
fi

# 显示重要文件
echo ""
echo "📄 重要文件:"
echo "------------"
echo "✅ README.md - 项目概述"
echo "✅ DEVELOPMENT_STATE.md - 详细开发状态"
echo "✅ PATH_RULES.md - 路径管理规则 🚨"
echo "✅ config.yaml - 配置文件"
echo "✅ Cargo.toml - 依赖配置"

# 显示路径提醒
echo ""
echo "🚨 路径管理提醒:"
echo "----------------"
echo "Go项目 (参考): /home/oscar/go-projects/src/goldenshovel-server"
echo "Rust项目 (开发): /home/oscar/projects/dexx-rust"
echo "⚠️  永远使用绝对路径！详见 PATH_RULES.md"

# 显示下一步
echo ""
echo "🎯 下一步开发建议:"
echo "------------------"
echo "1. 查看路径规则: cat PATH_RULES.md"
echo "2. 查看开发状态: cat DEVELOPMENT_STATE.md"
echo "3. 参考Go代码: ls /home/oscar/go-projects/src/goldenshovel-server/internal/"
echo "4. 开始实现: /home/oscar/projects/dexx-rust/src/models/user.rs"
echo "5. 启动服务器: cargo run"
echo "6. 测试API: curl http://localhost:8902/"

echo ""
echo "🔄 常用命令:"
echo "------------"
echo "cargo check    # 编译检查"
echo "cargo run      # 启动服务器"
echo "cargo test     # 运行测试"
echo "cargo fmt      # 格式化代码"
echo "cargo clippy   # 代码检查"

echo ""
echo "✨ 准备就绪! 记住使用绝对路径!"
