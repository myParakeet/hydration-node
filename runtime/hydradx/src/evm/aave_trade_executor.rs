use crate::evm::executor::{BalanceOf, CallResult, NonceIdOf};
use crate::evm::precompiles::erc20_mapping::HydraErc20Mapping;
use crate::evm::precompiles::handle::EvmDataWriter;
use crate::evm::{Erc20Currency, Executor};
use crate::Runtime;
use ethabi::{decode, ParamType};
use evm::ExitReason;
use evm::ExitReason::Succeed;
use frame_support::dispatch::DispatchResult;
use frame_support::ensure;
use frame_system::ensure_signed;
use frame_system::pallet_prelude::OriginFor;
use hex_literal::hex;
use hydradx_traits::evm::{CallContext, Erc20Encoding, Erc20Mapping, InspectEvmAccounts, ERC20, EVM};
use hydradx_traits::router::{ExecutorError, PoolType, TradeExecution};
use hydradx_traits::BoundErc20;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use pallet_democracy::EncodeInto;
use pallet_evm::GasWeightMapping;
use pallet_liquidation::BorrowingContract;
use polkadot_xcm::v3::MultiLocation;
use primitive_types::{H256, U256};
use primitives::{AccountId, AssetId, Balance, EvmAddress};
use scale_info::prelude::string::String;
use sp_arithmetic::traits::SaturatedConversion;
use sp_arithmetic::FixedU128;
use sp_runtime::format;
use sp_runtime::traits::CheckedConversion;
use sp_runtime::{DispatchError, RuntimeDebug};
use sp_std::boxed::Box;
use sp_std::marker::PhantomData;
use sp_std::vec;
use sp_std::vec::Vec;
use pallet_genesis_history::migration::Weight;

pub struct AaveTradeExecutor<T>(PhantomData<T>);

pub type Aave = AaveTradeExecutor<Runtime>;

#[module_evm_utility_macro::generate_function_selector]
#[derive(RuntimeDebug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum Function {
	// Pool
	Supply = "supply(address,uint256,address,uint16)",
	Withdraw = "withdraw(address,uint256,address)",
	GetReserveData = "getReserveData(address)",
	GetConfiguration = "getConfiguration(address)",

	// AToken
	UnderlyingAssetAddress = "UNDERLYING_ASSET_ADDRESS()",
	ScaledTotalSupply = "scaledTotalSupply()",
}

struct ReserveData {
	configuration: U256,
	liquidity_index: U256,
	current_liquidity_rate: U256,
	variable_borrow_index: U256,
	current_variable_borrow_rate: U256,
	current_stable_borrow_rate: U256,
	last_update_timestamp: U256,
	id: u16,
	atoken_address: EvmAddress,
	stable_debt_token_address: EvmAddress,
	variable_debt_token_address: EvmAddress,
	interest_rate_strategy_address: EvmAddress,
	accrued_to_treasury: U256,
}

impl ReserveData {
	fn decimals(&self) -> u8 {
		//bit 48-55: Decimals
		let mask = U256::from(0xFF) << 48;
		((self.configuration & mask) >> 48).saturated_into()
	}

	fn supply_cap_raw(&self) -> U256 {
		//bit 116-151 supply cap in whole tokens, supplyCap == 0 => no cap
		let mask = U256::from((1u128 << 36) - 1) << 116;
		(self.configuration & mask) >> 116
	}

	fn supply_cap(&self) -> U256 {
		if self.supply_cap_raw().is_zero() {
			U256::MAX
		} else {
			self.supply_cap_raw().saturating_mul(
				U256::from(10)
					.checked_pow(self.decimals().into())
					.unwrap_or_else(|| U256::one()),
			)
		}
	}

	fn current_supply(&self, scaled_total_supply: U256) -> U256 {
		scaled_total_supply
			.saturating_add(self.accrued_to_treasury)
			.saturating_mul(self.liquidity_index)
			/ U256::from(10).pow(27.into())
	}

	fn available_supply(&self, scaled_total_supply: U256) -> U256 {
		self.supply_cap()
			.saturating_sub(self.current_supply(scaled_total_supply))
	}
}

const TRADE_GAS_LIMIT: u64 = 500_000;
const VIEW_GAS_LIMIT: u64 = 100_000;

