# 🚨 绝对路径提醒 - 每次开发必读！

## ⚠️ **极其重要的操作规则**

### **永远使用绝对路径进行文件操作！**

#### ❌ **错误示例**
```bash
cat src/main.rs                    # 可能在错误目录
vim src/models/user.rs             # 不知道在哪个项目
cargo build                       # 可能编译错误项目
```

#### ✅ **正确示例**
```bash
cd /home/oscar/projects/dexx-rust && cat src/main.rs
cd /home/oscar/projects/dexx-rust && vim src/models/user.rs  
cd /home/oscar/projects/dexx-rust && cargo build
```

### **两个项目路径**
- **Go项目** (参考): `/home/oscar/go-projects/src/goldenshovel-server`
- **Rust项目** (开发): `/home/oscar/projects/dexx-rust`

### **每次操作前的检查**
```bash
# 1. 确认当前目录
pwd

# 2. 切换到正确目录
cd /home/oscar/projects/dexx-rust

# 3. 验证项目文件存在
ls Cargo.toml

# 4. 然后进行操作
cargo check
```

### **常用命令模板**
```bash
# 查看文件
cd /home/oscar/projects/dexx-rust && cat src/main.rs

# 编辑文件
cd /home/oscar/projects/dexx-rust && vim src/main.rs

# 编译项目
cd /home/oscar/projects/dexx-rust && cargo build

# 运行项目
cd /home/oscar/projects/dexx-rust && cargo run

# 查看Go参考代码
cd /home/oscar/go-projects/src/goldenshovel-server && cat internal/models/user.go
```

---
**🚨 违反此规则会导致项目混乱！请务必遵守！🚨**
