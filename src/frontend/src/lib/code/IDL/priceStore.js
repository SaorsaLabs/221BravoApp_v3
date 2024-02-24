export const priceStoreIDL = ({ IDL }) => {
    const PriceChangeData = IDL.Record({
      'sparkline' : IDL.Vec(IDL.Float64),
      'change_24' : IDL.Float64,
      'change_7d' : IDL.Float64,
      'latest_price' : IDL.Float64,
      'cross' : IDL.Text,
    });
    const PriceTuple = IDL.Record({
      'cross_price' : IDL.Float64,
      'usd_price' : IDL.Float64,
    });
    const OHLC = IDL.Record({
      'low' : PriceTuple,
      'close_time' : IDL.Nat64,
      'high' : PriceTuple,
      'close' : PriceTuple,
      'open' : PriceTuple,
      'volume' : IDL.Nat64,
      'open_time' : IDL.Nat64,
    });
    const LogEntry = IDL.Record({ 'text' : IDL.Text, 'timestamp' : IDL.Text });
    const MemoryStats = IDL.Record({
      'memory' : IDL.Nat64,
      'heap_memory' : IDL.Nat64,
    });
    return IDL.Service({
      'add_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
      'add_cross' : IDL.Func([IDL.Text], [], []),
      'canister_version' : IDL.Func([], [IDL.Text], ['query']),
      'deposit_cycles' : IDL.Func([], [], []),
      'get_all_authorised' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
      'get_all_change_data' : IDL.Func([], [IDL.Vec(PriceChangeData)], ['query']),
      'get_all_crosses' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
      'get_cross_count' : IDL.Func([], [IDL.Nat64], []),
      'get_cycles_balance' : IDL.Func([], [IDL.Nat64], ['query']),
      'get_d1_data' : IDL.Func(
          [IDL.Text, IDL.Nat64],
          [IDL.Opt(IDL.Vec(OHLC))],
          ['query'],
        ),
      'get_h1_data' : IDL.Func(
          [IDL.Text, IDL.Nat64],
          [IDL.Opt(IDL.Vec(OHLC))],
          ['query'],
        ),
      'get_logs' : IDL.Func([], [IDL.Opt(IDL.Vec(LogEntry))], ['query']),
      'get_m15_data' : IDL.Func(
          [IDL.Text, IDL.Nat64],
          [IDL.Opt(IDL.Vec(OHLC))],
          ['query'],
        ),
      'get_m5_data' : IDL.Func(
          [IDL.Text, IDL.Nat64],
          [IDL.Opt(IDL.Vec(OHLC))],
          ['query'],
        ),
      'get_memory_stats' : IDL.Func([], [MemoryStats], ['query']),
      'get_w1_data' : IDL.Func(
          [IDL.Text, IDL.Nat64],
          [IDL.Opt(IDL.Vec(OHLC))],
          ['query'],
        ),
      'remove_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
      'remove_cross' : IDL.Func([IDL.Text], [IDL.Text], []),
      'start_quotes_timer' : IDL.Func([IDL.Nat64, IDL.Text], [IDL.Text], []),
      'stop_all_timers' : IDL.Func([], [IDL.Text], []),
    });
  };