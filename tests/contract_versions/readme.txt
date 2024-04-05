These folders hold different versions of the csprUSD contract, 
    called v0, v1, etc in the context of integration testing of contract upgrades

The goal is that the integration test first installs v0, then upgrades to a later version
    After this the test proves that functionality from the newer version is available

The major change from v0 to v1: blacklister is a PublicKey (v1), instead of a Key::Account (v0)


Never versions must have an upgrade_contract() mechanism:
#[no_mangle]
pub extern "C" fn call() {
    match runtime::get_key(CONTRACT_VERSION) {
        None => {
            // The given key doesn't exist, so install the contract.
            install_contract();
            // Next, upgrade the contract.
            upgrade_contract_blacklister_key_to_publickey();
        }
        Some(_contract_key) => {
            // The stored contract and key exist, so upgrade the contract.
            upgrade_contract_blacklister_key_to_publickey();
        }
    }
}