impl<T> AaveTradeExecutor<T>
where
	T: pallet_evm::Config
		+ pallet_asset_registry::Config<AssetId = AssetId>
		+ pallet_liquidation::Config
		+ pallet_evm_accounts::Config
		+ frame_system::Config,
	T::AssetNativeLocation: Into<MultiLocation>,
	BalanceOf<T>: TryFrom<U256> + Into<U256>,
	T::AddressMapping: pallet_evm::AddressMapping<T::AccountId>,
	<T as frame_system::Config>::AccountId: AsRef<[u8; 32]>,
	pallet_evm::AccountIdOf<T>: From<T::AccountId>,
	NonceIdOf<T>: Into<T::Nonce>,
{
	fn get_reserve_data(pool: EvmAddress, asset: EvmAddress) -> Result<ReserveData, ExecutorError<DispatchError>> {
		let context = CallContext::new_view(pool);
		let data = EvmDataWriter::new_with_selector(Function::GetReserveData)
			.write(asset)
			.build();

		let (res, reserve_data) = Executor::<T>::view(context, data, VIEW_GAS_LIMIT);

		ensure!(
			matches!(res, Succeed(_)),
			ExecutorError::Error("Failed to get reserve data".into())
		);

		let param_types = vec![
			ParamType::Uint(256), // configuration
			ParamType::Uint(256), // liquidityIndex
			ParamType::Uint(256), // variableBorrowIndex
			ParamType::Uint(256), // currentLiquidityRate
			ParamType::Uint(256), // currentVariableBorrowRate
			ParamType::Uint(256), // currentStableBorrowRate
			ParamType::Uint(256), // lastUpdateTimestamp
			ParamType::Uint(16),  // id
			ParamType::Address,   // aTokenAddress
			ParamType::Address,   // stableDebtTokenAddress
			ParamType::Address,   // variableDebtTokenAddress
			ParamType::Address,   // interestRateStrategyAddress
			ParamType::Uint(256), // accruedToTreasury
		];

		let decoded = decode(&param_types, reserve_data.as_ref())
			.map_err(|_| ExecutorError::Error("Failed to decode reserve data".into()))?;

		Ok(ReserveData {
			configuration: decoded[0].clone().into_uint().unwrap_or_default(),
			liquidity_index: decoded[1].clone().into_uint().unwrap_or_default(),
			current_liquidity_rate: decoded[3].clone().into_uint().unwrap_or_default(),
			variable_borrow_index: decoded[2].clone().into_uint().unwrap_or_default(),
			current_variable_borrow_rate: decoded[4].clone().into_uint().unwrap_or_default(),
			current_stable_borrow_rate: decoded[5].clone().into_uint().unwrap_or_default(),
			last_update_timestamp: decoded[6].clone().into_uint().unwrap_or_default(),
			id: decoded[7].clone().into_uint().unwrap_or_default().saturated_into(),
			atoken_address: EvmAddress::from_slice((&decoded[8].clone().into_address().unwrap_or_default()).as_ref()),
			stable_debt_token_address: EvmAddress::from_slice(
				(&decoded[9].clone().into_address().unwrap_or_default()).as_ref(),
			),
			variable_debt_token_address: EvmAddress::from_slice(
				(&decoded[10].clone().into_address().unwrap_or_default()).as_ref(),
			),
			interest_rate_strategy_address: EvmAddress::from_slice(
				(&decoded[11].clone().into_address().unwrap_or_default()).as_ref(),
			),
			accrued_to_treasury: decoded[12].clone().into_uint().unwrap_or_default(),
		})
	}

	fn get_available_liquidity(atoken: EvmAddress, underlying: EvmAddress) -> Balance {
		<Erc20Currency<T> as ERC20>::balance_of(CallContext::new_view(underlying), atoken)
	}

	fn get_scaled_total_supply(atoken: EvmAddress) -> Result<U256, ExecutorError<DispatchError>> {
		let context = CallContext::new_view(atoken);
		let data = EvmDataWriter::new_with_selector(Function::ScaledTotalSupply).build();
		let (res, value) = Executor::<T>::view(context, data, VIEW_GAS_LIMIT);
		ensure!(
			matches!(res, Succeed(_)),
			ExecutorError::Error("Failed to get scaled total supply".into())
		);
		U256::checked_from(value.as_slice()).ok_or(ExecutorError::Error("Failed to decode scaled total supply".into()))
	}

	fn get_asset_address(asset: AssetId) -> EvmAddress {
		pallet_asset_registry::Pallet::<T>::contract_address(asset)
			.unwrap_or_else(|| HydraErc20Mapping::encode_evm_address(asset))
	}

	fn get_underlying_asset(atoken: AssetId) -> Option<EvmAddress> {
		let Some(atoken_address) = pallet_asset_registry::Pallet::<T>::contract_address(atoken) else {
			// not a contract
			return None;
		};

		let context = CallContext::new_view(atoken_address);
		let data = Into::<u32>::into(Function::UnderlyingAssetAddress)
			.to_be_bytes()
			.to_vec();

		let (res, value) = Executor::<T>::view(context, data, VIEW_GAS_LIMIT);

		if !matches!(res, Succeed(_)) {
			// not a token
			return None;
		}

		Some(EvmAddress::from(H256::from_slice(&value)))
	}

	fn supply(origin: OriginFor<T>, asset: EvmAddress, amount: Balance) -> Result<(), DispatchError> {
		let who = ensure_signed(origin)?;
		let on_behalf_of = T::EvmAccounts::evm_address(&who);

		let context = CallContext::new_call(<BorrowingContract<T>>::get(), on_behalf_of);
		let data = EvmDataWriter::new_with_selector(Function::Supply)
			.write(asset)
			.write(amount)
			.write(on_behalf_of)
			.write(0_u16)
			.build();

		handle_result(Executor::<T>::call(context, data, U256::zero(), TRADE_GAS_LIMIT))
	}

	fn withdraw(origin: OriginFor<T>, asset: EvmAddress, amount: Balance) -> Result<(), DispatchError> {
		let who = ensure_signed(origin)?;
		let to = T::EvmAccounts::evm_address(&who);

		let context = CallContext::new_call(<BorrowingContract<T>>::get(), to);
		let data = EvmDataWriter::new_with_selector(Function::Withdraw)
			.write(asset)
			.write(amount)
			.write(to)
			.build();

		handle_result(Executor::<T>::call(context, data, U256::zero(), TRADE_GAS_LIMIT))
	}

	pub fn trade_weight() -> Weight {
		<T as pallet_evm::Config>::GasWeightMapping::gas_to_weight(TRADE_GAS_LIMIT + VIEW_GAS_LIMIT, true)
	}
}

