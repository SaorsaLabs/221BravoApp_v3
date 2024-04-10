export const trackingDataIDL = ({ IDL }) => {
    const Direction = IDL.Variant({
      'Both' : IDL.Null,
      'Inbound' : IDL.Null,
      'Outbound' : IDL.Null,
    });
    const ValueType = IDL.Variant({
      'GreaterThan' : IDL.Null,
      'LessThan' : IDL.Null,
      'Equals' : IDL.Null,
    });
    const TrackTarget = IDL.Record({
      'direction' : IDL.Opt(Direction),
      'value_type' : IDL.Opt(ValueType),
      'responder' : IDL.Opt(IDL.Text),
      'value' : IDL.Opt(IDL.Nat64),
      'name' : IDL.Opt(IDL.Text),
      'group' : IDL.Opt(IDL.Text),
      'account' : IDL.Text,
      'tx_type' : IDL.Opt(IDL.Text),
    });
    const LogEntry = IDL.Record({ 'text' : IDL.Text, 'timestamp' : IDL.Text });
    const MemoryStats = IDL.Record({
      'memory' : IDL.Nat64,
      'heap_memory' : IDL.Nat64,
    });
    const Metrics = IDL.Record({
      'total_errors' : IDL.Nat64,
      'total_api_requests' : IDL.Nat64,
    });
    const txCountTuple = IDL.Tuple(IDL.Nat32, IDL.Nat);
    const OverviewPlus = IDL.Record({
      'balance' : IDL.Nat,
      'name' : IDL.Text,
      'sent' : txCountTuple,
      'last_active' : IDL.Nat64,
      'first_active' : IDL.Nat64,
      'received' : txCountTuple,
    });
    const ExchangeCollection = IDL.Record({
      'exchange_snapshots' : IDL.Vec(OverviewPlus),
      'snapshot_time' : IDL.Nat64,
    });
    const Overview = IDL.Record({
      'balance' : IDL.Nat,
      'sent' : txCountTuple,
      'last_active' : IDL.Nat64,
      'first_active' : IDL.Nat64,
      'received' : txCountTuple,
    });
    const OutputType = IDL.Variant({
      'OverviewPlus' : OverviewPlus,
      'ExchangeCollection' : ExchangeCollection,
      'Overview' : Overview,
    });
    return IDL.Service({
      'add_admin' : IDL.Func([IDL.Text], [IDL.Text], []),
      'add_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
      'add_new_package' : IDL.Func([IDL.Text], [IDL.Text], []),
      'add_target_to_dex_package' : IDL.Func([TrackTarget], [IDL.Text], []),
      'add_target_to_exchange_package' : IDL.Func([TrackTarget], [IDL.Text], []),
      'add_target_to_package' : IDL.Func([IDL.Text, TrackTarget], [IDL.Text], []),
      'clear_outcome_data_from_package' : IDL.Func([IDL.Text], [IDL.Text], []),
      'deposit_cycles' : IDL.Func([], [], []),
      'get_all_admins' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
      'get_all_authorised' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
      'get_all_package_names' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
      'get_all_targets_from_package' : IDL.Func(
          [IDL.Text],
          [IDL.Vec(TrackTarget)],
          ['query'],
        ),
      'get_canister_version' : IDL.Func([], [IDL.Text], ['query']),
      'get_cycles_balance' : IDL.Func([], [IDL.Nat64], ['query']),
      'get_logs' : IDL.Func([], [IDL.Opt(IDL.Vec(LogEntry))], ['query']),
      'get_memory_stats' : IDL.Func([], [MemoryStats], ['query']),
      'get_metrics' : IDL.Func([], [Metrics], ['query']),
      'get_outcome_data_from_package' : IDL.Func(
          [IDL.Text],
          [IDL.Vec(OutputType)],
          ['query'],
        ),
      'init_dex_package' : IDL.Func([], [IDL.Text], []),
      'init_exchange_package' : IDL.Func([], [IDL.Text], []),
      'remove_admin' : IDL.Func([IDL.Text], [IDL.Text], []),
      'remove_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
      'remove_package' : IDL.Func([IDL.Text], [IDL.Text], []),
      'remove_target_from_dex_package' : IDL.Func([IDL.Text], [IDL.Text], []),
      'remove_target_from_exchange_package' : IDL.Func(
          [IDL.Text],
          [IDL.Text],
          [],
        ),
      'remove_target_from_package' : IDL.Func(
          [IDL.Text, IDL.Text],
          [IDL.Text],
          [],
        ),
      'start_timers' : IDL.Func([IDL.Nat64, IDL.Nat64], [IDL.Text], []),
      'stop_all_timers' : IDL.Func([], [IDL.Text], []),
    });
  };