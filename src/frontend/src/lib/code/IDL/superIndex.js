export const superIndexIDL = ({ IDL }) => {
    const TxCount = IDL.Tuple(IDL.Nat32, IDL.Nat);
    const Overview = IDL.Record({
      'balance' : IDL.Nat,
      'sent' : TxCount,
      'last_active' : IDL.Nat64,
      'first_active' : IDL.Nat64,
      'received' : TxCount,
    });
    const LinkDataString = IDL.Record({
      'net' : IDL.Int,
      'linked_from' : IDL.Nat64,
      'linked_id' : IDL.Text,
      'number_txs' : IDL.Nat32,
      'gross' : IDL.Nat,
    });
    const ProcessedTX = IDL.Record({
      'hash' : IDL.Text,
      'to_account' : IDL.Text,
      'tx_value' : IDL.Nat,
      'from_account' : IDL.Text,
      'block' : IDL.Nat64,
      'tx_fee' : IDL.Opt(IDL.Nat),
      'tx_time' : IDL.Nat64,
      'tx_type' : IDL.Text,
      'spender' : IDL.Opt(IDL.Text),
    });
    const FullDataResponse = IDL.Record({
      'overview' : Overview,
      'links' : IDL.Vec(LinkDataString),
      'account_ref' : IDL.Text,
      'blocks' : IDL.Vec(ProcessedTX),
    });
    const LinkData = IDL.Record({
      'net' : IDL.Int,
      'linked_from' : IDL.Nat64,
      'linked_id' : IDL.Nat64,
      'number_txs' : IDL.Nat32,
      'gross' : IDL.Nat,
    });
    const FullDataResponseRaw = IDL.Record({
      'overview' : Overview,
      'links' : IDL.Vec(LinkData),
      'account_ref' : IDL.Nat64,
      'blocks' : IDL.Vec(IDL.Nat64),
    });
    const LogEntry = IDL.Record({ 'text' : IDL.Text, 'timestamp' : IDL.Text });
    const MemoryStats = IDL.Record({
      'memory' : IDL.Nat64,
      'heap_memory' : IDL.Nat64,
    });
    const TimeSearchArgs = IDL.Record({
      'id' : IDL.Text,
      'end' : IDL.Nat64,
      'start' : IDL.Nat64,
    });
    const MetricStats = IDL.Record({
      'total_errors' : IDL.Nat64,
      'total_api_requests' : IDL.Nat64,
    });
    const WorkingStats = IDL.Record({
      'metrics' : MetricStats,
      'next_block' : IDL.Nat64,
      'last_update_time' : IDL.Nat64,
      'ledger_tip_of_chain' : IDL.Nat64,
      'timer_active' : IDL.Bool,
      'is_upto_date' : IDL.Bool,
      'directory_count' : IDL.Nat64,
      'is_busy' : IDL.Bool,
    });
    const SetTargetArgs = IDL.Record({
      'target_ledger' : IDL.Text,
      'tx_store' : IDL.Text,
    });
    const IndexerType = IDL.Variant({
      'DfinityIcrc2' : IDL.Null,
      'DfinityIcrc3' : IDL.Null,
      'DfinityIcp' : IDL.Null,
    });
    return IDL.Service({
      'add_admin' : IDL.Func([IDL.Text], [IDL.Text], []),
      'add_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
      'deposit_cycles' : IDL.Func([], [], []),
      'get_all_admins' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
      'get_all_authorised' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
      'get_canister_version' : IDL.Func([], [IDL.Text], ['query']),
      'get_cycles_balance' : IDL.Func([], [IDL.Nat64], ['query']),
      'get_full_from_id' : IDL.Func([IDL.Text], [IDL.Opt(FullDataResponse)], []),
      'get_full_from_id_raw' : IDL.Func(
          [IDL.Text],
          [IDL.Opt(FullDataResponseRaw)],
          ['query'],
        ),
      'get_full_from_ref' : IDL.Func(
          [IDL.Nat64],
          [IDL.Opt(FullDataResponse)],
          [],
        ),
      'get_full_from_ref_raw' : IDL.Func(
          [IDL.Nat64],
          [IDL.Opt(FullDataResponseRaw)],
          ['query'],
        ),
      'get_latest_transactions' : IDL.Func(
          [IDL.Nat32],
          [IDL.Vec(ProcessedTX)],
          ['query'],
        ),
      'get_links_from_id' : IDL.Func(
          [IDL.Text],
          [IDL.Opt(IDL.Vec(LinkDataString))],
          ['query'],
        ),
      'get_links_from_id_raw' : IDL.Func(
          [IDL.Nat64],
          [IDL.Opt(IDL.Vec(LinkData))],
          ['query'],
        ),
      'get_links_from_ref' : IDL.Func(
          [IDL.Nat64],
          [IDL.Opt(IDL.Vec(LinkDataString))],
          ['query'],
        ),
      'get_links_from_ref_raw' : IDL.Func(
          [IDL.Nat64],
          [IDL.Opt(IDL.Vec(LinkData))],
          ['query'],
        ),
      'get_logs' : IDL.Func([], [IDL.Opt(IDL.Vec(LogEntry))], ['query']),
      'get_memory_stats' : IDL.Func([], [MemoryStats], ['query']),
      'get_multiple_tx' : IDL.Func(
          [IDL.Vec(IDL.Nat64)],
          [IDL.Vec(ProcessedTX)],
          [],
        ),
      'get_overview_by_id' : IDL.Func([IDL.Text], [IDL.Opt(Overview)], ['query']),
      'get_overview_by_ref' : IDL.Func(
          [IDL.Nat64],
          [IDL.Opt(Overview)],
          ['query'],
        ),
      'get_transactions_from_id' : IDL.Func(
          [IDL.Text],
          [IDL.Opt(IDL.Vec(ProcessedTX))],
          [],
        ),
      'get_transactions_from_id_raw' : IDL.Func(
          [IDL.Text],
          [IDL.Opt(IDL.Vec(IDL.Nat64))],
          ['query'],
        ),
      'get_transactions_from_ref' : IDL.Func(
          [IDL.Nat64],
          [IDL.Opt(IDL.Vec(ProcessedTX))],
          [],
        ),
      'get_transactions_from_ref_raw' : IDL.Func(
          [IDL.Nat64],
          [IDL.Opt(IDL.Vec(IDL.Nat64))],
          ['query'],
        ),
      'get_transactions_time_id' : IDL.Func(
          [TimeSearchArgs],
          [IDL.Opt(IDL.Vec(ProcessedTX))],
          [],
        ),
      'get_tx' : IDL.Func([IDL.Nat64], [IDL.Opt(ProcessedTX)], []),
      'get_working_stats' : IDL.Func([], [WorkingStats], ['query']),
      'init_target_ledger' : IDL.Func(
          [SetTargetArgs, IndexerType],
          [IDL.Text],
          [],
        ),
      'remove_admin' : IDL.Func([IDL.Text], [IDL.Text], []),
      'remove_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
      'start_processing_timer' : IDL.Func([IDL.Nat64], [IDL.Text], []),
      'stop_all_timers' : IDL.Func([], [IDL.Text], []),
    });
  };