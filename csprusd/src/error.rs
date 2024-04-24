//! Error handling on the Casper platform.
use casper_types::ApiError;

/// Errors that the contract can return.
///
/// When an `Error` is returned from a smart contract, it is converted to an [`ApiError::User`].
///
/// While the code consuming this contract needs to define further error variants, it can
/// return those via the [`Error::User`] variant or equivalently via the [`ApiError::User`]
/// variant.
#[repr(u16)]
#[derive(Clone, Copy)]
pub enum CsprUSDError {
    /// Contract called from within an invalid context.
    InvalidContext = 60000,
    /// Spender does not have enough balance.
    InsufficientBalance = 60001,
    /// Spender does not have enough allowance approved.
    InsufficientAllowance = 60002,
    /// Operation would cause an integer overflow.
    Overflow = 60003,
    /// A required package hash was not specified.
    PackageHashMissing = 60004,
    /// The package hash specified does not represent a package.
    PackageHashNotPackage = 60005,
    /// An unknown error occurred.
    Phantom = 60008,
    /// Failed to read the runtime arguments provided.
    FailedToGetArgBytes = 60009,
    /// The flag to enable the mint and burn mode is invalid.
    InvalidEnableMBFlag = 60014,
    /// This contract instance cannot be initialized again.
    AlreadyInitialized = 60015,
    ///  The mint and burn mode is disabled.
    CannotTargetSelfUser = 60017,
    /// Contract is currently paused, not allowed to progress
    ContractPaused = 65000,
    /// Operation disallowed because account is not the pauser
    NotPauser = 65001,
    /// Pauser must be provided on contract initialization
    NoPauserProvided = 65002,
    /// Owner must be provided on contract initialization
    NoOwnerProvided = 65003,
    /// Operation disallowed because account is not the owner
    NotOwner = 65004,
    /// Operation disallowed because account is not a minter account
    NotMinter = 65005,
    /// Operation disallowed because account is blacklisted
    BlackListedAccount = 65006,
    /// Can't proceed because amount in the request exceeds the account's mint allowance
    ExceedsMintAllowance = 65007,
    /// Error while calling into Casper for creating dictionary
    FailedToCreateDictionary = 65011,
    /// Minting negative amounts is not allowed
    CannotMintZeroAmount = 65012,
    /// Operation disallowed because account is not the master minter
    NotMasterMinter = 65013,
    /// Operation disallowed because account is not the blacklister
    NotBlacklister = 65014,
    /// One can only burn a positive amount of tokens
    CannotBurnZeroAmount = 65015,
    /// An account can't burn more than its balance
    BurnExceedsBalance = 65016,
    /// Key is already blacklisted
    AlreadyBlacklisted = 65018,
    /// Key is not blacklisted
    NotBlacklisted = 65019,
}

impl From<CsprUSDError> for ApiError {
    fn from(error: CsprUSDError) -> Self {
        ApiError::User(error as u16)
    }
}
