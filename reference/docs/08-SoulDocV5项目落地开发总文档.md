# SoulDocV5 项目落地开发总文档

> 2026-04-16 最终口径更新：
> 本文档若与旧版原型表述冲突，以本次收口后的定义为准。
> `Collection` 现在统一定义为“内容集合”，属于 `Scope（个人工作区 / 组织）` 下的可选内容分组；`Space` 可以直接挂在 `Scope` 下，也可以被归入 `Collection`；
> `Site` 统一定义为“发布站点”，只负责域名、SEO、公开导航和发布结果；
> `Section` 统一定义为“站点分区”，属于 `Site` 的公开导航层；
> `Site` 不直接拥有 `Collection` 或 `Document`，而是通过链接关系挂接公开 `Space`。
> 产品文档统一使用“GitHub 同步”，技术实现层保留 `git_sync` 命名即可。

## 1. 文档目的

本文档用于把 `SoulDocV5` 原型中已经表达的功能、逻辑、架构、对象模型、页面边界、权限体系、AI 能力、多语言机制、发布链路、开放平台与研发阶段计划收敛为一份完整的项目落地文档。

目标：

- 让产品、设计、前端、后端、AI、测试、运维能共享同一份基线
- 让后续开发不再依赖“看原型猜逻辑”
- 让系统边界、核心对象、关键流程和优先级一目了然
- 作为 `SoulDocV5` 的首版实施说明书

本文档基于以下资料汇总：

- `01-原型展示范围说明.md`
- `02-接口清单.md`
- `03-前后端模块拆解.md`
- `04-数据库表结构设计.md`
- `05-AI工具配置重构方案.md`
- `06-技术栈建议（Rust预留）.md`
- `07-V5收尾补全说明.md`
- `09-发布与工作流操作文档.md`
- `SoulDocV5` 原型页面全集

---

## 2. 项目定义

### 2.1 一句话定义

`SoulDoc` 是一套 AI 原生知识生产、治理、发布与分发平台，不是单纯的在线文档系统。

### 2.2 与普通文档系统的本质区别

- `AI` 与 `人类` 同为系统用户类型，而不是助手挂件
- 多语言不是附属翻译功能，而是文档治理对象
- 发布不是“点一个按钮”，而是 `版本 -> 变更请求 -> 预览 -> 发布 -> 公开结果页` 的链路
- 平台不只有 `Space + Document`，而是 `Account / Scope（Personal Workspace / Organization） / 内容集合（Collection，可选） / Space / Document Group / Document(locale) + 发布站点（Site） / 站点分区（Section） / Linked Space`
- GitHub 同步、开放平台、Webhook、AI 工具、公开阅读页与后台治理链在同一产品模型内

术语说明：

- 产品文案统一使用 `发布站点` 对应技术实体 `site`
- 产品文案统一使用 `内容集合` 对应技术实体 `collection`
- 产品文案统一使用 `站点分区` 对应站点导航分区，不再把它和 `collection` 混用

### 2.3 目标用户

- 内容团队
- 产品团队
- 技术团队
- 多语言文档团队
- AI 协作型组织
- 需要知识沉淀、对外发布和 Docs-as-Code 联动的团队

### 2.4 产品原则

- AI 与人类平权
- 编辑链与发布链解耦
- 高风险动作统一进入治理链
- 公共页与后台页解耦
- 多语言独立成文档版本
- 审计、恢复、可观测性必须贯穿系统

---

## 3. 总体架构

### 3.1 分层架构

```text
Account / Subscription
- Personal Workspace (Scope)
  - Space
    - Document Group
      - Document(locale=zh-CN)
      - Document(locale=en-US)
      - Document(locale=ja-JP)
    - Version / Change Request / Tag / File / Comment
    - Search / AI Task / Notification / Publish Target
  - Collection
    - Collection Space Binding -> Space
  - Template Library
  - 单人发布站点（可选）
- Organization (Scope)
  - Member / Role / Approval Policy / Audit Scope
  - Space
    - Document Group
    - Version / Change Request / Tag / File / Comment
    - Search / AI Task / Notification / Publish Target
  - Collection
    - Collection Space Binding -> Space
  - Template Library
  - Site
  - 站点分区（Site Section）
    - Site Space Binding -> Published Space
  - GitHub Binding / Release / Webhook / Capability
```

