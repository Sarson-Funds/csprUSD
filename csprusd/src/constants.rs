/// Name of named-key for `name`.
pub const NAME: &str = "name";
/// Name of named-key for `symbol`
pub const SYMBOL: &str = "symbol";
/// Name of named-key for `decimals`
pub const DECIMALS: &str = "decimals";
/// Name of dictionary-key for `balances`
pub const BALANCES: &str = "balances";
/// Name of dictionary-key for `allowances`
pub const ALLOWANCES: &str = "allowances";
/// Name of named-key for `total_supply`
pub const TOTAL_SUPPLY: &str = "total_supply";

pub const CONTRACT_PACKAGE_HASH: &str = "csprUSD_contract_package_hash";
pub const CONTRACT_HASH: &str = "csprUSD_contract_hash";
pub const CONTRACT_ACCESS: &str = "csprUSD_contract_package_access";
pub const CONTRACT_VERSION: &str = "csprUSD_contract_version";

/// Name of `name` entry point.
pub const NAME_ENTRY_POINT_NAME: &str = "name";
/// Name of `symbol` entry point.
pub const SYMBOL_ENTRY_POINT_NAME: &str = "symbol";
/// Name of `decimals` entry point.
pub const DECIMALS_ENTRY_POINT_NAME: &str = "decimals";
/// Name of `balance_of` entry point.
pub const BALANCE_OF_ENTRY_POINT_NAME: &str = "balance_of";
/// Name of `transfer` entry point.
pub const TRANSFER_ENTRY_POINT_NAME: &str = "transfer";
/// Name of `approve` entry point.
pub const APPROVE_ENTRY_POINT_NAME: &str = "approve";
/// Name of `allowance` entry point.
pub const ALLOWANCE_ENTRY_POINT_NAME: &str = "allowance";
/// Name of `transfer_from` entry point.
pub const TRANSFER_FROM_ENTRY_POINT_NAME: &str = "transfer_from";
/// Name of `total_supply` entry point.
pub const TOTAL_SUPPLY_ENTRY_POINT_NAME: &str = "total_supply";
/// Name of `transfer_from` entry point.
pub const MINT_ENTRY_POINT_NAME: &str = "mint";
/// Name of `burn` entry point.
pub const BURN_ENTRY_POINT_NAME: &str = "burn";
/// Name of `init` entry point.
pub const INIT_ENTRY_POINT_NAME: &str = "init";

pub const INCREASE_ALLOWANCE_ENTRY_POINT_NAME: &str = "increase_allowance";
pub const DECREASE_ALLOWANCE_ENTRY_POINT_NAME: &str = "decrease_allowance";

/// Name of `address` runtime argument.
pub const ADDRESS: &str = "address";
/// Name of `owner` runtime argument.
pub const OWNER: &str = "owner";
/// Name of `spender` runtime argument.
pub const SPENDER: &str = "spender";
/// Name of `amount` runtime argument.
pub const AMOUNT: &str = "amount";
/// Name of `recipient` runtime argument.
pub const RECIPIENT: &str = "recipient";
pub const PACKAGE_HASH: &str = "package_hash";
pub const SECURITY_BADGES: &str = "security_badges";
pub const ADMIN_LIST: &str = "admin_list";
pub const MINTER_LIST: &str = "minter_list";
pub const BURNER_LIST: &str = "burner_list";
pub const NONE_LIST: &str = "none_list";
pub const MINT_AND_BURN_LIST: &str = "mint_and_burn_list";
pub const ENABLE_MINT_BURN: &str = "enable_mint_burn";

pub const IS_PAUSED: &str = "is_paused";
pub const PAUSER: &str = "pauser";
pub const BLACKLISTER: &str = "blacklister";
pub const PAUSE_ENTRY_POINT_NAME: &str = "pause_contract";
pub const UNPAUSE_ENTRY_POINT_NAME: &str = "unpause_contract";
pub const UPDATE_PAUSER_ENTRY_POINT_NAME: &str = "update_pauser";
pub const TRANSFER_OWNERSHIP_ENTRY_POINT_NAME: &str = "transfer_ownership";
pub const MASTER_MINTER: &str = "master_minter";

pub const BLACKLISTED_ADDRESSES_COUNT: &str = "blacklisted_addresses_index";
pub const DICT_INDEX_TO_BLACKLISTED_ADDR: &str = "index_to_blacklisted_addr";
pub const DICT_BLACKLISTED_ADDR_TO_INDEX: &str = "blacklisted_addr_to_index";
pub const MINTERS: &str = "minters";
pub const CURRENCY: &str = "currency";
pub const MINTER_ALLOWED: &str = "minter_allowed";
pub const MINTER: &str = "minter";
pub const CONFIGURE_MINTER_ENTRY_POINT_NAME: &str = "configure_minter";
pub const REMOVE_MINTER_ENTRY_POINT_NAME: &str = "remove_minter";
pub const MINTER_ALLOWANCE_ENTRY_POINT_NAME: &str = "minter_allowance";
pub const IS_MINTER_ENTRY_POINT_NAME: &str = "is_minter";
pub const IS_BLACKLISTED_ENTRY_POINT_NAME: &str = "is_blacklisted";
pub const BLACKLIST_ENTRY_POINT_NAME: &str = "blacklist";
pub const UN_BLACKLIST_ENTRY_POINT_NAME: &str = "un_blacklist";
pub const UPDATE_BLACKLISTER_ENTRY_POINT_NAME: &str = "update_blacklister";
pub const UPDATE_MASTER_MINTER_ENTRY_POINT_NAME: &str = "update_master_minter";
pub const ACCOUNT: &str = "account";
pub const OWNER_ENTRY_POINT_NAME: &str = "owner";
pub const PAUSER_ENTRY_POINT_NAME: &str = "pauser";
pub const IS_PAUSED_ENTRY_POINT_NAME: &str = "is_paused";
pub const MASTER_MINTER_ENTRY_POINT_NAME: &str = "master_minter";
pub const BLACKLISTER_ENTRY_POINT_NAME: &str = "blacklister";
pub const NEW: &str = "new";
pub const KEY: &str = "key";
