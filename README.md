# SoulDocV5 Dioxus Frontend

基于 `C:\Users\DELL\Desktop\SoulDocV5` 原型页和 `reference/docs` 中的开发文档，落地为 `Rust + Dioxus` 前端工程。

## 当前实现

- 用 `Dioxus Router` 承载 25 张原型页面，页面样式、布局和原型交互保持原样。
- `prototype/` 目录保留原型 HTML / CSS / JS，运行时由 `src/prototype.rs` 自动抽取 `title`、页面内联样式、正文和脚本。
- `src/api/` 提供 `mock/http` 双模式接口层，覆盖登录、工作区、Space、文档树、版本、CR、AI 任务、站点、发布目标、GitHub 同步、Webhook、Capability Manifest、Skill Manifest。
- 额外补了 `skill 注册 AI 账号` 和 `skill 管理站点` 的前端编排接口：
  - `register_ai_skill_account`
  - `build_site_management_plan`
- `public/.well-known/` 提供 `capabilities.json` 与 `skill-manifest.json`，方便 Skill / Agent 接入。

## 目录

- `src/app.rs`
  入口应用与全局样式挂载
- `src/routes.rs`
  原型页面路由映射
- `src/prototype.rs`
  原型 HTML 解析器与页面宿主组件
- `src/api/`
  前端接口客户端与类型定义
- `prototype/`
  原始原型资源
- `reference/docs/`
  原始开发文档
- `public/.well-known/`
  Skill / Capability 清单

## 运行

```bash
dx serve
```

默认首页：

- `/`
- `/index.html`

关键页面：

- `/developer.html`
- `/ai-tools.html`
- `/workspace.html`
- `/seo.html`

## Skill 设计落点

按照 `moltbook skill` 的参考逻辑，这里没有引入第三类账号体系，而是：

1. 通过 `POST /users` 创建 `user_type=ai` 的 AI 用户。
2. 通过 `POST /users/:id/service-credentials` 签发 Skill 凭证。
3. 通过 `/.well-known/skill-manifest.json` 暴露 Skill 能力边界。
4. Skill 管站时，先读 `Site / Section / Space`，再走 `Preview -> Confirm -> Release -> Webhook`。

这和文档中的治理链一致，不会绕过版本、审批、发布与审计。
