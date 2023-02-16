#![allow(missing_docs)]
use std::{fmt, path::PathBuf, result};

use crate::Hash;
use derive_more::{From, TryInto};
use quic_rpc::{
    message::{Msg, RpcMsg, ServerStreaming},
    Service,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvideRequest {
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvideResponse {
    pub hash: Hash,
}

impl RpcMsg<SendmeService> for ProvideRequest {
    type Response = RpcResult<ProvideResponse>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListRequest;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse {
    pub path: PathBuf,
    pub hash: Hash,
    pub size: u64,
}

impl Msg<SendmeService> for ListRequest {
    type Pattern = ServerStreaming;
    type Update = Self;
    type Response = ListResponse;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WatchRequest;

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionRequest;

impl RpcMsg<SendmeService> for VersionRequest {
    type Response = VersionResponse;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WatchResponse {
    pub version: String,
}

impl Msg<SendmeService> for WatchRequest {
    type Response = WatchResponse;

    type Update = Self;

    type Pattern = ServerStreaming;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionResponse {
    pub version: String,
}

/// The RPC service for sendme.
#[derive(Debug, Clone)]
pub struct SendmeService;

/// Request enum
#[derive(Debug, Serialize, Deserialize, From, TryInto)]
pub enum SendmeRequest {
    Watch(WatchRequest),
    Version(VersionRequest),
    List(ListRequest),
    Provide(ProvideRequest),
}

/// Response enum
#[derive(Debug, Serialize, Deserialize, From, TryInto)]
pub enum SendmeResponse {
    Watch(WatchResponse),
    Version(VersionResponse),
    List(ListResponse),
    Provide(RpcResult<ProvideResponse>),
}

impl Service for SendmeService {
    type Req = SendmeRequest;
    type Res = SendmeResponse;
}

/// A serializable error type for use in RPC responses.
#[derive(Serialize, Deserialize, Debug)]
pub struct RpcError(serde_error::Error);

impl std::error::Error for RpcError {}

impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<anyhow::Error> for RpcError {
    fn from(e: anyhow::Error) -> Self {
        RpcError(serde_error::Error::new(&*e))
    }
}

pub type RpcResult<T> = result::Result<T, RpcError>;