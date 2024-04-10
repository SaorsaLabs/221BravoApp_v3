export const priceOracleIDL = ({ IDL }) => {
  const Marketplace = IDL.Variant({
    'ICDEX' : IDL.Null,
    'ICPSWAP' : IDL.Null,
    'SONIC' : IDL.Null,
  });
  const MarketplaceDetails = IDL.Record({
    'active' : IDL.Bool,
    'marketplace' : Marketplace,
    'canister_id' : IDL.Text,
    'unit_size' : IDL.Nat64,
    'reverse_cross' : IDL.Bool,
  });
  const Token = IDL.Record({
    'decimals' : IDL.Nat32,
    'ticker' : IDL.Text,
    'ledger' : IDL.Text,
  });
  const AddSwapInput = IDL.Record({
    'init_quote' : IDL.Float64,
    'swap_id' : IDL.Text,
    'token0' : Token,
    'token1' : Token,
    'swap_type' : IDL.Nat8,
  });
  const CrossRecord = IDL.Tuple(IDL.Text, IDL.Bool);
  const MIBV1 = IDL.Record({
    'name' : IDL.Text,
    'canister' : IDL.Text,
    'assigned_marketplace' : Marketplace,
    'crosses' : IDL.Vec(CrossRecord),
  });
  const QuoteCurrency = IDL.Variant({
    'ICP' : IDL.Null,
    'USD' : IDL.Null,
    'XDR' : IDL.Null,
  });
  const ExchangeSnapshot = IDL.Record({
    'ask' : IDL.Nat,
    'bid' : IDL.Nat,
    'spread_pct' : IDL.Float64,
    'liquidity' : IDL.Tuple(IDL.Nat64, IDL.Nat64),
    'exchange' : Marketplace,
    'price' : IDL.Float64,
    'swap_pair' : IDL.Text,
    'snapshot_time' : IDL.Nat64,
  });
  const OverviewV1 = IDL.Record({
    'average_price' : IDL.Float64,
    'cross_to_usd' : IDL.Float64,
    'token_cross' : IDL.Text,
    'exchange_snapshots' : IDL.Vec(ExchangeSnapshot),
    'snapshot_time' : IDL.Nat64,
  });
  const InternalRateEntry = IDL.Record({
    'quote' : IDL.Float64,
    'swap_pair' : IDL.Text,
  });
  const InternalRates = IDL.Record({ 'data' : IDL.Vec(InternalRateEntry) });
  const LogEntry = IDL.Record({ 'text' : IDL.Text, 'timestamp' : IDL.Text });
  const MemoryStats = IDL.Record({
    'memory' : IDL.Nat64,
    'heap_memory' : IDL.Nat64,
  });
  const Metrics = IDL.Record({
    'total_snapshots_taken' : IDL.Nat64,
    'total_http_outcalls' : IDL.Nat64,
    'total_errors' : IDL.Nat64,
    'total_api_requests_v1' : IDL.Nat64,
  });
  const SwapPairDetails = IDL.Record({
    'active' : IDL.Bool,
    'marketplaces' : IDL.Vec(MarketplaceDetails),
    'swap_id' : IDL.Text,
    'token0' : Token,
    'token1' : Token,
    'swap_type' : IDL.Nat8,
  });
  return IDL.Service({
    'add_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
    'add_marketplace_to_swap' : IDL.Func(
        [IDL.Text, MarketplaceDetails],
        [IDL.Text],
        [],
      ),
    'add_mib_canister' : IDL.Func(
        [IDL.Text, IDL.Text, Marketplace],
        [IDL.Text],
        [],
      ),
    'add_pair_to_mib_canister' : IDL.Func([IDL.Text, IDL.Text], [IDL.Text], []),
    'add_swap_to_oracle' : IDL.Func([AddSwapInput], [IDL.Text], []),
    'canister_version' : IDL.Func([], [IDL.Text], ['query']),
    'deposit_cycles' : IDL.Func([], [], []),
    'get_all_authorised' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'get_all_mib_canisters' : IDL.Func([], [IDL.Vec(MIBV1)], ['query']),
    'get_all_quotes_v1' : IDL.Func([QuoteCurrency], [IDL.Vec(OverviewV1)], []),
    'get_all_swap_marketplaces' : IDL.Func(
        [IDL.Text],
        [IDL.Opt(IDL.Vec(MarketplaceDetails))],
        ['query'],
      ),
    'get_all_swap_pairs' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'get_cycles_balance' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_internal_rates' : IDL.Func([], [InternalRates], ['query']),
    'get_last_update_time' : IDL.Func([], [IDL.Nat64, IDL.Nat64], ['query']),
    'get_logs' : IDL.Func([], [IDL.Opt(IDL.Vec(LogEntry))], ['query']),
    'get_memory_stats' : IDL.Func([], [MemoryStats], ['query']),
    'get_metrics' : IDL.Func([], [Metrics], ['query']),
    'get_quote_v1' : IDL.Func(
        [IDL.Text, QuoteCurrency],
        [IDL.Opt(OverviewV1)],
        [],
      ),
    'get_swap_details' : IDL.Func(
        [IDL.Text],
        [IDL.Opt(SwapPairDetails)],
        ['query'],
      ),
    'remove_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
    'remove_marketplace_from_swap' : IDL.Func(
        [IDL.Text, Marketplace],
        [IDL.Text],
        [],
      ),
    'remove_mib_canister' : IDL.Func([IDL.Text], [IDL.Text], []),
    'remove_pair_from_mib_canister' : IDL.Func(
        [IDL.Text, IDL.Text],
        [IDL.Text],
        [],
      ),
    'remove_swap_from_oracle' : IDL.Func([IDL.Text], [IDL.Text], []),
    'set_mib_marketplace' : IDL.Func([IDL.Text, Marketplace], [IDL.Text], []),
    'start_quotes_timer' : IDL.Func([IDL.Nat64, IDL.Nat64], [IDL.Text], []),
    'stop_all_timers' : IDL.Func([], [IDL.Text], []),
    'update_internal_rate' : IDL.Func([IDL.Text, IDL.Float64], [], []),
    'update_pair_status' : IDL.Func(
        [IDL.Text, IDL.Opt(Marketplace), IDL.Bool],
        [IDL.Text],
        [],
      ),
  });
};