export const priceOracleIDL = ({ IDL }) => {
  const SwapPair = IDL.Variant({
    'SNEED_ICP' : IDL.Null,
    'GLDGOV_ICP' : IDL.Null,
    'CKBTC_ICP' : IDL.Null,
    'GHOST_ICP' : IDL.Null,
    'MOTOKO_ICP' : IDL.Null,
    'CHAT_ICP' : IDL.Null,
    'NUA_ICP' : IDL.Null,
    'EXE_ICP' : IDL.Null,
    'NTN_ICP' : IDL.Null,
    'CKETH_ICP' : IDL.Null,
    'CAT_ICP' : IDL.Null,
    'MOD_ICP' : IDL.Null,
    'QUERIO_ICP' : IDL.Null,
    'ICX_ICP' : IDL.Null,
    'TAL_ICP' : IDL.Null,
    'OGY_ICP' : IDL.Null,
    'SONIC_ICP' : IDL.Null,
    'BOOM_ICP' : IDL.Null,
    'KINIC_ICP' : IDL.Null,
    'SNS1_ICP' : IDL.Null,
    'HOT_ICP' : IDL.Null,
    'TRAX_ICP' : IDL.Null,
  });
  const MibRecord = IDL.Tuple(IDL.Text, IDL.Text);
  const QuoteCurrency = IDL.Variant({
    'ICP' : IDL.Null,
    'USD' : IDL.Null,
    'XDR' : IDL.Null,
  });
  const Marketplace = IDL.Variant({
    'ICDEX' : IDL.Null,
    'ICPSWAP' : IDL.Null,
    'SONIC' : IDL.Null,
  });
  const ExchangeSnapshot = IDL.Record({
    'ask' : IDL.Nat,
    'bid' : IDL.Nat,
    'spread_pct' : IDL.Float64,
    'liquidity' : IDL.Tuple(IDL.Nat64, IDL.Nat64),
    'exchange' : Marketplace,
    'price' : IDL.Float64,
    'swap_pair' : SwapPair,
    'snapshot_time' : IDL.Nat64,
  });
  const OverviewV1 = IDL.Record({
    'average_price' : IDL.Float64,
    'cross_to_usd' : IDL.Float64,
    'token_cross' : IDL.Text,
    'exchange_snapshots' : IDL.Vec(ExchangeSnapshot),
    'snapshot_time' : IDL.Nat64,
  });
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
  return IDL.Service({
    'add_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
    'add_mib_canister' : IDL.Func([IDL.Text, IDL.Text], [IDL.Text], []),
    'add_pair_to_mib_canister' : IDL.Func([IDL.Text, SwapPair], [IDL.Text], []),
    'canister_version' : IDL.Func([], [IDL.Text], ['query']),
    'deposit_cycles' : IDL.Func([], [], []),
    'get_all_authorised' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'get_all_mib_canisters' : IDL.Func([], [IDL.Vec(MibRecord)], ['query']),
    'get_all_quotes_v1' : IDL.Func([QuoteCurrency], [IDL.Vec(OverviewV1)], []),
    'get_all_swap_pairs' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'get_cycles_balance' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_last_update_time' : IDL.Func([], [IDL.Nat64, IDL.Nat64], ['query']),
    'get_logs' : IDL.Func([], [IDL.Opt(IDL.Vec(LogEntry))], ['query']),
    'get_memory_stats' : IDL.Func([], [MemoryStats], ['query']),
    'get_metrics' : IDL.Func([], [Metrics], ['query']),
    'get_quote_v1' : IDL.Func(
        [IDL.Text, QuoteCurrency],
        [IDL.Opt(OverviewV1)],
        [],
      ),
    'remove_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
    'remove_mib_canister' : IDL.Func([IDL.Text], [IDL.Text], []),
    'remove_pair_from_mib_canister' : IDL.Func(
        [IDL.Text, SwapPair],
        [IDL.Text],
        [],
      ),
    'set_mib_marketplace' : IDL.Func([IDL.Text, Marketplace], [IDL.Text], []),
    'start_quotes_timer' : IDL.Func([IDL.Nat64, IDL.Nat64], [IDL.Text], []),
    'stop_all_timers' : IDL.Func([], [IDL.Text], []),
  });
};