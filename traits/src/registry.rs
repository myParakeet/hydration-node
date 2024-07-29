use frame_support::__private::DispatchError;
use sp_std::vec::Vec;

//TODO: inherit this in InpsectRegistry?!
pub trait InspectSufficiency<AssetId> {
	fn is_sufficient(asset: AssetId) -> bool;

	fn is_trade_supported(from: AssetId, into: AssetId) -> bool;
}

//TODO: replace this to better place
//TODO: give better name
pub trait InsufficientAssetTrader<AccountId, AssetId, Amount>: InspectSufficiency<AssetId> {
	fn buy(
		origin: &AccountId,
		dest: &AccountId,
		from: AssetId,
		into: AssetId,
		amount: Amount,
		max_limit: Amount,
	) -> DispatchResult;

	fn pool_trade_fee(swap_amount: Amount) -> Result<Amount, DispatchError>;

	fn get_amount_in_for_out(
		insuff_asset_id: AssetId,
		asset_out: AssetId,
		asset_out_amount: Amount,
	) -> Result<Amount, DispatchError>;
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum AssetKind {
	Token,
	XYK,
	StableSwap,
	Bond,
	External,
}

// Deprecated.
// TODO: the following macro is commented out for a reason for now - due to failing clippy in CI
// #[deprecated(since = "0.6.0", note = "Please use `AccountIdFor` instead")]
pub trait AssetPairAccountIdFor<AssetId, AccountId> {
	fn from_assets(asset_a: AssetId, asset_b: AssetId, identifier: &str) -> AccountId;
}

/// Abstraction over account id and account name creation for `Assets`
pub trait AccountIdFor<Assets> {
	type AccountId;

	/// Create account id for given assets and an identifier
	fn from_assets(assets: &Assets, identifier: Option<&[u8]>) -> Self::AccountId;

	/// Create a name to uniquely identify a share token for given assets and an identifier.
	fn name(assets: &Assets, identifier: Option<&[u8]>) -> Vec<u8>;
}

use frame_support::dispatch::{DispatchResult, Parameter};

pub trait Inspect {
	type AssetId: Parameter;
	type Location: Parameter;

	fn is_sufficient(id: Self::AssetId) -> bool;

	fn exists(id: Self::AssetId) -> bool;

	fn decimals(id: Self::AssetId) -> Option<u8>;

	fn asset_type(id: Self::AssetId) -> Option<AssetKind>;

	fn is_banned(id: Self::AssetId) -> bool;

	fn asset_name(id: Self::AssetId) -> Option<Vec<u8>>;

	fn asset_symbol(id: Self::AssetId) -> Option<Vec<u8>>;

	fn existential_deposit(id: Self::AssetId) -> Option<u128>;
}

#[allow(clippy::too_many_arguments)]
pub trait Create<Balance>: Inspect {
	type Error;
	type Name: Parameter + TryFrom<sp_std::prelude::Vec<u8>>;
	type Symbol: Parameter + TryFrom<sp_std::prelude::Vec<u8>>;

	fn register_asset(
		asset_id: Option<Self::AssetId>,
		name: Option<Self::Name>,
		kind: AssetKind,
		existential_deposit: Option<Balance>,
		symbol: Option<Self::Symbol>,
		decimals: Option<u8>,
		location: Option<Self::Location>,
		xcm_rate_limit: Option<Balance>,
		is_sufficient: bool,
	) -> Result<Self::AssetId, Self::Error>;

	fn register_insufficient_asset(
		asset_id: Option<Self::AssetId>,
		name: Option<Self::Name>,
		kind: AssetKind,
		existential_deposit: Option<Balance>,
		symbol: Option<Self::Symbol>,
		decimals: Option<u8>,
		location: Option<Self::Location>,
		xcm_rate_limit: Option<Balance>,
	) -> Result<Self::AssetId, Self::Error> {
		Self::register_asset(
			asset_id,
			name,
			kind,
			existential_deposit,
			symbol,
			decimals,
			location,
			xcm_rate_limit,
			false,
		)
	}

	fn register_sufficient_asset(
		asset_id: Option<Self::AssetId>,
		name: Option<Self::Name>,
		kind: AssetKind,
		existential_deposit: Balance,
		symbol: Option<Self::Symbol>,
		decimals: Option<u8>,
		location: Option<Self::Location>,
		xcm_rate_limit: Option<Balance>,
	) -> Result<Self::AssetId, Self::Error> {
		Self::register_asset(
			asset_id,
			name,
			kind,
			Some(existential_deposit),
			symbol,
			decimals,
			location,
			xcm_rate_limit,
			true,
		)
	}

	fn get_or_register_asset(
		name: Self::Name,
		kind: AssetKind,
		existential_deposit: Option<Balance>,
		symbol: Option<Self::Symbol>,
		decimals: Option<u8>,
		location: Option<Self::Location>,
		xcm_rate_limit: Option<Balance>,
		is_sufficient: bool,
	) -> Result<Self::AssetId, Self::Error>;

	fn get_or_register_sufficient_asset(
		name: Self::Name,
		kind: AssetKind,
		existential_deposit: Balance,
		symbol: Option<Self::Symbol>,
		decimals: Option<u8>,
		location: Option<Self::Location>,
		xcm_rate_limit: Option<Balance>,
	) -> Result<Self::AssetId, Self::Error> {
		Self::get_or_register_asset(
			name,
			kind,
			Some(existential_deposit),
			symbol,
			decimals,
			location,
			xcm_rate_limit,
			true,
		)
	}

	fn get_or_register_insufficient_asset(
		name: Self::Name,
		kind: AssetKind,
		existential_deposit: Option<Balance>,
		symbol: Option<Self::Symbol>,
		decimals: Option<u8>,
		location: Option<Self::Location>,
		xcm_rate_limit: Option<Balance>,
	) -> Result<Self::AssetId, Self::Error> {
		Self::get_or_register_asset(
			name,
			kind,
			existential_deposit,
			symbol,
			decimals,
			location,
			xcm_rate_limit,
			false,
		)
	}
}

pub trait Mutate<Balance>: Inspect {
	type Error;

	/// Set location for existing asset id if it wasn't set yet.
	fn set_location(asset_id: Self::AssetId, location: Self::Location) -> Result<(), Self::Error>;
}
