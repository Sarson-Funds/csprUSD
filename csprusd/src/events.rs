use casper_types::{Key, PublicKey, U256};

use casper_event_standard::{emit, Event, Schemas};

pub enum Event {
    Mint(Mint),
    Burn(Burn),
    Pause(Pause),
    Unpause(Unpause),
    PauserChanged(NewPauser),
    MasterMinterChanged(MasterMinterChanged),
    Blacklisted(Blacklisted),
    UnBlacklisted(UnBlacklisted),
    BlacklisterChanged(BlacklisterChanged),
    OwnershipTransferred(OwnershipTransferred),
    MinterConfigured(MinterConfigured),
    MinterRemoved(MinterRemoved),
    SetAllowance(SetAllowance),
    IncreaseAllowance(IncreaseAllowance),
    DecreaseAllowance(DecreaseAllowance),
    Transfer(Transfer),
    TransferFrom(TransferFrom),
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Mint {
    pub minter: Key,
    pub recipient: Key,
    pub amount: U256,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Burn {
    pub minter: Key,
    pub amount: U256,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Pause {}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Unpause {}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct NewPauser {
    pub new_pauser: PublicKey,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Blacklisted {
    pub key: Key,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct UnBlacklisted {
    pub key: Key,
}
#[derive(Event, Debug, PartialEq, Eq)]
pub struct BlacklisterChanged {
    pub new_blacklister: PublicKey,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct MasterMinterChanged {
    pub new_master_minter: Key,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct OwnershipTransferred {
    pub new_owner: Key,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct MinterConfigured {
    pub minter: Key,
    pub minter_allowance: U256,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct MinterRemoved {
    pub minter: Key,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct SetAllowance {
    pub owner: Key,
    pub spender: Key,
    pub allowance: U256,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct IncreaseAllowance {
    pub owner: Key,
    pub spender: Key,
    pub allowance: U256,
    pub inc_by: U256,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct DecreaseAllowance {
    pub owner: Key,
    pub spender: Key,
    pub allowance: U256,
    pub decr_by: U256,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct Transfer {
    pub sender: Key,
    pub recipient: Key,
    pub amount: U256,
}

#[derive(Event, Debug, PartialEq, Eq)]
pub struct TransferFrom {
    pub spender: Key,
    pub owner: Key,
    pub recipient: Key,
    pub amount: U256,
}

pub fn emit_event(event: Event) {
    match event {
        Event::Mint(ev) => emit(ev),
        Event::Burn(ev) => emit(ev),
        Event::Pause(ev) => emit(ev),
        Event::Unpause(ev) => emit(ev),
        Event::PauserChanged(ev) => emit(ev),
        Event::Blacklisted(ev) => emit(ev),
        Event::UnBlacklisted(ev) => emit(ev),
        Event::BlacklisterChanged(ev) => emit(ev),
        Event::MasterMinterChanged(ev) => emit(ev),
        Event::OwnershipTransferred(ev) => emit(ev),
        Event::MinterConfigured(ev) => emit(ev),
        Event::MinterRemoved(ev) => emit(ev),
        Event::SetAllowance(ev) => emit(ev),
        Event::IncreaseAllowance(ev) => emit(ev),
        Event::DecreaseAllowance(ev) => emit(ev),
        Event::Transfer(ev) => emit(ev),
        Event::TransferFrom(ev) => emit(ev),
    }
}

pub fn init_events() {
    let schemas = Schemas::new()
        .with::<Mint>()
        .with::<Burn>()
        .with::<Pause>()
        .with::<Unpause>()
        .with::<NewPauser>()
        .with::<MasterMinterChanged>()
        .with::<Blacklisted>()
        .with::<UnBlacklisted>()
        .with::<BlacklisterChanged>()
        .with::<OwnershipTransferred>()
        .with::<MinterConfigured>()
        .with::<MinterRemoved>()
        .with::<SetAllowance>()
        .with::<IncreaseAllowance>()
        .with::<DecreaseAllowance>()
        .with::<Transfer>()
        .with::<TransferFrom>();
    casper_event_standard::init(schemas);
}