fn handle_result(result: CallResult) -> DispatchResult {
	let (exit_reason, value) = result;
	match exit_reason {
		Succeed(_) => Ok(()),
		e => {
			let hex_value = hex::encode(&value);
			log::error!(target: "evm", "evm call failed with : {:?}, value: 0x{}, decoded: {:?}", e, hex_value, String::from_utf8_lossy(&value).into_owned());
			Err(DispatchError::Other(&*Box::leak(
				format!("evm:0x{}", hex_value).into_boxed_str(),
			)))
		}
	}
}

impl<T> TradeExecution<OriginFor<T>, AccountId, AssetId, Balance> for AaveTradeExecutor<T>
where
	T: pallet_evm::Config
		+ pallet_asset_registry::Config<AssetId = AssetId>
		+ pallet_liquidation::Config
		+ pallet_evm_accounts::Config
		+ frame_system::Config,
	T::AssetNativeLocation: Into<MultiLocation>,
	BalanceOf<T>: TryFrom<U256> + Into<U256>,
	T::AddressMapping: pallet_evm::AddressMapping<T::AccountId>,
	<T as frame_system::Config>::AccountId: AsRef<[u8; 32]>,
	pallet_evm::AccountIdOf<T>: From<T::AccountId>,
	NonceIdOf<T>: Into<T::Nonce>,
{
	type Error = DispatchError;

	fn calculate_sell(
		pool_type: PoolType<AssetId>,
		_asset_in: AssetId,
		_asset_out: AssetId,
		amount_in: Balance,
	) -> Result<Balance, ExecutorError<Self::Error>> {
		if pool_type != PoolType::Aave {
			return Err(ExecutorError::NotSupported);
		}

		// For both supply and withdraw, amount out is always 1:1
		// to save weight we just assume the operation will be available
		Ok(amount_in)
	}

	fn calculate_buy(
		pool_type: PoolType<AssetId>,
		asset_in: AssetId,
		asset_out: AssetId,
		amount_out: Balance,
	) -> Result<Balance, ExecutorError<Self::Error>> {
		Self::calculate_sell(pool_type, asset_in, asset_out, amount_out)
	}

	fn execute_sell(
		who: OriginFor<T>,
		pool_type: PoolType<AssetId>,
		asset_in: AssetId,
		asset_out: AssetId,
		amount_in: Balance,
		min_limit: Balance,
	) -> Result<(), ExecutorError<Self::Error>> {
		if pool_type != PoolType::Aave {
			return Err(ExecutorError::NotSupported);
		}

		ensure!(amount_in >= min_limit, ExecutorError::Error("Slippage exceeded".into()));

		if let Some(underlying) = AaveTradeExecutor::<T>::get_underlying_asset(asset_out) {
			// Supplying asset_in to get aToken (asset_out)
			let asset_address = AaveTradeExecutor::<T>::get_asset_address(asset_in);
			ensure!(
				asset_address == underlying,
				ExecutorError::Error("Asset mismatch: supplied asset must match aToken's underlying".into())
			);
			AaveTradeExecutor::<T>::supply(who, asset_address, amount_in).map_err(|e| ExecutorError::Error(e))?;
		} else if let Some(underlying) = AaveTradeExecutor::<T>::get_underlying_asset(asset_in) {
			// Withdrawing aToken (asset_in) to get underlying asset
			let asset_address = AaveTradeExecutor::<T>::get_asset_address(asset_out);
			ensure!(
				asset_address == underlying,
				ExecutorError::Error("Asset mismatch: output asset must match aToken's underlying".into())
			);
			AaveTradeExecutor::<T>::withdraw(who, underlying, amount_in).map_err(|e| ExecutorError::Error(e))?;
		} else {
			return Err(ExecutorError::Error("Invalid asset pair".into()));
		}

		Ok(())
	}

	fn execute_buy(
		who: OriginFor<T>,
		pool_type: PoolType<AssetId>,
		asset_in: AssetId,
		asset_out: AssetId,
		amount_out: Balance,
		max_limit: Balance,
	) -> Result<(), ExecutorError<Self::Error>> {
		Self::execute_sell(who, pool_type, asset_in, asset_out, amount_out, max_limit)
	}

	fn get_liquidity_depth(
		pool_type: PoolType<AssetId>,
		asset_in: AssetId,
		asset_out: AssetId,
	) -> Result<Balance, ExecutorError<Self::Error>> {
		if pool_type != PoolType::Aave {
			return Err(ExecutorError::NotSupported);
		}

		let pool = <BorrowingContract<T>>::get();

		if let Some(underlying) = AaveTradeExecutor::<T>::get_underlying_asset(asset_out) {
			let asset_address = pallet_asset_registry::Pallet::<T>::contract_address(asset_out).unwrap_or_default();
			Ok(AaveTradeExecutor::<T>::get_available_liquidity(
				asset_address,
				underlying,
			))
		} else {
			let asset_address = AaveTradeExecutor::<T>::get_asset_address(asset_out);
			let atoken_address = pallet_asset_registry::Pallet::<T>::contract_address(asset_in);
			let reserve_data = AaveTradeExecutor::<T>::get_reserve_data(pool, asset_address)?;
			ensure!(
				atoken_address == Some(reserve_data.atoken_address),
				ExecutorError::Error("Asset mismatch: first asset atoken has to match second asset reserve".into())
			);
			let scaled_token_supply = AaveTradeExecutor::<T>::get_scaled_total_supply(reserve_data.atoken_address)?;
			Ok(Balance::from(
				reserve_data
					.available_supply(scaled_token_supply)
					.saturated_into::<u128>(),
			))
		}
	}

	fn calculate_spot_price_with_fee(
		pool_type: PoolType<AssetId>,
		_asset_in: AssetId,
		_asset_out: AssetId,
	) -> Result<FixedU128, ExecutorError<Self::Error>> {
		if pool_type != PoolType::Aave {
			return Err(ExecutorError::NotSupported);
		}

		// Price is always 1:1
		Ok(FixedU128::from(1))
	}
}
