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
