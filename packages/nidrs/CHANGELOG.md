# Changelog

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## nidrs-v0.4.0 - 2025-01-19
#### Features
- **(nidrs)** add exception handling module with predefined HTTP exceptions - (62949c7) - *Lyda*
- **(nidrs-diesel)** support multi connect - (0c52f25) - *Lyda*
#### Refactoring
- **(nidrs)** enhance exception constructors to accept generic message types - (d7540cc) - *Lyda*

- - -

## nidrs-v0.3.1 - 2025-01-06
#### Refactoring
- **(nidrs)** simplify error handling in interceptor response - (5df6daf) - *Lyda*

- - -

## nidrs-v0.3.0 - 2024-11-15
#### Bug Fixes
- **(nidrs)** adapt impl meta - (c1a6873) - *WumaCoder*
#### Features
- **(nidrs)** add register_module - (cc0d2eb) - *Lyda*
#### Miscellaneous Chores
- remove commit - (e013008) - Lyda
#### Refactoring
- **(nidrs)** add valid and openapi features - (d23cd01) - *WumaCoder*

- - -

## nidrs-v0.2.0 - 2024-09-25
#### Features
- **(nidrs)** update nidrs_extern and nidrs_macro - (6944a0b) - *Lyda*

- - -

## nidrs-v0.1.0 - 2024-09-24
#### Bug Fixes
- **(nidrs)** 修复缺少接口的问题 - (931d4c9) - *Lyda*
- **(nidrs)** 修复异常透传 status - (4f16f1d) - *WumaCoder*
- 支持自动识别返回类型 - (2723065) - WumaCoder
- 使用新的路由注册逻辑 - (132a4fa) - WumaCoder
#### Documentation
- update CHANGELOG.md formatting - (a4a13e5) - Lyda
- update - (c989f4d) - Lyda
- update ifdian.net - (29a2255) - Lyda
#### Features
- **(nidrs)** 支持 openapi 的参数解析 - (48a18d6) - *Lyda*
- **(nidrs)** 支持全局中间件应用 - (2e47165) - *Lyda*
- 初步完成 swagger 接入 - (a728aaa) - WumaCoder
- 创建 valid 包 - (8b8afb4) - WumaCoder
#### Miscellaneous Chores
- remove / path - (7d1ad25) - WumaCoder
- Refactor NidrsFactory to use OpenApiBuilder for API documentation - (ab6ce09) - WumaCoder
- Update nidrs module to use OpenApiBuilder for API documentation - (1d8a5a7) - WumaCoder
- add openapi - (66a2aef) - WumaCoder
#### Refactoring
- **(nidrs)** 修改命名 - (f10cb46) - *Lyda*
- **(nidrs)**  重新支持全局拦截器 - (f970200) - *Lyda*
- **(nidrs-openapi)** 将 openapi 的代码提炼到对应库中 - (991f97f) - *WumaCoder*
- 规范导出行为 - (d4a97e3) - WumaCoder
- update localhost - (8b68402) - Lyda
- 优化对泛型的支持 - (0a5c5fa) - Lyda
- 使用更通用的 Expr 类型去掉 PathIdent - (1291ef9) - Lyda
- 完成拦截器的重构实现 - (76c3b69) - Lyda
- 重构 meta 相关的细节实现 - (61f8251) - WumaCoder
- 使用新的路由注册逻辑 - (31432cd) - WumaCoder
- 初步重构路由注册 - (d9314ee) - Lyda
- 重构路由注册方法 - (e9a9745) - WumaCoder
- 确定了 meta 在框架中的重要性，并且规范了在编译环境下 meta 的读写规范 - (3a9deb4) - WumaCoder
- Update router scheme to use body instead of query parameters - (641931c) - WumaCoder
#### Tests
- 初步实验完成 - (b47d25b) - WumaCoder

- - -


## nidrs-v0.0.11 - 2024-06-12

#### Refactoring

- **(nidrs)** 修改 metadata 为 datasets - (3f9950b) - _WumaCoder_

- - -

## nidrs-v0.0.10 - 2024-06-11

#### Bug Fixes

- 修复 meta.set_data 被 Box 包装的问题 - (3beb01d) - WumaCoder

#### Documentation

- update readme - (ed7424d) - WumaCoder

#### Features

- meta 添加 take 方法 - (c07c6e8) - WumaCoder
- add match_full_path fn - (f24c019) - WumaCoder
- 支持 tower 中间件 - (e0a126e) - WumaCoder

#### Refactoring

- 使用 meta.set_data 替代 meta.set 来提高使用体验 - (c563c39) - WumaCoder
- 移动 meta 相关的内容到 nidrs-extern 里 - (15038c8) - WumaCoder
- 修改 meta 返回值为 Option 类型 - (16b5934) - WumaCoder
- 完善 metadata 在整个框架中配置作用 - (d35cc7e) - WumaCoder
- update meta - (14f7a41) - WumaCoder

#### Style

- format - (4658e92) - WumaCoder

#### Tests

- meta - (d0c3f59) - WumaCoder

- - -

## nidrs-v0.0.9 - 2024-06-03

#### Features

- add AnyResponse interceptor helper - (ae52dd6) - WumaCoder

#### Miscellaneous Chores

- Update NidrsFactory listen method to use module context and add externs module - (aec1b8b) - WumaCoder

#### Refactoring

- 优化导入包结构和数量 - (d33674a) - WumaCoder
- 优化 meta 和宏导出的细节 - (0460927) - WumaCoder
- 优化初始化细节 - (43e832d) - WumaCoder
- Update NidrsFactory listen method to use module context and add externs module - (3bf72f6) - WumaCoder
- Update NidrsFactory listen method to use module context - (399a0fe) - WumaCoder
- Update NidrsFactory listen method to use module context - (3d861fb) - WumaCoder

- - -

## nidrs-v0.0.8 - 2024-05-05

#### Bug Fixes

- 优化框架细节 - (60744b5) - WumaCoder
- 完善错误打印提示 - (eb302b3) - WumaCoder
- add global metadata to conf module - (b28314a) - WumaCoder

#### Miscellaneous Chores

- **(version)** nidrs-v0.0.7 - (d19d685) - _WumaCoder_
- update toml - (7cbe44e) - WumaCoder

#### Refactoring

- Update DieselOptions struct to use generic type for driver - (b2dbc64) - WumaCoder
- 优化 global service 获取逻辑 - (d36c717) - WumaCoder

- - -

## nidrs-v0.0.7 - 2024-04-28

#### Bug Fixes

- update version - (c058256) - WumaCoder

#### Miscellaneous Chores

- update readme - (5b557d0) - WumaCoder

- - -

## nidrs-v0.0.6 - 2024-04-28

#### Bug Fixes

- 完成模块的 exports 支持 - (209bc7c) - WumaCoder
- 自动设置 worker - (47fdafc) - WumaCoder
- 不重复注册 service - (654806e) - WumaCoder

#### Documentation

- update readme - (35cd0fa) - WumaCoder

#### Features

- 初步支持模块作用域 - (088ba72) - WumaCoder

#### Miscellaneous Chores

- update cargo toml - (78cd499) - WumaCoder
- add core test - (1d166f2) - WumaCoder
- update - (9436e15) - WumaCoder
- add hook - (7946733) - WumaCoder

#### Style

- format code - (d847b7a) - WumaCoder
- code - (aef08d1) - WumaCoder

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).
