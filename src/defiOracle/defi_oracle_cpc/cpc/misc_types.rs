use candid::CandidType;
use serde::{Serialize, Deserialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct XdrRates {
    pub icp_xdr: Option<u64>,
}

// Types for exchange rate canister (ICP/USD)
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct GetExchangeRateRequest {
    pub timestamp: Option<u64>,
    pub quote_asset: Asset,
    pub base_asset: Asset,
  }

  #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
  pub struct Asset { pub class: AssetClass, pub symbol: String }

  #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
  pub enum AssetClass { Cryptocurrency, FiatCurrency }

  #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
  pub enum GetExchangeRateResult { Ok(ExchangeRate), Err(ExchangeRateError) }
  
  #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
  pub struct ExchangeRateMetadata {
    pub decimals: u32,
    pub forex_timestamp: Option<u64>,
    pub quote_asset_num_received_rates: u64,
    pub base_asset_num_received_rates: u64,
    pub base_asset_num_queried_sources: u64,
    pub standard_deviation: u64,
    pub quote_asset_num_queried_sources: u64,
  }
  
  #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
  pub struct ExchangeRate {
    pub metadata: ExchangeRateMetadata,
    pub rate: u64,
    pub timestamp: u64,
    pub quote_asset: Asset,
    pub base_asset: Asset,
  }
  
  #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
  pub enum ExchangeRateError {
    AnonymousPrincipalNotAllowed,
    CryptoQuoteAssetNotFound,
    FailedToAcceptCycles,
    ForexBaseAssetNotFound,
    CryptoBaseAssetNotFound,
    StablecoinRateTooFewRates,
    ForexAssetsNotFound,
    InconsistentRatesReceived,
    RateLimited,
    StablecoinRateZeroRate,
    Other{ code: u32, description: String },
    ForexInvalidTimestamp,
    NotEnoughCycles,
    ForexQuoteAssetNotFound,
    StablecoinRateNotFound,
    Pending,
  }