### 3.2 架构层次

1. 账户与平台层
2. 内容与治理层
3. 协作与资源层
4. AI 与多语言层
5. 发布与开放层
6. 系统与安全层

### 3.3 核心主链

1. 内容归属链  
`Account -> Scope（Personal Workspace / Organization） -> Space -> Document Group -> Document`

补充说明：
- `Collection` 是 Scope 下的可选分组，用于组织多个 Space，而不是所有 Space 的必经父层

2. 发布编排链  
`Scope -> 发布站点（Site） -> 站点分区（Section） -> Linked Space -> Published Document`

3. 内容生产链  
`Create / Dialogue -> Draft -> Edit -> Version -> Change Request`

4. 发布治理链  
`CR -> Preview -> Publish -> Release -> Public Site`

5. 多语言治理链  
`Source Locale -> Translation Workflow -> Review -> Locale Publish`

6. AI 工作链  
`Capability -> Async Task -> Review/Audit -> Notification -> Result Backflow`

7. 集成分发链  
`Publish -> GitHub 同步 -> Webhook -> External Consumer`

---

## 4. 核心对象模型

### 4.1 Account

含义：

- 系统根节点
- 持有订阅、身份、额度、全局配置
- 管理个人工作区、多个 Organization、用户和 AI 用户

主要职责：

- 订阅与套餐
- 全局安全策略
- 全局身份策略
- 全局可观测性策略
- 多站点能力边界

#### Personal Workspace

含义：

- 账号天然自带的个人载体
- 承载个人草稿、私有知识沉淀、偏好和未进入组织治理链的文档

主要职责：

- 存放个人空间与个人文档
- 管理个人偏好、通知和草稿归档
- 作为内容迁移到组织空间前的缓冲区

#### Organization

含义：

- 团队协作与所有权载体
- 承载成员目录、组织角色、内容集合、组织空间与发布站点

主要职责：

- 统一管理组织成员与 AI 用户
- 统一承载发布站点、内容集合、组织空间、模板库和发布目标
- 统一承载账单范围、审计策略与权限模板
- 作为个人文档进入正式协作链路的目标边界

### 4.2 发布站点（Site）

含义：

- 公开发布视图与品牌入口
- 面向外部阅读者的结果层
- 默认归属于某个 `Organization`

主要职责：

- 品牌名称、Logo、域名
- 公开导航结构
- 多站点分区聚合
- SEO 发布站点级配置
- 公开阅读页输出

### 4.3 内容集合（Collection）

含义：

- Scope 下的内容分组层
- 对多个 Space 做目录编排

主要职责：

- 分组导航
- Space 编排
- 作为模板、权限和内容归属的上层内容结构

### 4.4 站点分区（Section）

含义：

- Site 下的公开导航分区
- 用于组织已发布的 Space，而不是组织内部内容所有权

主要职责：

- 公开站点菜单排序
- Space 对外入口编排
- 多语言或版本变体入口组织

### 4.5 Space

含义：

- 业务与权限边界
- 真正的协作容器
- 归属于 `Personal Workspace` 或 `Organization`

空间类型：

- `Personal Space`
- `Team Space`

可见性：

- `公共空间`
- `私有空间`

主要职责：

- 文档树与知识域容器
- 成员、角色与权限
- 标签、文件、通知、AI 任务
- 语言版本与发布链

### 4.5 Document Group

含义：

- 同一篇文档的多语言聚合对象

主要职责：

- 通过 `doc_group_id` 聚合同文档的不同语言版本
- 持有默认语言、回退语言
- 管理语言发布状态与对照关系

### 4.6 Document

含义：

- 单语言文档对象

特点：

- 每个 `document` 只对应一个语言版本
- 文档树中的节点是语言文档，不是语言包
- 具体正文由块级内容组成

### 4.7 Block

含义：

- 文档的最小可编辑单元

主要职责：

- 承载 Markdown / 富文本 / 结构块
- 支撑块级改写、Diff、评论、AI 处理

### 4.8 Version

含义：

- 文档内容快照

主要职责：

- 版本时间线
- 差异对比
- 恢复
- 发布前基线

