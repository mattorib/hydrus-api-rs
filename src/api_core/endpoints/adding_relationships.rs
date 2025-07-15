use std::collections::HashMap;

use crate::api_core::{
    common::{FileSelection, FileServiceSelection},
    endpoints::Endpoint,
};

#[derive(Debug, Clone, Serialize)]
pub struct SetFileRelationshipsRequest {
    pub relationships: Vec<FileRelationship>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileRelationship {
    pub hash_a: String,
    pub hash_b: String,
    pub relationship: u8,
    pub do_default_content_merge: bool,
    pub delete_a: bool,
    pub delete_b: bool,
}

pub struct SetFileRelationships;

impl Endpoint for SetFileRelationships {
    type Request = SetFileRelationshipsRequest;
    type Response = ();

    fn path() -> String {
        String::from("manage_file_relationships/set_file_relationships")
    }
}

pub struct FileRelationshipBuilder {
    hash_a: String,
    hash_b: String,
    relationship: SetFileRelationshipType,
    do_default_content_merge: bool,
    delete_a: bool,
    delete_b: bool,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Serialize)]
pub enum SetFileRelationshipType {
    PotentialDuplicates = 0,
    FalsePositives = 1,
    SameQuality = 2,
    Alternatives = 3,
    ABetter = 4,
    BBetter = 7,
}

impl FileRelationshipBuilder {
    pub fn new(hash_a: String, hash_b: String, relationship: SetFileRelationshipType) -> Self {
        Self {
            hash_a,
            hash_b,
            relationship,
            do_default_content_merge: false,
            delete_a: false,
            delete_b: false,
        }
    }

    pub fn set_hash_a<S: AsRef<str>>(mut self, hash: S) -> Self {
        self.hash_a = hash.as_ref().into();

        self
    }

    pub fn set_hash_b<S: AsRef<str>>(mut self, hash: S) -> Self {
        self.hash_b = hash.as_ref().into();

        self
    }

    pub fn set_relationship(mut self, relationship: SetFileRelationshipType) -> Self {
        self.relationship = relationship;

        self
    }

    pub fn set_do_default_content_merge(mut self, do_default_content_merge: bool) -> Self {
        self.do_default_content_merge = do_default_content_merge;

        self
    }

    pub fn set_delete_a(mut self, delete_a: bool) -> Self {
        self.delete_a = delete_a;

        self
    }

    pub fn set_delete_b(mut self, delete_a: bool) -> Self {
        self.delete_b = delete_a;

        self
    }

    /// builds the request
    pub fn build(self) -> FileRelationship {
        FileRelationship {
            hash_a: self.hash_a,
            hash_b: self.hash_b,
            relationship: self.relationship as u8,
            do_default_content_merge: self.do_default_content_merge,
            delete_a: self.delete_a,
            delete_b: self.delete_b,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GetFileRelationshipsRequest {
    #[serde(flatten)]
    pub file_selection: FileSelection,
    #[serde(flatten)]
    pub service_selection: FileServiceSelection,
}

pub struct GetFileRelationships;

impl Endpoint for GetFileRelationships {
    type Request = GetFileRelationshipsRequest;
    type Response = GetFileRelationshipsResponse;

    fn path() -> String {
        String::from("manage_file_relationships/get_file_relationships")
    }
}

#[derive(Debug, Deserialize)]
pub struct GetFileRelationshipsResponse {
    pub file_relationships: HashMap<String, FileRelationships>,
}

#[derive(Debug, Deserialize)]
pub struct FileRelationships {
    pub is_king: bool,
    pub king: String,
    pub king_is_on_file_domain: bool,
    pub king_is_local: bool,
    #[serde(rename = "0")]
    pub potential_duplicates: Vec<String>,
    #[serde(rename = "1")]
    pub false_positives: Vec<String>,
    #[serde(rename = "3")]
    pub alternates: Vec<String>,
    #[serde(rename = "8")]
    pub duplicates: Vec<String>,
}
