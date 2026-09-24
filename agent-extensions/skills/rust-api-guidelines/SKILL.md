---
name: "rust-api-guidelines"
description: >-
  Rust API 设计规范速查与评审清单，覆盖命名、互操作性、宏、文档、可预测性、
  灵活性、类型安全、可靠性、可调试性、向后兼容与必要条件。
  当用户设计/评审 Rust crate 公共 API、命名方法或类型、写 rustdoc、
  发布 crate 或询问 Rust API 最佳实践时调用。
---

# Rust API 设计规范（Rust API Guidelines）

这是一套面向 Rust 公共 API 设计与评审的 **API 设计建议**。它们是「建议」而非 强制规范，但遵循它们能让你的 crate 与生态更好地集成。

## 何时使用

- 设计或评审某个 Rust crate 的 **公共 API**（函数、方法、类型、trait、宏）。
- 为类型、方法命名，或纠结于 `as_` / `to_` / `into_` / `from_` / `new` 等约定。
- 编写 rustdoc 文档、`Cargo.toml` 元数据，或准备发布 crate。
- 需要一份「评审清单」逐项检查 API 质量时。

## 使用方式

1. 明确评审对象（新 API 设计 or 存量 API 审查）。
2. 对照下方 **11 大类 / ~40 条规范**逐条核对，每条都有唯一编号 `C-XXX`。
3. 输出时引用规范编号（如「违反 C-GETTER」），并给出符合约定的改法。

---

## 1. 命名 Naming

*crate 与 Rust 命名约定一致*

- **C-CASE**：大小写遵循 RFC 430。类型/trait/枚举变体用 `UpperCamelCase`；
  函数/方法/模块/变量用 `snake_case`；静态量/常量用 `SCREAMING_SNAKE_CASE`；
  类型参数用简洁的单个大写字母 `T`。缩写按一个词处理（用 `Uuid` 而非 `UUID`）。
  crate 名不要带 `-rs`/`-rust` 后缀。
- **C-CONV**：临时转换方法用前缀区分成本与所有权：
    - `as_`：零成本，借用 → 借用。
    - `to_`：昂贵，通常借用 → 拥有（或做实际计算）。
    - `into_`：变动成本，拥有 → 拥有（消耗自身）。
    - 包装单值的类型用 `into_inner()` 取出内部值；`mut` 出现在返回类型时命名如 `as_mut_slice`。
- **C-GETTER**：getter 不加 `get_` 前缀（`foo()` 而非 `get_foo()`）；
  仅当「唯一且显然」时才用 `get`（如 `Cell::get`）。可提供 `unsafe get_unchecked`。
- **C-ITER**：集合产出迭代器的方法命名为 `iter` / `iter_mut` / `into_iter`。
- **C-ITER-TY**：迭代器类型名与产出方法对应（`into_iter()` → `IntoIter`）。
- **C-FEATURE**：Cargo feature 名不含占位词（用 `abc`，不用 `use-abc`/`with-abc`）；
  可选标准库依赖统一用 `std`；feature 必须是 **叠加**的（避免 `no-abc`）。
- **C-WORD-ORDER**：全 crate 内词序保持一致（如错误类型统一「动-宾-Error」，
  `ParseAddrError` 而非 `AddrParseError`）。

## 2. 互操作性 Interoperability

*crate 与其他库良好协作*

- **C-COMMON-TRAITS**：因孤儿规则，类型应 **主动**实现常见 trait：
  `Copy`、`Clone`、`Eq`、`PartialEq`、`Ord`、`PartialOrd`、`Hash`、`Debug`、
  `Display`、`Default`。同时提供空参 `new` 与 `Default`。
- **C-CONV-TRAITS**：转换实现 `From` / `TryFrom` / `AsRef` / `AsMut`； **不要**实现 `Into` / `TryInto`（它们有基于 `From`
  的通用实现）。
- **C-COLLECT**：集合实现 `FromIterator` 与 `Extend`。
- **C-SERDE**：数据结构类型实现 Serde 的 `Serialize` / `Deserialize`，
  用名为 `"serde"` 的可选 feature 门控。
