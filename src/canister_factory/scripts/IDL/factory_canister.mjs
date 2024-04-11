export const factoryIDL = ({ IDL }) => {
    const LogEntry = IDL.Record({ 'text' : IDL.Text, 'timestamp' : IDL.Text });
    const MemoryStats = IDL.Record({
      'memory' : IDL.Nat64,
      'heap_memory' : IDL.Nat64,
    });
    return IDL.Service({
      'add_admin' : IDL.Func([IDL.Text], [IDL.Text], []),
      'add_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
      'add_wasm' : IDL.Func(
          [IDL.Vec(IDL.Nat8), IDL.Text, IDL.Opt(IDL.Text)],
          [IDL.Text],
          [],
        ),
      'add_wasm_chunk' : IDL.Func([IDL.Vec(IDL.Nat8), IDL.Text], [IDL.Text], []),
      'clear_wasm_vec' : IDL.Func([IDL.Text], [IDL.Text], []),
      'deposit_cycles' : IDL.Func([], [], []),
      'get_all_admins' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
      'get_all_authorised' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
      'get_canister_version' : IDL.Func([], [IDL.Text], ['query']),
      'get_cycles_balance' : IDL.Func([], [IDL.Nat64], ['query']),
      'get_logs' : IDL.Func([], [IDL.Opt(IDL.Vec(LogEntry))], ['query']),
      'get_memory_stats' : IDL.Func([], [MemoryStats], ['query']),
      'get_wasm_length' : IDL.Func([IDL.Text], [IDL.Nat64], ['query']),
      'remove_admin' : IDL.Func([IDL.Text], [IDL.Text], []),
      'remove_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
      'remove_wasm' : IDL.Func([IDL.Text], [IDL.Text], []),
    });
  };