### 4.9 Change Request

含义：

- 类 Git 的文档治理对象

主要职责：

- 审核
- 评论
- 预览
- 合并
- 拒绝

### 4.10 User

用户类型：

- `human`
- `ai`

原则：

- 共享同一套 `role` 与 `permission`
- 地位平等
- 风险边界不同

### 4.11 Role / Permission

典型角色：

- Owner
- Admin
- Editor
- Member
- Viewer

关键点：

- 角色是权限模板
- 用户类型不决定高低
- AI 用户与人类用户都通过角色获得基础权限
- 高风险动作需要额外执行边界

### 4.12 Tag / File / Comment / Notification

- `Tag`：语义组织和检索辅助
- `File`：文档附件与媒体资产
- `Comment`：审阅与协作反馈
- `Notification`：任务、审核、发布、语言、集成回执

### 4.13 AI Task

含义：

- AI 能力执行的异步任务对象

主要职责：

- 排队
- 执行
- 回执
- 重试
- 结果回流

### 4.14 Publish Target / Release / Public Site

- `Publish Target`：发布目标配置
- `Release`：发布记录
- `Public Site`：面向外部的发布站点结果态

### 4.15 Git Binding / Webhook / Capability Manifest

- `Git Binding`：仓库绑定
- `GitHub Sync Task`：同步任务
- `Webhook`：对外事件订阅
- `Capability Manifest`：AI/开放平台能力清单

---

## 5. 用户、角色与权限逻辑

### 5.1 用户模型

用户只有两类：

- 人类用户
- AI 用户

规则：

- 两类用户都是系统一级公民
- 任何显示用户名的地方都应带类型标识
- 类型用于识别，不用于判断等级

### 5.2 角色模型

建议角色含义：

| 角色 | 主要能力 |
| --- | --- |
| Owner | 空间所有权、治理链最高权限、成员与发布目标管理 |
| Admin | 空间管理、权限配置、审核与发布管理 |
| Editor | 创建和编辑文档、发起 CR、参与语言治理 |
| Member | 查看、评论、参与协作流程 |
| Viewer | 只读访问已授权内容 |

### 5.3 权限模型

权限应分为以下几类：

- 读取权限
- 写入权限
- 治理权限
- 发布权限
- 成员管理权限
- 集成权限
- AI 执行权限

### 5.4 AI 用户执行边界

AI 用户默认允许：

- 读取上下文
- 列出文档树
- 搜索
- 草稿生成
- 文本分析
- 翻译生成
- SEO 建议

AI 用户默认不直接执行：

- 硬删除
- 直接发布
- 跳过审核
- 越权读取私有内容
- 修改系统级安全配置

高风险动作统一策略：

- 先生成预览
- 进入审核或确认
- 记录审计链
- 给出通知回执

### 5.5 空间可见性

只有两种：

- 公共空间：任何人可查看已发布公开内容
- 私有空间：只有成员可查看内容与语言版本

规则：

- 草稿、CR、内部评论、AI 任务永远不是公开态
- 公开阅读页只读取发布态
- 私有空间的公开链接应返回权限提示，不应裸露内容

### 5.6 删除与恢复

策略：

- 默认软删除
- 进入回收站
- 保留 30 天恢复窗口
- 恢复产生新版本，不覆盖既有历史

---

## 6. 功能架构

### 6.1 工作台与结构层

模块：

- 首页工作台
- 站点架构
- 空间列表
- 空间概览

功能：

- 展示平台骨架
- 快速进入高频功能
- 查看 Site / Section 与 Scope / Collection（可选） / Space 的双链关系
- 显示空间活跃度、治理信号、多语言和发布信号

### 6.2 文档与治理主链

模块：

- 文档中心
- 模板中心
- 编辑器
- 版本历史
- 变更请求

功能：

- 创建文档
- 从模板生成文档
- 模板制作、升级与替换
- 树状管理
- 结构化编辑
- 版本快照
- Diff
- 恢复
- CR 审核与合并

### 6.3 协作与资源层

模块：

- 成员权限
- 标签管理
- 文件管理
- 通知中心
- 个人中心

功能：

- 用户与 AI 用户统一管理
- 角色与权限分配
- 标签语义组织
- 文件资产管理
- 回执、提醒、审核通知
- 用户资料、订阅、安全状态查看

