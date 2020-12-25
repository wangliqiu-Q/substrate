# Own Version

> 状态区块链结构:
> * 当前状态是从 genesis 开始，通过交易产生了状态变更，不断累计出来的。
> * 块中存储的是`状态迁移`的记录
> * 状态不是储存在块中，而是每个节点自身独立维护的。

> 状态区块链特性:
> * 在最高块下的节点中可以获取到当前所有对象的状态。
> * 块中含有对这个块下的状态的证明。（state_root 作为当前这个区块进行共识的状态证明）
> * 可以通过任意一个历史区块，取到在这个历史区块下的状态。（每个块的状态都存储）

在Substrate中MPT简称为trie。
Substrate采用了和以太坊一样的模型，所以以上3个特性都满足。
在打包执行当前区块时使用的root即是上一个区块的root，也就是打包区块时取当前最新的状态。在Substrate中是block_id


Runtime
-----
Runtime 的依赖:

    sp-std sp-io
    | |
    | frame-support, frame-system
    | /
    frame-assets, frame-balances ...
    | /
    node-runtime
> 以上Runtime 的依赖树中出现的crate都必须遵循以下规则：
> * lib.rs 的第一行必须有`#![cfg_attr(not(feature = "std"), no_std)]`
> * 引入标准库的类型，如Vec, Result，BTreeMap等等，必须通过 [sp-std](primitives/std) 引入。
> * 要使用没有在 sp-std 中引入的类型，则通过 if_std! 来实现条件编译。
例如 [fn issue](frame/assets/src/lib.rs)  [pub struct Vicinity](frame/evm/src/backend.rs)

因此引入一个第三方库兼容Runtime wasm的编译环境是不推荐的，若只是需要一些小的工具函数，那么直接拷贝进入runtime为妙。若是需要一些密码库，
那么请参考Substrate实现ed25519，escda等密码学函数的方法，抽离定义，将实现通过runtime_interface放在native下实现。

![Runtime结构](https://pic4.zhimg.com/80/v2-39aaf60f0abde1e6cb28196cef0f8a4b_720w.jpg)
[Header, Block, Digest](/primitives/runtime/src/generic)
[validate_transaction, apply_extrinsic_with_len](frame/executive/src/lib.rs)


流程
-----
![交易流程](.note/1.jpg)

runtime api 定义 `decl_runtime_apis!`
[](primitives/api/src/lib.rs) 
[](primitives/block-builder/src/lib.rs)
runtime api 实现 `impl_runtime_apis!`
[](bin/node/runtime/src/lib.rs) 


client 实现 BlockBuilderProvider::new_block -> BlockBuilder::new 关联方法 -> 
client 通过 ProvideRuntimeApi::runtime_api 获取到ApiRef，通过ApiRef调用initialize_block_with_context -> 
BlockBuilder::new返回的 BlockBuilder 实例会持有这个ApiRef，因而在下面的流程 BlockBuilder::push 中可以通过api调用apply_extrinsic_with_context。
[](client/service/src/client/client.rs)
[](client/block-builder/src/lib.rs)


在Pos中，一般都会推选出一个proposer来构建出这个区块
Proposer -> propose_with -> BlockBuilder::push -> node/runtime `impl_runtime_apis!` fn apply_extrinsic
[](client/basic-authorship/src/basic_authorship.rs)










