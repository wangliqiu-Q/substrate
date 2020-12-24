# Own Version




[Header, Block, Digest](/primitives/runtime/src/generic)


[validate_transaction, apply_extrinsic_with_len](frame/executive/src/lib.rs)


![交易流程](.note/1.jpg)

runtime api 定义 `decl_runtime_apis!`
[](primitives/api/src/lib.rs) 
[](primitives/block-builder/src/lib.rs)
runtime api 实现 `impl_runtime_apis!`
[](bin/node/runtime/src/lib.rs) 

在Pos中，一般都会推选出一个proposer来构建出这个区块
Proposer -> propose_with -> BlockBuilder::push -> node/runtime `impl_runtime_apis!` fn apply_extrinsic
[](client/basic-authorship/src/basic_authorship.rs)

[](client/service/src/client/client.rs)