### 6.4 AI 与多语言层

模块：

- 搜索
- 语言版本中心
- AI 任务中心
- AI 工具配置

功能：

- 关键词搜索
- 语义搜索
- 跨语言召回
- 语言版本治理
- 翻译工作流
- 术语库与翻译记忆
- AI 任务排队与回执
- AI 能力配置与连接器编排

### 6.5 发布与开放层

模块：

- 发布 & SEO
- 公开阅读页
- GitHub 同步
- 开发者平台

功能：

- SEO 配置
- 发布目标管理
- 公开结果态查看
- 预览环境与发布记录
- 仓库同步
- Webhook
- 对外能力清单

### 6.6 系统层

模块：

- 安装
- 登录
- 系统设置

功能：

- 初始化部署
- 认证
- 系统级安全策略
- 审计、监控、恢复配置

---

## 7. 页面地图与研发映射

### 7.1 工作台与结构层

| 页面 | 文件 | 作用 |
| --- | --- | --- |
| 首页 | `index.html` | 平台概览、工作台、关键入口 |
| 组织主页 | `organization.html` | 展示组织成员、组织空间、发布站点与个人/组织边界 |
| 发布站点 | `workspace.html` | 展示 Scope / Collection（可选） / Space 的内容关系，以及 Site / 站点分区 / Linked Space 的发布结构 |
| 空间列表 | `spaces.html` | 展示个人空间、组织空间与创建空间 |
| 空间概览 | `space.html` | 单个 Space 的业务与治理总览 |

### 7.2 文档与治理层

| 页面 | 文件 | 作用 |
| --- | --- | --- |
| 文档中心 | `docs.html` | 文档树、列表、元信息、右侧摘要 |
| 模板中心 | `templates.html` | 模板选择、制作、升级、替换与 GitHub 同步 |
| 编辑器 | `editor.html` | 内容编辑、AI 辅助、版本入口 |
| 版本中心 | `versions.html` | 时间线、Diff、恢复 |
| 变更请求 | `change-request.html` | CR 审核与讨论 |

### 7.3 协作与资源层

| 页面 | 文件 | 作用 |
| --- | --- | --- |
| 成员权限 | `members.html` | 成员、角色、权限矩阵 |
| 标签管理 | `tags.html` | 标签维护与语义组织 |
| 文件管理 | `files.html` | 文件库与引用 |
| 通知中心 | `notifications.html` | 任务、审核、发布与系统通知 |
| 个人中心 | `profile.html` | 用户资料、个人工作区、账号状态、安全与审计摘要 |

### 7.4 AI 与多语言层

| 页面 | 文件 | 作用 |
| --- | --- | --- |
| 搜索 | `search.html` | 关键词/语义/跨语言搜索 |
| 语言版本 | `language.html` | 文档多语言治理中心 |
| AI 任务中心 | `ai-tasks.html` | AI 异步任务与回执 |
| AI 工具配置 | `ai-tools.html` | 6 大能力族与开放能力映射 |

### 7.5 发布与开放层

| 页面 | 文件 | 作用 |
| --- | --- | --- |
| 发布 & SEO | `seo.html` | SEO 配置、发布记录、目标管理 |
| 公开阅读页 | `public-doc.html` | 对外阅读结果页 |
| GitHub 同步 | `git-sync.html` | 仓库绑定、同步任务、冲突回流 |
| 开发者平台 | `developer.html` | API Key、Webhook、AI 用户接入、开放能力 |

### 7.6 系统层

| 页面 | 文件 | 作用 |
| --- | --- | --- |
| 安装 | `install.html` | 初次部署引导 |
| 登录 | `login.html` | 认证入口 |
| 系统设置 | `settings.html` | 系统安全、身份、审计、恢复 |
| 兼容页 | `ai-center.html` | 旧入口兼容跳转 |

---

## 8. 核心业务流程

### 8.1 安装与登录流程

1. 进入安装页，完成基础配置
2. 创建首个管理员账户
3. 登录系统
4. 进入首页工作台

### 8.2 创建空间流程

