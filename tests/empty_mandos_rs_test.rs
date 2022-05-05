use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();

    blockchain.register_contract_builder("file:output/flip.wasm", flip::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    elrond_wasm_debug::mandos_rs("mandos/init.scen.json", world());
}
