export const backendCanisterIDL = ({ IDL }) => {
  const PublicAddressEntry = IDL.Tuple(IDL.Text, IDL.Text);
  const LogEntry = IDL.Record({ 'text' : IDL.Text, 'timestamp' : IDL.Text });
  const MemoryStats = IDL.Record({
    'memory' : IDL.Nat64,
    'heap_memory' : IDL.Nat64,
  });
  const SentRecData = IDL.Tuple(IDL.Nat32, IDL.Nat);
  const Overview = IDL.Record({
    'balance' : IDL.Nat,
    'sent' : SentRecData,
    'last_active' : IDL.Nat64,
    'first_active' : IDL.Nat64,
    'received' : SentRecData,
  });
  const HolderBalanceResponse = IDL.Record({
    'data' : Overview,
    'holder' : IDL.Text,
  });
  const TopHolderData = IDL.Record({
    'cross' : IDL.Text,
    'stats221' : IDL.Text,
    'accounts' : IDL.Vec(HolderBalanceResponse),
    'principals' : IDL.Vec(HolderBalanceResponse),
  });
  const TokenData = IDL.Record({
    'sparkline' : IDL.Vec(IDL.Float64),
    'decimals' : IDL.Nat8,
    'mcap' : IDL.Float64,
    'cross' : IDL.Text,
    'change24' : IDL.Float64,
    'change7d' : IDL.Float64,
    'ledger' : IDL.Text,
    'supply' : IDL.Float64,
    'price' : IDL.Float64,
  });
  const UserData = IDL.Record({
    'user_name' : IDL.Text,
    'user_account' : IDL.Text,
    'user_tokens' : IDL.Nat32,
    'user_oc_principal' : IDL.Opt(IDL.Text),
  });
  return IDL.Service({
    'add_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
    'add_new_user' : IDL.Func([IDL.Text], [IDL.Text], []),
    'add_public_named_accounts' : IDL.Func(
        [IDL.Text, IDL.Text],
        [IDL.Text],
        [],
      ),
    'add_token_to_processing_list' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Nat8, IDL.Text],
        [IDL.Text],
        [],
      ),
    'add_user_named_accounts' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Text],
        [IDL.Text],
        [],
      ),
    'canister_version' : IDL.Func([], [IDL.Text], ['query']),
    'deposit_cycles' : IDL.Func([], [], []),
    'get_all_authorised' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'get_all_user_named_accounts' : IDL.Func(
        [IDL.Text],
        [IDL.Opt(IDL.Vec(PublicAddressEntry))],
        ['query'],
      ),
    'get_cycles_balance' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_logs' : IDL.Func([], [IDL.Opt(IDL.Vec(LogEntry))], ['query']),
    'get_memory_stats' : IDL.Func([], [MemoryStats], ['query']),
    'get_multiple_account' : IDL.Func(
        [IDL.Text, IDL.Nat32, IDL.Nat32],
        [IDL.Vec(IDL.Text)],
        ['query'],
      ),
    'get_public_named_accounts' : IDL.Func(
        [IDL.Vec(IDL.Text)],
        [IDL.Opt(IDL.Vec(PublicAddressEntry))],
        ['query'],
      ),
    'get_single_account' : IDL.Func(
        [IDL.Text, IDL.Nat32],
        [IDL.Text],
        ['query'],
      ),
    'get_top_holders' : IDL.Func(
        [IDL.Text],
        [IDL.Opt(TopHolderData)],
        ['query'],
      ),
    'get_top_tokens_data' : IDL.Func([], [IDL.Vec(TokenData)], ['query']),
    'get_user_data' : IDL.Func([IDL.Text], [IDL.Opt(UserData)], ['query']),
    'get_user_named_accounts' : IDL.Func(
        [IDL.Text, IDL.Vec(IDL.Text)],
        [IDL.Opt(IDL.Vec(PublicAddressEntry))],
        ['query'],
      ),
    'remove_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
    'remove_public_named_account' : IDL.Func([IDL.Text], [IDL.Text], []),
    'remove_token_from_processing_list' : IDL.Func([IDL.Text], [IDL.Text], []),
    'remove_user_named_account' : IDL.Func(
        [IDL.Text, IDL.Text],
        [IDL.Text],
        [],
      ),
    'set_user_oc_id' : IDL.Func([IDL.Text, IDL.Text], [IDL.Text], []),
    'start_quotes_timer' : IDL.Func(
        [IDL.Nat64, IDL.Nat64, IDL.Nat64],
        [IDL.Text],
        [],
      ),
    'stop_all_timers' : IDL.Func([], [IDL.Text], []),
    'update_user_tokens' : IDL.Func([IDL.Text, IDL.Nat32], [IDL.Text], []),
    'update_username' : IDL.Func([IDL.Text, IDL.Text], [IDL.Text], []),
    'encrypt' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'decrypt' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
    'add_user_tokens': IDL.Func([IDL.Text, IDL.Nat32], [IDL.Text], []),
  });
};