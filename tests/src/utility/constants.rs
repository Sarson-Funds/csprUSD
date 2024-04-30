use casper_types::{account::AccountHash, Key, PublicKey, SecretKey};
use once_cell::sync::Lazy;

pub const CSPR_USD_CONTRACT_WASM: &str = "csprusd.wasm";
pub const CSPR_USD_TEST_CONTRACT_WASM: &str = "csprusd_test_contract.wasm";
pub const NAME_KEY: &str = "name";
pub const SYMBOL_KEY: &str = "symbol";
pub const CONTRACT_HASH: &str = "csprUSD_contract_hash";
pub const PACKAGE_HASH: &str = "csprUSD_contract_package_hash";
pub const DECIMALS_KEY: &str = "decimals";
pub const TOTAL_SUPPLY_KEY: &str = "total_supply";
pub const BLACKLISTED_ADDRESSES_COUNT: &str = "blacklisted_addresses_index";
pub const BALANCES_KEY: &str = "balances";
pub const ALLOWANCES_KEY: &str = "allowances";
pub const OWNER: &str = "owner";
pub const AMOUNT: &str = "amount";

pub const ARG_NAME: &str = "name";
pub const ARG_SYMBOL: &str = "symbol";
pub const ARG_DECIMALS: &str = "decimals";

pub const _ERROR_INVALID_CONTEXT: u16 = 60000;
pub const ERROR_INSUFFICIENT_BALANCE: u16 = 60001;
pub const ERROR_INSUFFICIENT_ALLOWANCE: u16 = 60002;
pub const NON_BLACKLISTER: u16 = 65014;
pub const ALREADY_BLACKLISTED: u16 = 65018;
pub const NOT_BLACKLISTED: u16 = 65019;
pub const NOT_OWNER: u16 = 65004;
pub const BLACKLISTED_ACCOUNT: u16 = 65006;
pub const NOT_MASTER_MINTER: u16 = 65013;
pub const ERROR_OVERFLOW: u16 = 60003;
pub const ERROR_EXCEEDS_MINT_ALLOWANCE: u16 = 65007;

pub const TOKEN_NAME: &str = "CasperTest";
pub const TOKEN_SYMBOL: &str = "CSPRT";
pub const TOKEN_DECIMALS: u8 = 100;
pub const TOKEN_TOTAL_SUPPLY: u64 = 1_000_000_000;

pub const METHOD_TRANSFER: &str = "transfer";
pub const ARG_AMOUNT: &str = "amount";
pub const ARG_RECIPIENT: &str = "recipient";

pub const METHOD_APPROVE: &str = "approve";
pub const ARG_OWNER: &str = "owner";
pub const ARG_SPENDER: &str = "spender";

pub const METHOD_TRANSFER_FROM: &str = "transfer_from";

pub const CHECK_TOTAL_SUPPLY_ENTRYPOINT: &str = "check_total_supply";
pub const CHECK_BALANCE_OF_ENTRYPOINT: &str = "check_balance_of";
pub const CHECK_ALLOWANCE_OF_ENTRYPOINT: &str = "check_allowance_of";
pub const ARG_TOKEN_CONTRACT: &str = "token_contract";
pub const KEY: &str = "key";
pub const ADDRESS: &str = "address";
pub const RESULT_KEY: &str = "result";
pub const TEST_CONTRACT_PACKAGE_HASH: &str = "csprusd_test_contract_package_hash";

pub static ACCOUNT_1_SECRET_KEY: Lazy<SecretKey> =
    Lazy::new(|| SecretKey::secp256k1_from_bytes([221u8; 32]).unwrap());
pub static ACCOUNT_1_PUBLIC_KEY: Lazy<PublicKey> =
    Lazy::new(|| PublicKey::from(&*ACCOUNT_1_SECRET_KEY));
pub static ACCOUNT_1_ADDR: Lazy<AccountHash> = Lazy::new(|| ACCOUNT_1_PUBLIC_KEY.to_account_hash());

pub static ACCOUNT_2_SECRET_KEY: Lazy<SecretKey> =
    Lazy::new(|| SecretKey::secp256k1_from_bytes([212u8; 32]).unwrap());
pub static ACCOUNT_2_PUBLIC_KEY: Lazy<PublicKey> =
    Lazy::new(|| PublicKey::from(&*ACCOUNT_2_SECRET_KEY));
pub static ACCOUNT_2_ADDR: Lazy<AccountHash> = Lazy::new(|| ACCOUNT_2_PUBLIC_KEY.to_account_hash());

pub const TRANSFER_AMOUNT_1: u64 = 200_001;
pub const TRANSFER_AMOUNT_2: u64 = 19_999;
pub const ALLOWANCE_AMOUNT_1: u64 = 456_789;
pub const ALLOWANCE_AMOUNT_2: u64 = 87_654;

pub const METHOD_TRANSFER_AS_STORED_CONTRACT: &str = "transfer_as_stored_contract";
pub const METHOD_APPROVE_AS_STORED_CONTRACT: &str = "approve_as_stored_contract";
pub const METHOD_FROM_AS_STORED_CONTRACT: &str = "transfer_from_as_stored_contract";

pub const TOKEN_OWNER_ADDRESS_1: Key = Key::Account(AccountHash::new([42; 32]));
pub const TOKEN_OWNER_AMOUNT_1: u64 = 1_000_000;
pub const TOKEN_OWNER_ADDRESS_2: Key = Key::Hash([42; 32]);
pub const TOKEN_OWNER_AMOUNT_2: u64 = 2_000_000;

pub const METHOD_MINT: &str = "mint";
pub const METHOD_BURN: &str = "burn";
pub const DECREASE_ALLOWANCE: &str = "decrease_allowance";
pub const INCREASE_ALLOWANCE: &str = "increase_allowance";
pub const METHOD_TRANSFER_OWNERSHIP: &str = "transfer_ownership";
pub const UPDATE_BLACKLISTER_ENTRY_POINT: &str = "update_blacklister";

pub const IS_PAUSED: &str = "is_paused";
pub const PAUSER: &str = "pauser";
pub const MASTER_MINTER: &str = "master_minter";
pub const BLACKLISTER: &str = "blacklister";
pub const METHOD_PAUSE: &str = "pause_contract";
pub const METHOD_UNPAUSE: &str = "unpause_contract";
pub const METHOD_UPDATE_PAUSER: &str = "update_pauser";
pub const CONTRACT_PAUSED_ERROR_CODE: u16 = 65000;
pub const NON_PAUSER_ERROR_CODE: u16 = 65001;
pub const CANNOT_TRANSFER_ZERO_AMOUNT: u16 = 65017;

pub const ARG_CURRENCY: &str = "currency";
pub const TOKEN_CURRENCY: &str = "SOME_CCY";

pub const CONFIGURE_MINTER_ENTRY_POINT_NAME: &str = "configure_minter";
pub const UPDATE_MASTER_MINTER_ENTRY_POINT_NAME: &str = "update_master_minter";
pub const BLACKLIST: &str = "blacklist";
pub const UN_BLACKLIST: &str = "un_blacklist";
pub const APPROVE_ENTRY_POINT_NAME: &str = "approve";
pub const ARG_MASTER_MINTER: &str = "master_minter";
pub const RECIPIENT: &str = "recipient";
pub const MINTER: &str = "minter";
pub const SPENDER: &str = "spender";
pub const MINTER_ALLOWED: &str = "minter_allowed";
pub const NEW: &str = "new";
