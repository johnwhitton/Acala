//! Mocks for the currencies module.

#![cfg(test)]

use frame_support::{impl_outer_event, impl_outer_origin, ord_parameter_types, parameter_types};
use orml_traits::parameter_type_with_key;
use pallet_balances;
use primitives::{mocks::MockAddressMapping, TokenSymbol};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{AccountIdConversion, IdentityLookup},
	AccountId32, ModuleId, Perbill,
};

use tokens;

use super::*;
use frame_system::EnsureSignedBy;
use module_evm::GenesisAccount;
use sp_core::{bytes::from_hex, H160};
use sp_std::collections::btree_map::BTreeMap;
use sp_std::str::FromStr;

mod currencies {
	pub use crate::Event;
}

impl_outer_event! {
	pub enum TestEvent for Runtime {
		frame_system<T>,
		currencies<T>,
		tokens<T>,
		pallet_balances<T>,
		module_evm<T>,
	}
}

impl_outer_origin! {
	pub enum Origin for Runtime {}
}

// Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Runtime;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: u32 = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}

pub type AccountId = AccountId32;
impl frame_system::Config for Runtime {
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type BlockWeights = ();
	type BlockLength = ();
	type Version = ();
	type PalletInfo = ();
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = ();
	type SystemWeightInfo = ();
}
pub type System = frame_system::Module<Runtime>;

type Balance = u128;

parameter_type_with_key! {
	pub ExistentialDeposits: |currency_id: CurrencyId| -> Balance {
		Default::default()
	};
}

parameter_types! {
	pub DustAccount: AccountId = ModuleId(*b"orml/dst").into_account();
}

impl tokens::Config for Runtime {
	type Event = TestEvent;
	type Balance = Balance;
	type Amount = i64;
	type CurrencyId = CurrencyId;
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = tokens::TransferDust<Runtime, DustAccount>;
	type WeightInfo = ();
}
pub type Tokens = tokens::Module<Runtime>;

pub const NATIVE_CURRENCY_ID: CurrencyId = CurrencyId::Token(TokenSymbol::ACA);
pub const X_TOKEN_ID: CurrencyId = CurrencyId::Token(TokenSymbol::AUSD);

parameter_types! {
	pub const GetNativeCurrencyId: CurrencyId = NATIVE_CURRENCY_ID;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = TestEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
}

pub type PalletBalances = pallet_balances::Module<Runtime>;

parameter_types! {
	pub const MinimumPeriod: u64 = 1000;
}
impl pallet_timestamp::Config for Runtime {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub const ContractExistentialDeposit: u64 = 1;
	pub const TransferMaintainerDeposit: u64 = 1;
	pub NetworkContractSource: H160 = H160::default();
}

ord_parameter_types! {
	pub const CouncilAccount: AccountId32 = AccountId32::from([1u8; 32]);
	pub const TreasuryAccount: AccountId32 = AccountId32::from([2u8; 32]);
	pub const NetworkContractAccount: AccountId32 = AccountId32::from([0u8; 32]);
	pub const StorageDepositPerByte: u128 = 10;
	pub const StorageDefaultQuota: u32 = 0x6000;
	pub const DeveloperDeposit: u64 = 1000;
	pub const DeploymentFee: u64 = 200;
}

impl module_evm::Config for Runtime {
	type AddressMapping = MockAddressMapping;
	type Currency = PalletBalances;
	type MergeAccount = ();
	type ContractExistentialDeposit = ContractExistentialDeposit;
	type TransferMaintainerDeposit = TransferMaintainerDeposit;
	type StorageDepositPerByte = StorageDepositPerByte;
	type StorageDefaultQuota = StorageDefaultQuota;

	type Event = TestEvent;
	type Precompiles = ();
	type ChainId = ();
	type GasToWeight = ();
	type NetworkContractOrigin = EnsureSignedBy<NetworkContractAccount, AccountId>;
	type NetworkContractSource = NetworkContractSource;

	type DeveloperDeposit = DeveloperDeposit;
	type DeploymentFee = DeploymentFee;
	type TreasuryAccount = TreasuryAccount;
	type FreeDeploymentOrigin = EnsureSignedBy<CouncilAccount, AccountId32>;