1. 从空间列表选择归属载体：`个人工作区` 或 `Organization`
2. 选择 `Personal Space` 或 `Team Space`
3. 选择 `公共` 或 `私有`
4. 配置默认角色模板
5. 进入空间概览

### 8.3 创建文档流程

支持三种入口：

- 空白文档
- 从模板生成
- 从对话转文档

标准流程：

1. 选择父级目录
2. 选择文档类型
3. 生成草稿
4. 进入编辑器

### 8.4 编辑与版本流程

1. 用户进入编辑器修改内容
2. 系统自动保存草稿
3. 用户手动或自动生成版本快照
4. 可在版本中心查看 Diff
5. 如需回滚，走恢复操作生成新版本

### 8.5 变更请求流程

1. 从草稿发起 CR
2. 指定审核人
3. 系统生成变更预览
4. 审核人评论、通过或拒绝
5. 合并后进入发布链

### 8.6 多语言流程

1. 主语言文档更新
2. 语言版本中心识别受影响语言
3. 发起翻译工作流
4. AI 生成翻译草稿
5. 人类或高权限 AI 用户复审
6. 各语言独立发布

### 8.7 搜索流程

1. 用户输入关键词或语义查询
2. 系统先在当前语言检索
3. 如开启跨语言召回，补充其他语言结果
4. 如目标语言缺失，按回退语言返回

### 8.8 AI 任务流程

1. 用户触发 AI 能力
2. 系统生成异步任务
3. 任务进入队列
4. 任务执行后返回结果
5. 结果进入通知中心、文档、CR 或语言版本链

### 8.9 发布流程

1. 选择发布目标
2. 生成预览
3. 执行发布
4. 生成 Release 记录
5. 更新公开阅读页
6. 触发 SEO 刷新与外部回调

### 8.10 公共/私有访问流程

公共空间：

- 所有人可访问已发布公开文档

私有空间：

- 非成员访问时返回权限提示页

### 8.11 GitHub 同步流程

1. 绑定仓库与分支
2. 生成同步任务
3. 执行推送或回拉
4. 产生同步结果
5. 回流通知、审计链、Webhook

### 8.12 开放平台流程

1. 创建 API Key
2. 配置 Webhook
3. 查看 Capability Manifest
4. 外部系统读取文档、版本、语言、发布与任务状态

---

## 9. AI 能力架构

### 9.1 设计原则

- AI 是用户，不是外挂
- AI 操作必须可追踪
- AI 能力必须分层，不能扁平堆叠
- 写入、发布、删除等高风险动作不能绕开治理链

### 9.2 6 大能力族

1. Context  
读取当前上下文、导航位置、权限边界

2. Knowledge  
列内容、读内容、混合搜索、相关推荐

3. Drafting  
创建草稿、块级改写、对话转文档

4. Governance  
版本快照、CR 预览、审批提交流

5. Publishing  
SEO 检查、发布预览、发布执行

6. Connectors  
GitHub 同步、Webhook、外部连接器、能力清单

### 9.3 AI 任务类型

- 摘要生成
- 大纲生成
- FAQ 生成
- 翻译
- SEO 优化
- 内容缺口分析
- 审阅建议
- 版本审查
- 对话转文档

### 9.4 AI 结果回流位置

- 文档草稿
- 语言版本草稿
- CR 预览
- AI 任务中心
- 通知中心
- 发布前检查

### 9.5 AI 审计要求

- 记录输入对象
- 记录调用能力
- 记录用户类型
- 记录执行边界
- 记录结果状态
- 记录回执位置

---

## 10. 多语言设计

### 10.1 设计原则

- 同一篇文档的不同语言是独立文档
- 多语言通过 `doc_group_id` 关联
- 语言版本应独立保存、独立审核、独立发布

### 10.2 关键字段

- `doc_group_id`
- `locale`
- `source_locale`
- `default_locale`
- `fallback_locale`
- `translation_status`

### 10.3 回退规则

1. 优先当前用户语言
2. 若不存在，回退到默认语言
3. 若仍不存在，返回语言缺失提示

### 10.4 跨语言搜索

规则：

- 先检索当前语言
- 再补充其他语言结果
- 对结果标记语言来源
- 对缺失语言执行回退显示

### 10.5 翻译工作流

