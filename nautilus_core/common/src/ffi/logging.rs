// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2024 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

use std::ffi::{c_char, CStr};

use nautilus_core::{
    ffi::string::{cstr_to_string, cstr_to_ustr, optional_cstr_to_string},
    uuid::UUID4,
};
use nautilus_model::identifiers::trader_id::TraderId;

use crate::{
    enums::{LogColor, LogLevel},
    logging::{self},
};

/// Initialize tracing.
///
/// Tracing is meant to be used to trace/debug async Rust code. It can be
/// configured to filter modules and write up to a specific level only using
/// by passing a configuration using the `RUST_LOG` environment variable.
///
/// # Safety
///
/// Should only be called once during an applications run, ideally at the
/// beginning of the run.
#[no_mangle]
pub extern "C" fn tracing_init() {
    logging::init_tracing();
}

/// Initialize logging.
///
/// Logging should be used for Python and sync Rust logic which is most of
/// the components in the main `nautilus_trader` package.
/// Logging can be configured to filter components and write up to a specific level only
/// by passing a configuration using the `NAUTILUS_LOG` environment variable.
///
/// # Safety
///
/// Should only be called once during an applications run, ideally at the
/// beginning of the run.
///
/// - Assume `config_spec_ptr` is a valid C string pointer.
/// - Assume `directory_ptr` is either NULL or a valid C string pointer.
/// - Assume `file_name_ptr` is either NULL or a valid C string pointer.
/// - Assume `file_format_ptr` is either NULL or a valid C string pointer.
#[no_mangle]
pub unsafe extern "C" fn logging_init(
    trader_id: TraderId,
    instance_id: UUID4,
    config_spec_ptr: *const c_char,
    directory_ptr: *const c_char,
    file_name_ptr: *const c_char,
    file_format_ptr: *const c_char,
) {
    let config_spec = cstr_to_string(config_spec_ptr);

    let directory = optional_cstr_to_string(directory_ptr);
    let file_name = optional_cstr_to_string(file_name_ptr);
    let file_format = optional_cstr_to_string(file_format_ptr);

    logging::init_logging(
        trader_id,
        instance_id,
        config_spec,
        directory,
        file_name,
        file_format,
    );
}

/// Create a new log event.
///
/// # Safety
///
/// - Assumes `component_ptr` is a valid C string pointer.
/// - Assumes `message_ptr` is a valid C string pointer.
#[no_mangle]
pub unsafe extern "C" fn logger_log(
    timestamp_ns: u64,
    level: LogLevel,
    color: LogColor,
    component_ptr: *const c_char,
    message_ptr: *const c_char,
) {
    let component = cstr_to_ustr(component_ptr);
    let message = CStr::from_ptr(message_ptr).to_string_lossy();

    logging::log(timestamp_ns, level, color, component, message);
}

/// Flush logger buffers.
#[no_mangle]
pub extern "C" fn logger_flush() {
    log::logger().flush()
}