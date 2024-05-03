/*
 * Copyright 2023, WiltonDB Software
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::*;

fn to_hash_set(list: Vec<&str>) -> HashSet<String> {
    list.iter()
        .map(|s| s.to_string())
        .collect()
}

pub(super) const AUTOVACUUM: &str = "Autovacuum Only";
pub(super) fn autovacuum() -> HashSet<String> {
    to_hash_set(vec!(
        "autovacuum",
        "autovacuum_max_workers",
        "autovacuum_naptime",
        "autovacuum_vacuum_threshold",
        "autovacuum_vacuum_insert_threshold",
        "autovacuum_analyze_threshold",
        "autovacuum_vacuum_scale_factor",
        "autovacuum_vacuum_insert_scale_factor",
        "autovacuum_analyze_scale_factor",
        "autovacuum_freeze_max_age",
        "autovacuum_multixact_freeze_max_age",
        "autovacuum_vacuum_cost_delay",
        "autovacuum_vacuum_cost_limit",
    ))
}

pub(super) const ESCAPE_HATCHES: &str = "Escape Hatches";
pub(super) fn escape_hatches() -> HashSet<String> {
    to_hash_set(vec!(
        "babelfishpg_tsql.escape_hatch_checkpoint",
        "babelfishpg_tsql.escape_hatch_constraint_name_for_default",
        "babelfishpg_tsql.escape_hatch_database_misc_options",
        "babelfishpg_tsql.escape_hatch_for_replication",
        "babelfishpg_tsql.escape_hatch_fulltext",
        "babelfishpg_tsql.escape_hatch_ignore_dup_key",
        "babelfishpg_tsql.escape_hatch_index_clustering",
        "babelfishpg_tsql.escape_hatch_index_columnstore",
        "babelfishpg_tsql.escape_hatch_join_hints",
        "babelfishpg_tsql.escape_hatch_language_non_english",
        "babelfishpg_tsql.escape_hatch_login_hashed_password",
        "babelfishpg_tsql.escape_hatch_login_misc_options",
        "babelfishpg_tsql.escape_hatch_login_old_password",
        "babelfishpg_tsql.escape_hatch_login_password_must_change",
        "babelfishpg_tsql.escape_hatch_login_password_unlock",
        "babelfishpg_tsql.escape_hatch_nocheck_add_constraint",
        "babelfishpg_tsql.escape_hatch_nocheck_existing_constraint",
        "babelfishpg_tsql.escape_hatch_query_hints",
        "babelfishpg_tsql.escape_hatch_rowguidcol_column",
        "babelfishpg_tsql.escape_hatch_rowversion",
        "babelfishpg_tsql.escape_hatch_schemabinding_function",
        "babelfishpg_tsql.escape_hatch_schemabinding_procedure",
        "babelfishpg_tsql.escape_hatch_schemabinding_trigger",
        "babelfishpg_tsql.escape_hatch_schemabinding_view",
        "babelfishpg_tsql.escape_hatch_session_settings",
        "babelfishpg_tsql.escape_hatch_showplan_all",
        "babelfishpg_tsql.escape_hatch_storage_on_partition",
        "babelfishpg_tsql.escape_hatch_storage_options",
        "babelfishpg_tsql.escape_hatch_table_hints",
        "babelfishpg_tsql.escape_hatch_unique_constraint",
    ))
}

pub(super) const LOCALE: &str = "Locale Only";
pub(super) fn locale() -> HashSet<String> {
    to_hash_set(vec!(
        "datestyle",
        "intervalstyle",
        "timezone",
        "timezone_abbreviations",
        "extra_float_digits",
        "client_encoding",
        "lc_messages",
        "lc_monetary",
        "lc_numeric",
        "lc_time",
        "default_text_search_config",
    ))
}

pub(super) const LOGGING: &str = "Logging Only";
pub(super) fn logging() -> HashSet<String> {
    to_hash_set(vec!(
        "log_destination",
        "logging_collector",
        "log_directory",
        "log_filename",
        "log_file_mode",
        "log_rotation_age",
        "log_rotation_size",
        "log_truncate_on_rotation",
        "syslog_facility",
        "syslog_ident",
        "syslog_sequence_numbers",
        "syslog_split_messages",
        "event_source",
        "log_min_messages",
        "log_min_error_statement",
        "log_min_duration_statement",
        "log_min_duration_sample",
        "log_statement_sample_rate",
        "log_transaction_sample_rate",
        "log_startup_progress_interval",
        "debug_print_parse",
        "debug_print_rewritten",
        "debug_print_plan",
        "debug_pretty_print",
        "log_autovacuum_min_duration",
        "log_checkpoints",
        "log_connections",
        "log_disconnections",
        "log_duration",
        "log_error_verbosity",
        "log_hostname",
        "log_line_prefix",
        "log_lock_waits",
        "log_recovery_conflict_waits",
        "log_parameter_max_length",
        "log_parameter_max_length_on_error",
        "log_statement",
        "log_replication_commands",
        "log_temp_files",
        "log_timezone",
    ))
}

pub(super) const MEMORY: &str = "Memory Only";
pub(super) fn memory() -> HashSet<String> {
    to_hash_set(vec!(
        "shared_buffers",
        "huge_pages",
        "huge_page_size",
        "temp_buffers",
        "max_prepared_transactions",
        "work_mem",
        "hash_mem_multiplier",
        "maintenance_work_mem",
        "autovacuum_work_mem",
        "logical_decoding_work_mem",
        "max_stack_depth",
        "shared_memory_type",
        "dynamic_shared_memory_type",
        "min_dynamic_shared_memory",
    ))
}

pub(super) const MONITORING: &str = "Monitoring Only";
pub(super) fn monitoring() -> HashSet<String> {
    to_hash_set(vec!(
        "track_activities",
        "track_activity_query_size",
        "track_counts",
        "track_io_timing",
        "track_wal_io_timing",
        "track_functions",
        "stats_fetch_consistency",
        "compute_query_id",
        "log_statement_stats",
        "log_parser_stats",
        "log_planner_stats",
        "log_executor_stats",
    ))
}

pub(super) const NETWORKING: &str = "Networking Only";
pub(super) fn networking() -> HashSet<String> {
    to_hash_set(vec!(
        "listen_addresses",
        "port",
        "babelfishpg_tds.port",
        "tcp_keepalives_idle",
        "tcp_keepalives_interval",
        "tcp_keepalives_count",
        "tcp_user_timeout",
        "client_connection_check_interval",
    ))
}

pub(super) const PARALLEL: &str = "Parallel Only";
pub(super) fn parallel() -> HashSet<String> {
    to_hash_set(vec!(
        "backend_flush_after",
        "effective_io_concurrency",
        "maintenance_io_concurrency",
        "max_worker_processes",
        "max_parallel_workers_per_gather",
        "max_parallel_maintenance_workers",
        "max_parallel_workers",
        "parallel_leader_participation",
        "old_snapshot_threshold",
    ))
}

pub(super) const TDS: &str = "TDS Only";
pub(super) fn tds() -> HashSet<String> {
    to_hash_set(vec!(
        "babelfishpg_tds.default_server_name",
        "babelfishpg_tds.listen_addresses",
        "babelfishpg_tds.port",
        "babelfishpg_tds.product_version",
        "babelfishpg_tds.tds_debug_log_level",
        "babelfishpg_tds.tds_default_numeric_precision",
        "babelfishpg_tds.tds_default_numeric_scale",
        "babelfishpg_tds.tds_default_packet_size",
        "babelfishpg_tds.tds_default_protocol_version",
        "babelfishpg_tds.tds_ssl_encrypt",
        "babelfishpg_tds.tds_ssl_max_protocol_version",
        "babelfishpg_tds.tds_ssl_min_protocol_version",
        "babelfishpg_tds.unix_socket_directories",
        "babelfishpg_tds.unix_socket_group",
        "babelfishpg_tds.unix_socket_permissions",
    ))
}

pub(super) const SSL: &str = "SSL Only";
pub(super) fn ssl() -> HashSet<String> {
    to_hash_set(vec!(
        "ssl",
        "ssl_ca_file",
        "ssl_cert_file",
        "ssl_crl_file",
        "ssl_crl_dir",
        "ssl_key_file",
        "ssl_ciphers",
        "ssl_prefer_server_ciphers",
        "ssl_ecdh_curve",
        "ssl_min_protocol_version",
        "ssl_max_protocol_version",
        "ssl_dh_params_file",
        "ssl_passphrase_command",
        "ssl_passphrase_command_supports_reload",
    ))
}