1. 主语言更新
2. 标记相关语言待复审
3. AI 生成草稿
4. 术语库校验
5. 人工复审
6. 独立发布

### 10.6 扩展能力

- 翻译记忆
- 术语库
- 自动语言检测
- 区域化 SEO
- RTL 支持

---

## 11. 发布、公开阅读与 GitHub 同步

### 11.1 发布链定义

发布不是编辑器里的即时行为，而是治理链结果。

更细的页面职责、操作顺序、Space / 发布站点状态与发布链说明，见 `09-发布与工作流操作文档.md`。

标准链路：

`Draft -> Version -> Change Request -> Preview -> Publish -> Release`

### 11.2 发布目标

建议支持：

- 公共阅读页
- 私有成员阅读页
- 静态导出
- Git 分支预览

### 11.3 公开阅读页逻辑

公共空间：

- 返回已发布语言版本
- 支持语言切换
- 支持 SEO 元数据与 OG

私有空间：

- 非成员显示权限提示
- 不返回敏感内容

### 11.4 SEO 逻辑

覆盖：

- 标题
- 描述
- 关键词
- URL Slug
- Open Graph
- robots.txt
- sitemap.xml
- llms.txt
- 结构化数据

### 11.5 GitHub 同步逻辑

场景：

- 可视化编辑结果推送到仓库
- 仓库变更同步回系统
- 冲突进入治理链而不是直接覆盖

关键对象：

- 仓库绑定
- 分支映射
- 同步任务
- 冲突结果
- 回执通知

---

## 12. 开放平台与开发者能力

### 12.1 对外能力

- API Key
- Webhook
- Capability Manifest
- AI 用户接入

### 12.2 Webhook 事件建议

- `document.published`
- `document.version_created`
- `change_request.opened`
- `change_request.merged`
- `locale.review_required`
- `space.visibility_changed`
- `git.sync.completed`
- `ai.task.completed`

### 12.3 Capability Manifest 作用

- 向外部 Agent / Skill / 平台暴露能力边界
- 标识同步接口、异步任务和连接器能力
- 作为后续 AI 生态接入标准

### 12.4 开放平台边界

开放平台可读：

- Scope / Space / 内容集合（Collection，可选） / 发布站点（Site） / 站点分区（Section）
- 文档与版本
- 搜索结果
- 语言版本状态
- 发布与公开页状态
- GitHub 同步与任务状态

开放平台不可绕过：

- 审核链
- 权限系统
- 发布审批
- 审计记录

---

## 13. 前端架构建议

### 13.1 页面域拆分

建议分为以下前端域：

- shell：导航、布局、全局状态
- workspace：组织、发布站点与空间
- docs：文档树、文档详情、编辑器
- governance：版本、CR、通知
- resources：成员、标签、文件
- ai：搜索、AI 任务、AI 工具、语言版本
- publish：SEO、公开页、GitHub 同步
- platform：开发者平台、系统设置、个人中心、工作区切换

### 13.2 核心组件

- SidebarShell
- TopbarShell
- WorkspaceScopeSwitcher
- OrganizationOverview
- SpaceSwitcher
- DocumentTree
- EditorSurface
- VersionTimeline
- ChangeRequestPanel
- PermissionMatrix
- LanguageVersionBoard
- SearchResultPanel
- AiTaskBoard
- PublishTargetBoard
- GitSyncBoard
- PublicReaderShell

### 13.3 状态层建议

全局状态：

- 当前用户
- 当前空间
- 当前站点
- 当前文档组
- 当前语言
- 通知计数

业务状态：

- 文档树
- 编辑器内容
- 版本与 CR
- 搜索结果
- AI 任务
- 发布状态

### 13.4 前端研发约束

- 页面不是孤岛，必须映射到服务域
- 公共阅读页与后台页状态隔离
- 语言切换必须建立在 `doc_group_id` 上
- AI 相关页面必须支持任务回执

---

## 14. 后端架构建议

### 14.1 服务域

建议服务：

- AuthService
- AccountService
- PersonalWorkspaceService
- OrganizationService
- SiteService
- CollectionService
- SpaceService
- MemberService
- RoleService
- PermissionService
- DocumentService
- BlockService
- VersionService
- ChangeRequestService
- TagService
- FileService
- CommentService
- NotificationService
- SearchService
- AiService
- LanguageService
- PublishService
- PublicSiteService
- GitSyncService
- WebhookService
- CapabilityService
- AuditService
- SystemSettingService

