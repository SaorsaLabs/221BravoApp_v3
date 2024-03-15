export const priceAlertsIDL = ({ IDL }) => {
    const AlertData = IDL.Record({
      'id' : IDL.Nat64,
      'direction' : IDL.Nat8,
      'oc_id' : IDL.Text,
      'cross' : IDL.Text,
      'user' : IDL.Text,
      'price' : IDL.Float64,
    });
    const LogEntry = IDL.Record({ 'text' : IDL.Text, 'timestamp' : IDL.Text });
    const MemoryStats = IDL.Record({
      'memory' : IDL.Nat64,
      'heap_memory' : IDL.Nat64,
    });
    return IDL.Service({
      'add_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
      'add_price_alert' : IDL.Func([AlertData], [IDL.Int64], []),
      'deposit_cycles' : IDL.Func([], [], []),
      'get_all_authorised' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
      'get_all_cross_alerts' : IDL.Func(
          [IDL.Text],
          [IDL.Opt(IDL.Vec(AlertData))],
          ['query'],
        ),
      'get_all_user_alerts' : IDL.Func(
          [IDL.Text],
          [IDL.Opt(IDL.Vec(AlertData))],
          ['query'],
        ),
      'get_canister_version' : IDL.Func([], [IDL.Text], ['query']),
      'get_cycles_balance' : IDL.Func([], [IDL.Nat64], ['query']),
      'get_logs' : IDL.Func([], [IDL.Opt(IDL.Vec(LogEntry))], ['query']),
      'get_memory_stats' : IDL.Func([], [MemoryStats], ['query']),
      'remove_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
      'remove_price_alert' : IDL.Func([AlertData], [IDL.Text], []),
      'start_alert_timer' : IDL.Func([IDL.Nat64], [IDL.Text], []),
      'stop_all_timers' : IDL.Func([], [IDL.Text], []),
      'update_oracle_id' : IDL.Func([IDL.Text], [IDL.Text], []),
    });
  };