- **C-SEND-SYNC**：尽可能让类型是 `Send` 与 `Sync`（可用 `assert_send/assert_sync` 测试守护）。
- **C-GOOD-ERR**：错误类型有意义且规范——实现 `std::error::Error` + `Send` + `Sync`； **绝不用 `()` 作错误类型**；`Display`
  文案小写、无句尾标点、简洁；不实现已弃用的 `description()`。
- **C-NUM-FMT**：会做位运算的数字类型实现 `UpperHex`/`LowerHex`/`Octal`/`Binary`。
- **C-RW-VALUE**：泛型读写函数按值接收 `R: Read` / `W: Write`（调用方可传 `&mut f`），并在文档提醒。

## 3. 宏 Macros

*宏行为良好*

- **C-EVOCATIVE**：输入语法应「暗示」输出（声明结构体就用 `struct` 关键字、常量用分号结尾）。
- **C-MACRO-ATTR**：产出多个 item 的宏应允许对任意 item 加属性（如 `#[cfg(...)]`、`#[derive(...)]`）。
- **C-ANYWHERE**：item 宏在模块作用域与函数作用域内都能工作（测试都要覆盖）。
- **C-MACRO-VIS**：遵循 Rust 可见性——默认私有，显式 `pub` 才公开。
- **C-MACRO-TY**：`$t:ty` 片段应兼容基本类型、相对/绝对/向上路径、泛型。

## 4. 文档 Documentation

*文档充分*

- **C-CRATE-DOC**：crate 级文档详尽并含示例（RFC 1687）。
- **C-EXAMPLE**：每个公共 item 都有 rustdoc 示例，重点展示「为何要用」而非仅「怎么用」。
- **C-QUESTION-MARK**：示例用 `?`，不要 `try!` 或 `unwrap`（用 `# fn main() -> Result<...>` 隐藏样板）。
- **C-FAILURE**：函数文档包含 `# Errors` / `# Panics` / `# Safety` 小节说明失败与不变量。
- **C-LINK**：文档正文用超链接指向相关类型/方法（RFC 1574「链接一切」）。
- **C-METADATA**：`Cargo.toml` 含 `authors`、`description`、`license`、`repository`、
  `keywords`、`categories`（可选 `documentation`、`homepage`）。
- **C-RELNOTES**：Release notes 记录所有重要变更，明确标注破坏性变更，并为每次发布打 Git 标签。
- **C-HIDDEN**：rustdoc 不暴露无用实现细节（用 `#[doc(hidden)]`、`pub(crate)`）。

## 5. 可预测性 Predictability

*代码「所见即所为」*

- **C-SMART-PTR**：智能指针不加固有方法（用 `Box::into_raw(b)` 而非 `b.into_raw()`，避免与 Deref 目标混淆）。
- **C-CONV-SPECIFIC**：转换定义在更「具体」的类型上（`str` 提供 `as_bytes`/`from_utf8`，而非污染 `&[u8]`）；拿不准时优先 `to_`/
  `as_`/`into_` 而非 `from_`。
- **C-METHOD**：有明确接收者的函数写成方法（`impl Foo { fn frob(&self, ...) }`）。
- **C-NO-OUT**：不使用出参；用元组/结构体返回多个值（复用缓冲区是例外）。
- **C-OVERLOAD**：运算符重载不出人意料（`Mul` 只用于类乘法且满足结合律等）。
- **C-DEREF**：只有智能指针才实现 `Deref`/`DerefMut`。
- **C-CTOR**：构造器是 **静态固有方法**（`new`；转换构造器用 `from_`；多选项考虑 builder）。

## 6. 灵活性 Flexibility

*支持多样的真实场景*

- **C-INTERMEDIATE**：函数暴露有用的中间结果，避免调用方重复计算
  （如 `Vec::binary_search`、`HashMap::insert` 的返回）。