### 14.2 Worker / Async Job

建议异步 Worker：

- Search Index Worker
- Embedding Worker
- Translation Worker
- Seo Worker
- DialogueToDoc Worker
- Publish Worker
- GitHub Sync Worker
- Notification Dispatcher
- Audit Sink

### 14.3 事件总线

建议引入 Event Bus 处理：

- 发布事件
- CR 状态变更
- 语言待复审
- AI 任务完成
- GitHub 同步完成
- 空间可见性变更

### 14.4 可观测性

建议纳入：

- OpenTelemetry
- Trace
- Metrics
- Structured Logs
- Task Logs
- Audit Logs

---

## 15. 数据模型与数据库边界

### 15.1 核心业务表

建议最小集合：

- `account`
- `site`
- `collection`
- `space`
- `user`
- `role`
- `space_member`
- `document_group`
- `document`
- `document_block`
- `document_version`
- `change_request`
- `tag`
- `document_tag`
- `file_asset`
- `comment`
- `notification`
- `ai_task`
- `search_index`
- `publish_target`
- `release_record`
- `git_repository_binding`
- `git_sync_task`
- `webhook`
- `capability_manifest`
- `audit_log`

### 15.2 关键关系

- `scope 1:n collection`
- `scope 1:n space`
- `collection n:n space` 通过 `collection_space_binding`
- `site 1:n site_section`
- `site_section n:n space` 通过 `site_space_binding`
- `space 1:n document_group`
- `document_group 1:n document`
- `document 1:n block`
- `document 1:n version`
- `document 1:n change_request`
- `space n:n user` 通过 `space_member`

### 15.3 数据设计原则

- 语言独立成文档对象
- 搜索、AI、发布、GitHub 同步独立扩展表
- 不把任务结果直接塞回主业务表
- 审计日志不可被业务更新覆盖
- 发布视图与编辑视图隔离

### 15.4 索引重点

- `document.doc_group_id + locale`
- `document.space_id + parent_id + sort_order`
- `version.document_id + created_at`
- `change_request.document_id + status`
- `ai_task.space_id + status + created_at`
- `notification.user_id + read_status + created_at`

---

## 16. 接口边界

### 16.1 接口域划分

- 系统与安装
- 认证与用户
- Scope / Space / Collection（可选） / Site
- 成员、角色与权限
- 文档树、文档、文档组、语言版本
- Block / Version / Change Request
- 标签、文件、评论、通知
- 搜索、AI、能力清单
- SEO、发布、公开阅读
- GitHub 同步、Webhook、开放平台

### 16.2 接口风格

同步接口：

- REST
- 用于读取、轻量写入、列表、详情、配置读取

异步接口：

- 用于 AI 任务、搜索索引、发布、翻译、GitHub 同步

回调接口：

- Webhook

描述接口：

- Capability Manifest

### 16.3 接口约束

- 统一分页、排序、过滤
- 所有高风险动作返回预览或任务号
- 所有异步任务必须可查询状态
- 所有 AI 写入动作必须支持审计

---

## 17. 安全、审计、恢复

### 17.1 安全要求

- 支持 MFA / 2FA
- 可扩展 OIDC / SSO
- TLS 传输加密
- 密钥与对象存储隔离
- 提示注入防护
- 模型白名单
- 工具边界校验

### 17.2 审计要求

必须记录：

- 操作人或 AI 用户
- 用户类型
- 所属空间
- 操作对象
- 动作类型
- 风险等级
- 执行结果
- 时间

### 17.3 恢复要求

- 软删除
- 回收站恢复
- 发布回滚
- 任务重试
- 备份与灾备策略

### 17.4 合规建议

- 数据分类
- 敏感操作留痕
- 导出与删除能力
- 多地区部署预留

---

## 18. 技术栈建议

### 18.1 当前建议

现阶段先锁定架构边界，不在本轮把所有技术细节一次定死。

### 18.2 前端建议

- Web 前端采用现代组件化框架
- 编辑器采用块级/富文本内核
- 搜索、版本、任务、发布页保持页面域拆分

