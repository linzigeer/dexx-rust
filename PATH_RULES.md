# ⚠️ **极其重要：路径管理规则**

## 🚨 **绝对路径规则 - 永远不能忘记！**

### **两个项目的绝对路径**
- **Go项目** (只读参考): `/home/oscar/go-projects/src/goldenshovel-server`
- **Rust项目** (开发目标): `/home/oscar/projects/dexx-rust`

### **操作规则**
1. **查看/分析Go代码**: 必须使用 `/home/oscar/go-projects/src/goldenshovel-server/` 路径
2. **创建/修改Rust代码**: 必须使用 `/home/oscar/projects/dexx-rust/` 路径
3. **所有文件操作**: 必须明确指定完整绝对路径
4. **工作目录切换**: 每次操作前确认当前目录

### **常见错误防范**
- ❌ 在Go项目中创建Rust文件
- ❌ 在Rust项目中查找Go文件
- ❌ 使用相对路径导致混淆
- ❌ 忘记切换工作目录

### **正确操作示例**

#### 查看Go代码 (参考)
```bash
cat /home/oscar/go-projects/src/goldenshovel-server/internal/models/user.go
ls /home/oscar/go-projects/src/goldenshovel-server/internal/
```

#### 创建Rust代码 (开发)
```bash
# 创建文件
touch /home/oscar/projects/dexx-rust/src/models/user.rs

# 编辑文件
nano /home/oscar/projects/dexx-rust/src/models/user.rs

# 或使用cat创建
cat > /home/oscar/projects/dexx-rust/src/models/user.rs << 'RUST_EOF'
// Rust代码内容
RUST_EOF
```

#### 确认工作目录
```bash
cd /home/oscar/projects/dexx-rust && pwd
```

### **每次开发前的检查清单**
- [ ] 确认当前工作目录: `pwd`
- [ ] Go项目路径正确: `/home/oscar/go-projects/src/goldenshovel-server`
- [ ] Rust项目路径正确: `/home/oscar/projects/dexx-rust`
- [ ] 使用绝对路径进行所有文件操作

### **快速验证命令**
```bash
# 验证两个项目都存在
ls -la /home/oscar/go-projects/src/goldenshovel-server/
ls -la /home/oscar/projects/dexx-rust/

# 确认在正确的Rust项目目录
cd /home/oscar/projects/dexx-rust && pwd && ls Cargo.toml
```

---
**⚠️ 这个规则比任何技术细节都重要！违反此规则会导致项目混乱！**
