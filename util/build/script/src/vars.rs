// Copyright (c) 2018-2020 MobileCoin Inc.

//! Environment variable names used by cargo.

// Environment variables Cargo reads
pub const ENV_CARGO_HOME: &str = "CARGO_HOME";
pub const ENV_CARGO_TARGET_DIR: &str = "CARGO_TARGET_DIR";
pub const ENV_RUSTC_WRAPPER: &str = "CARGO_RUSTC_WRAPPER";
pub const ENV_RUSTDOCFLAGS: &str = "RUSTDOCFLAGS";
pub const ENV_RUSTFLAGS: &str = "RUSTFLAGS";
pub const ENV_CARGO_INCREMENTAL: &str = "CARGO_INCREMENTAL";
pub const ENV_CARGO_CACHE_RUSTC_INFO: &str = "CARGO_CACHE_RUSTC_INFO";
pub const ENV_HTTPS_PROXY: &str = "HTTPS_PROXY";
pub const ENV_HTTP_TIMEOUT: &str = "HTTP_TIMEOUT";
pub const ENV_TERM: &str = "TERM";

// Configuration build variables
pub const ENV_CARGO_BUILD_JOBS: &str = "CARGO_BUILD_JOBS";
pub const ENV_CARGO_BUILD_TARGET: &str = "CARGO_BUILD_TARGET";
pub const ENV_CARGO_BUILD_DEP_INFO_BASEDIR: &str = "CARGO_BUILD_DEP_INFO_BASEDIR";
pub const ENV_CARGO_BUILD_PIPELINING: &str = "CARGO_BUILD_PIPELINING";
pub const ENV_CARGO_HTTP_DEBUG: &str = "CARGO_HTTP_DEBUG";
pub const ENV_CARGO_HTTP_CAINFO: &str = "CARGO_HTTP_CAINFO";
pub const ENV_CARGO_HTTP_CHECK_REVOKE: &str = "CARGO_HTTP_CHECK_REVOKE";
pub const ENV_CARGO_HTTP_SSL_VERSION: &str = "CARGO_HTTP_SSL_VERSION";
pub const ENV_CARGO_HTTP_LOW_SPEED_LIMIT: &str = "CARGO_HTTP_LOW_SPEED_LIMIT";
pub const ENV_CARGO_HTTP_MULTIPLEXING: &str = "CARGO_HTTP_MULTIPLEXING";
pub const ENV_CARGO_HTTP_USER_AGENT: &str = "CARGO_HTTP_USER_AGENT";
pub const ENV_CARGO_NET_RETRY: &str = "CARGO_NET_RETRY";
pub const ENV_CARGO_NET_GIT_FETCH_WITH_CLI: &str = "CARGO_NET_GIT_FETCH_WITH_CLI";
pub const ENV_CARGO_NET_OFFLINE: &str = "CARGO_NET_OFFLINE";
pub const ENV_CARGO_TERM_VERBOSE: &str = "CARGO_TERM_VERBOSE";
pub const ENV_CARGO_TERM_COLOR: &str = "CARGO_TERM_COLOR";

// Other build-script variables
pub const ENV_CARGO: &str = "CARGO";
pub const ENV_CARGO_LOCKED: &str = "CARGO_LOCKED";
pub const ENV_CARGO_MANIFEST_DIR: &str = "CARGO_MANIFEST_DIR";
pub const ENV_CARGO_MANIFEST_LINKS: &str = "CARGO_MANIFEST_LINKS";

// Other build vars
pub const ENV_DEBUG: &str = "DEBUG";
pub const ENV_HOST: &str = "HOST";
pub const ENV_NUM_JOBS: &str = "NUM_JOBS";
pub const ENV_OUT_DIR: &str = "OUT_DIR";
pub const ENV_OPT_LEVEL: &str = "OPT_LEVEL";
pub const ENV_PROFILE: &str = "PROFILE";
pub const ENV_RUSTC: &str = "RUSTC";
pub const ENV_RUSTC_LINKER: &str = "RUSTC_LINKER";
pub const ENV_RUSTDOC: &str = "RUSTDOC";
pub const ENV_TARGET: &str = "TARGET";
pub const ENV_LD: &str = "LD";

// On windows, this is dead code
#[allow(dead_code)]
pub const ENV_PATH: &str = "PATH";

// CARGO_PKG_*
pub const ENV_CARGO_PKG_VERSION: &str = "CARGO_PKG_VERSION";
pub const ENV_CARGO_PKG_VERSION_MAJOR: &str = "CARGO_PKG_VERSION_MAJOR";
pub const ENV_CARGO_PKG_VERSION_MINOR: &str = "CARGO_PKG_VERSION_MINOR";
pub const ENV_CARGO_PKG_VERSION_PATCH: &str = "CARGO_PKG_VERSION_PATCH";
pub const ENV_CARGO_PKG_VERSION_PRE: &str = "CARGO_PKG_VERSION_PRE";
pub const ENV_CARGO_PKG_AUTHORS: &str = "CARGO_PKG_AUTHORS";
pub const ENV_CARGO_PKG_NAME: &str = "CARGO_PKG_NAME";
pub const ENV_CARGO_PKG_DESCRIPTION: &str = "CARGO_PKG_DESCRIPTION";
pub const ENV_CARGO_PKG_HOMEPAGE: &str = "CARGO_PKG_HOMEPAGE";
pub const ENV_CARGO_PKG_REPOSITORY: &str = "CARGO_PKG_REPOSITORY";

// CARGO_CFG_*
pub const ENV_CARGO_CFG_DEBUG_ASSERTIONS: &str = "CARGO_CFG_DEBUG_ASSERTIONS";
pub const ENV_CARGO_CFG_PROC_MACRO: &str = "CARGO_CFG_PROC_MACRO";
pub const ENV_CARGO_CFG_TARGET_ARCH: &str = "CARGO_CFG_TARGET_ARCH";
pub const ENV_CARGO_CFG_TARGET_ENDIAN: &str = "CARGO_CFG_TARGET_ENDIAN";
pub const ENV_CARGO_CFG_TARGET_ENV: &str = "CARGO_CFG_TARGET_ENV";
pub const ENV_CARGO_CFG_TARGET_FAMILY: &str = "CARGO_CFG_TARGET_FAMILY";
pub const ENV_CARGO_CFG_TARGET_FEATURE: &str = "CARGO_CFG_TARGET_FEATURE";
pub const ENV_CARGO_CFG_TARGET_HAS_ATOMIC: &str = "CARGO_CFG_TARGET_HAS_ATOMIC";
pub const ENV_CARGO_CFG_TARGET_HAS_ATOMIC_LOAD_STORE: &str =
    "CARGO_CFG_TARGET_HAS_ATOMIC_LOAD_STORE";
pub const ENV_CARGO_CFG_TARGET_OS: &str = "CARGO_CFG_TARGET_OS";
pub const ENV_CARGO_CFG_TARGET_POINTER_WIDTH: &str = "CARGO_CFG_TARGET_POINTER_WIDTH";
pub const ENV_CARGO_CFG_TARGET_THREAD_LOCAL: &str = "CARGO_CFG_TARGET_THREAD_LOCAL";
pub const ENV_CARGO_CFG_TARGET_VENDOR: &str = "CARGO_CFG_TARGET_VENDOR";