### 18.3 后端建议

Rust 作为后端主技术栈建议正式保留，数据库底座以 `SurrealDB` 为优先。

建议优先评估组合：

- `axum`
- `tokio`
- `surrealdb` Rust SDK
- `redis` 或 `NATS` 作为任务与限流辅助层
- `s3 / MinIO` 兼容对象存储

说明：

- 关系、图链接、全文索引、向量召回优先收敛到 `SurrealDB`
- 只有在规模或召回质量需要时，再补独立检索组件

### 18.4 不在本轮定死的部分

- 最终编辑器内核
- 搜索引擎的最终选型
- 向量数据库最终选型
- GitHub 同步的推送/回拉策略细节
- 多租户模式是否一次到位

---

## 19. 研发阶段建议

### 19.1 P0：基础可运行平台

目标：

- 安装、登录
- 首页、站点架构、空间列表、空间概览
- 文档中心、编辑器
- 基础版本能力
- 用户、角色、空间成员

交付：

- 基础表结构
- 基础认证
- Space / Document 主链
- 编辑器可保存

### 19.2 P1：治理与协作闭环

目标：

- 版本时间线
- Change Request
- 通知中心
- 标签、文件
- 权限矩阵
- 搜索基础版

### 19.3 P2：AI 与多语言闭环

目标：

- AI 任务中心
- AI 工具配置
- 对话转文档
- 多语言文档组
- 翻译工作流
- 跨语言搜索

### 19.4 P3：发布与开放闭环

目标：

- SEO 与发布
- 公开阅读页
- GitHub 同步
- Webhook
- API Key
- Capability Manifest

### 19.5 P4：安全与平台增强

目标：

- MFA / SSO
- 审计链
- 回收站
- 备份恢复
- OpenTelemetry
- 风险控制增强

---

## 20. 测试与验收建议

### 20.1 功能验收

必须覆盖：

- 空间创建与可见性
- 文档创建与编辑
- 版本与恢复
- CR 审核与合并
- 成员与权限
- 标签与文件
- 搜索与跨语言
- AI 任务回执
- 发布与公开访问
- GitHub 同步与 Webhook

### 20.2 权限验收

重点验证：

- 人类与 AI 用户权限一致性
- 私有空间越权访问
- AI 高风险动作拦截
- 发布链审批拦截

### 20.3 非功能验收

重点验证：

- 审计可追踪
- 任务可重试
- 删除可恢复
- 发布可回滚
- 搜索延迟
- 任务吞吐

---

## 21. 当前原型未完全展开但需要研发预留的部分

- 订阅与计费
- 更细粒度的企业组织结构
- 完整模型治理
- 更复杂的翻译记忆与术语库管理页
- 全量安全运营面板
- 多区域部署与合规策略细化

这些内容在原型中没有完全页面化，但研发设计时要预留扩展边界。

---

## 22. 最终落地结论

`SoulDocV5` 的本质不是一个“带 AI 的知识库”，而是一套：

- 以 `Account / Scope / 内容集合（Collection，可选） / Space / Document Group / Document + 发布站点（Site） / 站点分区（Section） / Linked Space` 为骨架
- 以 `AI 与人类平权` 为用户模型
- 以 `Version / Change Request / Publish` 为治理链
- 以 `多语言独立版本` 为内容模型
- 以 `Public Site / GitHub 同步 / Webhook / Capability Manifest` 为分发与开放边界

的 AI 原生知识生产与治理平台。

后续开发应以本文档为总纲，以 `01-07` 为补充细节文档。

---

## 23. 配套文档索引

- `01-原型展示范围说明.md`：原型范围与页面地图
- `02-接口清单.md`：接口域与优先级
- `03-前后端模块拆解.md`：前后端模块边界
- `04-数据库表结构设计.md`：数据模型
- `05-AI工具配置重构方案.md`：AI 工具与权限策略
- `06-技术栈建议（Rust预留）.md`：技术栈方向
- `07-V5收尾补全说明.md`：V5 与前期文档对照补全
- `09-发布与工作流操作文档.md`：编辑、审核、发布、预览与分发主流程

本文档建议作为后续研发 kickoff 的默认入口文档。
