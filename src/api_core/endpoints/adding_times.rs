use crate::api_core::endpoints::Endpoint;
use serde::Serialize;

#[derive(Clone, Debug, Copy, Serialize)]
pub enum DbTimeRequestType {
    FileImportedTime,
    FileDeletedTime,
    FileOriginallyImportedTime,
}

#[derive(Clone, Debug, Copy, Serialize)]
enum TimestampType {
    WebDomain = 0,
    FileModifiedTime = 1,
    FileImportedTime = 3,
    FileDeletedTime = 4,
    ArchivedTime = 5,
    LastViewed = 6,
    FileOriginallyImportedTime = 7,
}

impl From<DbTimeRequestType> for TimestampType {
    fn from(value: DbTimeRequestType) -> Self {
        match value {
            DbTimeRequestType::FileImportedTime => TimestampType::FileImportedTime,
            DbTimeRequestType::FileDeletedTime => TimestampType::FileDeletedTime,
            DbTimeRequestType::FileOriginallyImportedTime => {
                TimestampType::FileOriginallyImportedTime
            }
        }
    }
}

#[derive(Clone, Debug, Serialize)]
enum SetTimeRequestInner {
    WebDomain {
        hashes: Vec<String>,
        timestamp_ms: Option<String>,
        domain: String,
    },
    Disk {
        hashes: Vec<String>,
        timestamp_ms: Option<String>,
    },
    // file import time, file delete time, or file originally imported time
    Db {
        request_type: DbTimeRequestType,
        hashes: Vec<String>,
        timestamp_ms: Option<String>,
        file_service_key: String,
    },
    Archived {
        hashes: Vec<String>,
        timestamp_ms: Option<String>,
    },
    LastViewed {
        hashes: Vec<String>,
        timestamp_ms: Option<String>,
        canvas_type: u64,
    },
}

impl SetTimeRequestInner {
    fn add_hashes<I: IntoIterator<Item = String>>(&mut self, new_hashes: I) {
        match self {
            SetTimeRequestInner::WebDomain { hashes, .. }
            | SetTimeRequestInner::Disk { hashes, .. }
            | SetTimeRequestInner::Db { hashes, .. }
            | SetTimeRequestInner::Archived { hashes, .. }
            | SetTimeRequestInner::LastViewed { hashes, .. } => {
                hashes.extend(new_hashes);
            }
        }
    }

    fn set_timestamp(&mut self, new_timestamp: Option<String>) {
        match self {
            SetTimeRequestInner::WebDomain { timestamp_ms, .. }
            | SetTimeRequestInner::Disk { timestamp_ms, .. }
            | SetTimeRequestInner::Db { timestamp_ms, .. }
            | SetTimeRequestInner::Archived { timestamp_ms, .. }
            | SetTimeRequestInner::LastViewed { timestamp_ms, .. } => {
                *timestamp_ms = new_timestamp;
            }
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct SetTimeRequest {
    timestamp_type: TimestampType,
    #[serde(flatten)]
    request: SetTimeRequestInner,
}

impl From<SetTimeRequestInner> for SetTimeRequest {
    fn from(request: SetTimeRequestInner) -> Self {
        match request {
            SetTimeRequestInner::WebDomain { .. } => SetTimeRequest {
                timestamp_type: TimestampType::WebDomain,
                request,
            },
            SetTimeRequestInner::Disk { .. } => SetTimeRequest {
                timestamp_type: TimestampType::FileModifiedTime,
                request,
            },
            SetTimeRequestInner::Db { request_type, .. } => SetTimeRequest {
                timestamp_type: request_type.into(),
                request,
            },
            SetTimeRequestInner::Archived { .. } => SetTimeRequest {
                timestamp_type: TimestampType::ArchivedTime,
                request,
            },
            SetTimeRequestInner::LastViewed { .. } => SetTimeRequest {
                timestamp_type: TimestampType::LastViewed,
                request,
            },
        }
    }
}

pub struct SetTimeRequestBuilder {
    inner: SetTimeRequestInner,
}

impl SetTimeRequestBuilder {
    pub fn web_domain_time<S: ToString>(domain: S) -> Self {
        Self {
            inner: SetTimeRequestInner::WebDomain {
                hashes: Vec::new(),
                timestamp_ms: None,
                domain: domain.to_string(),
            },
        }
    }

    pub fn disk_time() -> Self {
        Self {
            inner: SetTimeRequestInner::Disk {
                hashes: Vec::new(),
                timestamp_ms: None,
            },
        }
    }

    pub fn db_time<S: ToString>(request_type: DbTimeRequestType, file_service_key: S) -> Self {
        Self {
            inner: SetTimeRequestInner::Db {
                request_type,
                hashes: Vec::new(),
                timestamp_ms: None,
                file_service_key: file_service_key.to_string(),
            },
        }
    }

    pub fn archived_time() -> Self {
        Self {
            inner: SetTimeRequestInner::Archived {
                hashes: Vec::new(),
                timestamp_ms: None,
            },
        }
    }

    pub fn last_viewed_time(canvas_type: u64) -> Self {
        Self {
            inner: SetTimeRequestInner::LastViewed {
                hashes: Vec::new(),
                timestamp_ms: None,
                canvas_type,
            },
        }
    }

    /// Adds a file hash to the request
    pub fn add_hash<S: AsRef<str>>(mut self, hash: S) -> Self {
        self.inner.add_hashes([hash.as_ref().into()]);

        self
    }

    /// Adds multiple file hashes to the request
    pub fn add_hashes(mut self, hashes: &[String]) -> Self {
        self.inner.add_hashes(hashes.into_iter().cloned());

        self
    }

    pub fn set_timestamp<S: ToString>(mut self, timestamp_ms: Option<S>) -> Self {
        self.inner
            .set_timestamp(timestamp_ms.map(|ts| ts.to_string()));

        self
    }

    pub fn build(self) -> SetTimeRequest {
        self.inner.into()
    }
}

pub struct SetTime;

impl Endpoint for SetTime {
    type Request = SetTimeRequest;
    type Response = ();

    fn path() -> String {
        String::from("edit_times/set_time")
    }
}