- **C-CALLER-CONTROL**：由调用方决定拷贝与放置——需要所有权就取所有权，不需要就借用；
  `Copy` 仅在确实必要时作为约束。
- **C-GENERIC**：用泛型（如 `IntoIterator`）最小化对参数的假设，提升复用性；权衡代码膨胀与签名可读性。
- **C-OBJECT**：可能作 trait object 使用的 trait 要保持对象安全（用 `where Self: Sized` 排除泛型方法）。

## 7. 类型安全 Type safety

*充分利用类型系统*

- **C-NEWTYPE**：用 newtype 做静态区分（如 `Miles(f64)` vs `Kilometers(f64)`）。
- **C-CUSTOM-TYPE**：用专门类型而非 `bool`/`Option` 传达含义（`Widget::new(Small, Round)` 而非 `(true, false)`）。
- **C-BITFLAG**：一组标志位用 `bitflags` crate，而非枚举。
- **C-BUILDER**：复杂值用 builder 模式构建；优先 **非消耗式** builder（方法取 `&mut self` 返回 `&mut Self`），
  消耗式 builder 则统一取/返 `self`。

## 8. 可靠性 Dependability

*不容易做错事*

- **C-VALIDATE**：函数校验参数，优先级： **静态强制**（用类型排除非法输入）＞ **动态校验**＞ `debug_assert!` ＞ 提供
  `_unchecked` 退出选项。
- **C-DTOR-FAIL**：析构器绝不失败（panic 时析构失败会 abort）；提供单独的 `close() -> Result`。
- **C-DTOR-BLOCK**：可能阻塞的析构提供非阻塞的替代方法。

## 9. 可调试性 Debuggability

*便于调试*

- **C-DEBUG**：所有公共类型实现 `Debug`（例外极少）。
- **C-DEBUG-NONEMPTY**：`Debug` 输出永不为空（空串输出 `""`，空 Vec 输出 `[]`）。

## 10. 向后兼容 Future proofing

*可自由演进而不破坏用户代码*

- **C-SEALED**：用「密封 trait」模式（私有 `Sealed` 父 trait）阻止下游实现，从而可非破坏地增方法。
- **C-STRUCT-PRIVATE**：结构体字段私有（公开字段会锁死表示并丧失不变量校验）。
- **C-NEWTYPE-HIDE**：用 newtype 隐藏实现细节（如包裹复杂迭代器类型）；也可权衡使用 `impl Trait`。
- **C-STRUCT-BOUNDS**：数据结构不重复可 derive 的 trait 约束
  （`struct Good<T>` 而非 `struct Bad<T: Clone + Debug>`）；
  例外：引用关联类型、`?Sized`、`Drop` 需要的约束。

## 11. 必要条件 Necessities

*一旦相关，就极其重要*

- **C-STABLE**：稳定 crate（≥1.0.0）的 **公共依赖**也必须稳定（注意错误类型的 `From` impl 会悄悄引入公共依赖）。
- **C-PERMISSIVE**：crate 及其依赖使用宽松许可证，推荐 `MIT OR Apache-2.0` 双许可以获得与 Rust 生态的最大兼容性。

---

## 快速评审清单

命名一致（C-CASE / C-CONV / C-GETTER / C-ITER / C-ITER-TY / C-FEATURE / C-WORD-ORDER）·
常见 trait 齐全（C-COMMON-TRAITS / C-CONV-TRAITS / C-COLLECT / C-SERDE / C-SEND-SYNC）·
错误规范（C-GOOD-ERR）· 文档与元数据完整（C-EXAMPLE / C-FAILURE / C-LINK / C-METADATA / C-RELNOTES）·
类型安全（C-NEWTYPE / C-CUSTOM-TYPE / C-BITFLAG / C-BUILDER）·
可靠（C-VALIDATE / C-DTOR-FAIL）· 可调试（C-DEBUG）· 兼容演进（C-SEALED / C-STRUCT-PRIVATE / C-STRUCT-BOUNDS）·
依赖稳定与许可证（C-STABLE / C-PERMISSIVE）。
