//! Error types for the PeterParker scanner core
//!
//! This module provides a centralized error handling system with specific error types
//! for different components of the scanner. This allows for better error handling,
//! debugging, and user feedback.

use std::fmt;

/// Custom error type for the scanner engine
#[derive(Debug, Clone)]
pub enum ScannerError {
    /// Network-related errors
    Network(String),
    /// ARP table access errors
    Arp(String),
    /// HTTP request errors
    Http(String),
    /// Configuration errors
    Config(String),
    /// I/O errors
    Io(String),
    /// Timeout errors
    Timeout(String),
    /// Vendor lookup errors
    Vendor(String),
    /// Parse errors
    Parse(String),
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScannerError::Network(msg) => write!(f, "Network error: {}", msg),
            ScannerError::Arp(msg) => write!(f, "ARP error: {}", msg),
            ScannerError::Http(msg) => write!(f, "HTTP error: {}", msg),
            ScannerError::Config(msg) => write!(f, "Configuration error: {}", msg),
            ScannerError::Io(msg) => write!(f, "I/O error: {}", msg),
            ScannerError::Timeout(msg) => write!(f, "Timeout error: {}", msg),
            ScannerError::Vendor(msg) => write!(f, "Vendor lookup error: {}", msg),
            ScannerError::Parse(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for ScannerError {}

impl ScannerError {
    /// Create a new network error
    pub fn network<S: Into<String>>(msg: S) -> Self {
        Self::Network(msg.into())
    }

    /// Create a new ARP error
    pub fn arp<S: Into<String>>(msg: S) -> Self {
        Self::Arp(msg.into())
    }

    /// Create a new HTTP error
    pub fn http<S: Into<String>>(msg: S) -> Self {
        Self::Http(msg.into())
    }

    /// Create a new configuration error
    pub fn config<S: Into<String>>(msg: S) -> Self {
        Self::Config(msg.into())
    }

    /// Create a new I/O error
    pub fn io<S: Into<String>>(msg: S) -> Self {
        Self::Io(msg.into())
    }

    /// Create a new timeout error
    pub fn timeout<S: Into<String>>(msg: S) -> Self {
        Self::Timeout(msg.into())
    }

    /// Create a new vendor lookup error
    pub fn vendor<S: Into<String>>(msg: S) -> Self {
        Self::Vendor(msg.into())
    }

    /// Create a new parse error
    pub fn parse<S: Into<String>>(msg: S) -> Self {
        Self::Parse(msg.into())
    }
}

/// Result type alias for convenience
pub type ScannerResult<T> = Result<T, ScannerError>;

// Conversion from common error types
impl From<std::io::Error> for ScannerError {
    fn from(err: std::io::Error) -> Self {
        Self::io(err.to_string())
    }
}

impl From<tokio::time::error::Elapsed> for ScannerError {
    fn from(err: tokio::time::error::Elapsed) -> Self {
        Self::timeout(err.to_string())
    }
}

/// Create error-specific modules for different components
pub mod arp {
    use super::*;

    /// Errors specific to ARP operations
    #[derive(Debug, Clone)]
    pub enum ArpError {
        /// Failed to execute ARP command
        CommandFailed(String),
        /// Failed to parse ARP output
        ParseError(String),
        /// Invalid MAC address format
        InvalidMac(String),
        /// ARP table not accessible
        NotAccessible,
    }

    impl fmt::Display for ArpError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ArpError::CommandFailed(msg) => write!(f, "ARP command failed: {}", msg),
                ArpError::ParseError(msg) => write!(f, "Failed to parse ARP output: {}", msg),
                ArpError::InvalidMac(msg) => write!(f, "Invalid MAC address: {}", msg),
                ArpError::NotAccessible => write!(f, "ARP table not accessible"),
            }
        }
    }

    impl std::error::Error for ArpError {}

    impl From<ArpError> for ScannerError {
        fn from(err: ArpError) -> Self {
            Self::arp(err.to_string())
        }
    }

    pub type ArpResult<T> = Result<T, ArpError>;
}

pub mod http {
    use super::*;

    /// Errors specific to HTTP operations
    #[derive(Debug, Clone)]
    pub enum HttpError {
        /// Connection failed
        ConnectionFailed(String),
        /// Invalid HTTP response
        InvalidResponse(String),
        /// Request timeout
        RequestTimeout(String),
    }

    impl fmt::Display for HttpError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                HttpError::ConnectionFailed(msg) => write!(f, "HTTP connection failed: {}", msg),
                HttpError::InvalidResponse(msg) => write!(f, "Invalid HTTP response: {}", msg),
                HttpError::RequestTimeout(msg) => write!(f, "HTTP request timeout: {}", msg),
            }
        }
    }

    impl std::error::Error for HttpError {}

    impl From<HttpError> for ScannerError {
        fn from(err: HttpError) -> Self {
            Self::http(err.to_string())
        }
    }

    pub type HttpResult<T> = Result<T, HttpError>;
}