	type WeightInfo = ();
}

pub type EVM = module_evm::Module<Runtime>;

impl module_evm_bridge::Config for Runtime {
	type EVM = EVM;
}

pub type EVMBridge = module_evm_bridge::Module<Runtime>;

impl Config for Runtime {
	type Event = TestEvent;
	type MultiCurrency = Tokens;
	type NativeCurrency = AdaptedBasicCurrency;
	type WeightInfo = ();
	type AddressMapping = MockAddressMapping;
	type EVMBridge = EVMBridge;
}
pub type Currencies = Module<Runtime>;
pub type NativeCurrency = Currency<Runtime, GetNativeCurrencyId>;
pub type AdaptedBasicCurrency = BasicCurrencyAdapter<Runtime, PalletBalances, i64, u64>;

pub fn erc20_address() -> H160 {
	H160::from_str("2000000000000000000000000000000000000001").unwrap()
}

pub fn alice() -> AccountId {
	<Runtime as Config>::AddressMapping::get_account_id(
		&H160::from_str("1000000000000000000000000000000000000001").unwrap(),
	)
}

pub fn bob() -> AccountId {
	<Runtime as Config>::AddressMapping::get_account_id(
		&H160::from_str("1000000000000000000000000000000000000002").unwrap(),
	)
}

pub const ALICE: AccountId = AccountId::new([1u8; 32]);
pub const BOB: AccountId = AccountId::new([2u8; 32]);
pub const EVA: AccountId = AccountId::new([5u8; 32]);

pub const ID_1: LockIdentifier = *b"1       ";

pub struct ExtBuilder {
	endowed_accounts: Vec<(AccountId, CurrencyId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			endowed_accounts: vec![],
		}
	}
}

impl ExtBuilder {
	pub fn balances(mut self, endowed_accounts: Vec<(AccountId, CurrencyId, Balance)>) -> Self {
		self.endowed_accounts = endowed_accounts;
		self
	}

	pub fn one_hundred_for_alice_n_bob(self) -> Self {
		self.balances(vec![
			(ALICE, NATIVE_CURRENCY_ID, 100),
			(BOB, NATIVE_CURRENCY_ID, 100),
			(ALICE, X_TOKEN_ID, 100),
			(BOB, X_TOKEN_ID, 100),
		])
	}

	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.unwrap();

		pallet_balances::GenesisConfig::<Runtime> {
			balances: self
				.endowed_accounts
				.clone()
				.into_iter()
				.filter(|(_, currency_id, _)| *currency_id == NATIVE_CURRENCY_ID)
				.map(|(account_id, _, initial_balance)| (account_id, initial_balance))
				.collect::<Vec<_>>(),
		}
		.assimilate_storage(&mut t)
		.unwrap();

		tokens::GenesisConfig::<Runtime> {
			endowed_accounts: self
				.endowed_accounts
				.into_iter()
				.filter(|(_, currency_id, _)| *currency_id != NATIVE_CURRENCY_ID)
				.collect::<Vec<_>>(),
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let mut accounts = BTreeMap::new();
		let mut storage = BTreeMap::new();
		storage.insert(
			H256::from_str("0000000000000000000000000000000000000000000000000000000000000002").unwrap(),
			H256::from_str("00000000000000000000000000000000ffffffffffffffffffffffffffffffff").unwrap(),
		);
		storage.insert(
			H256::from_str("e6f18b3f6d2cdeb50fb82c61f7a7a249abf7b534575880ddcfde84bba07ce81d").unwrap(),
			H256::from_str("00000000000000000000000000000000ffffffffffffffffffffffffffffffff").unwrap(),
		);
		accounts.insert(
			erc20_address(),
			GenesisAccount {
				nonce: 1,
				balance: 0,
				storage,
				code: from_hex(include!("../../evm-bridge/src/erc20_demo_contract")).unwrap(),
			},
		);
		module_evm::GenesisConfig::<Runtime> {
			accounts,
			network_contract_index: 2048,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		t.into()
	}
}
