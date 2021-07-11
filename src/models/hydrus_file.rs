use crate::endpoints::adding_tags::{AddTagsRequestBuilder, TagAction};
use crate::endpoints::common::{FileIdentifier, FileMetadataInfo};
use crate::error::Result;
use crate::service::ServiceName;
use crate::tag::Tag;
use crate::utils::tag_list_to_string_list;
use crate::Client;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum FileStatus {
    ReadyForImport,
    InDatabase,
    Deleted,
    Unknown,
}

impl Eq for FileStatus {}

#[derive(Clone)]
pub struct HydrusFile {
    pub(crate) client: Client,
    pub id: FileIdentifier,
    pub status: FileStatus,
    pub(crate) metadata: Option<FileMetadataInfo>,
}

impl HydrusFile {
    pub(crate) fn from_id(client: Client, id: u64) -> Self {
        Self {
            client,
            id: FileIdentifier::ID(id),
            status: FileStatus::Unknown,
            metadata: None,
        }
    }

    pub(crate) fn from_raw_status_and_hash<S: ToString>(
        client: Client,
        status: u8,
        hash: S,
    ) -> Self {
        let status = if status == 3 {
            FileStatus::Deleted
        } else if status == 0 {
            FileStatus::ReadyForImport
        } else {
            FileStatus::InDatabase
        };
        Self {
            client,
            id: FileIdentifier::Hash(hash.to_string()),
            status,
            metadata: None,
        }
    }

    pub(crate) fn from_metadata(client: Client, metadata: FileMetadataInfo) -> Self {
        let status = if metadata.is_trashed {
            FileStatus::Deleted
        } else {
            FileStatus::InDatabase
        };

        Self {
            client,
            id: FileIdentifier::Hash(metadata.hash.clone()),
            status,
            metadata: Some(metadata),
        }
    }

    /// Deletes the internally stored metadata about the file retrieves it again
    pub async fn update(&mut self) -> Result<()> {
        self.metadata = None;
        self.metadata().await?;
        Ok(())
    }

    /// Returns the metadata for the given file
    /// if there's already known metadata about the file it uses that
    async fn metadata(&mut self) -> Result<&FileMetadataInfo> {
        if self.metadata.is_none() {
            let metadata = self
                .client
                .get_file_metadata_by_identifier(self.id.clone())
                .await?;
            self.status = if metadata.is_trashed {
                FileStatus::Deleted
            } else {
                FileStatus::InDatabase
            };
            self.metadata = Some(metadata);
        }

        Ok(self.metadata.as_ref().unwrap())
    }

    /// Returns the hash of the file
    /// if the file identifier is an id it calls hydrus to resolve the file
    pub async fn hash(&mut self) -> Result<String> {
        match &self.id {
            FileIdentifier::ID(_) => {
                let metadata = self.metadata().await?;
                Ok(metadata.hash.clone())
            }
            FileIdentifier::Hash(hash) => Ok(hash.clone()),
        }
    }

    /// Associates the file with a list of urls
    pub async fn associate_urls(&mut self, urls: Vec<String>) -> Result<()> {
        let hash = self.hash().await?;
        self.client.associate_urls(urls, vec![hash]).await
    }

    /// Disassociates the file with a list of urls
    pub async fn disassociate_urls(&mut self, urls: Vec<String>) -> Result<()> {
        let hash = self.hash().await?;
        self.client.disassociate_urls(urls, vec![hash]).await
    }

    /// Returns map mapping lists of tags to services
    pub async fn services_with_tags(&mut self) -> Result<HashMap<ServiceName, Vec<Tag>>> {
        let metadata = self.metadata().await?;
        let mut tag_mappings = HashMap::new();

        for (service, status_tags) in &metadata.service_names_to_statuses_to_tags {
            let mut tag_list = Vec::new();

            for (_, tags) in status_tags {
                tag_list.append(&mut tags.into_iter().map(|t| t.into()).collect())
            }
            tag_mappings.insert(ServiceName(service.clone()), tag_list);
        }

        Ok(tag_mappings)
    }

    /// Returns a list of all tags assigned to the file
    pub async fn tags(&mut self) -> Result<Vec<Tag>> {
        let mut tag_list = Vec::new();
        let tag_mappings = self.services_with_tags().await?;

        for (_, mut tags) in tag_mappings {
            tag_list.append(&mut tags);
        }

        Ok(tag_list)
    }

    /// Adds tags for a specific service to the file
    pub async fn add_tags(&mut self, service: ServiceName, tags: Vec<Tag>) -> Result<()> {
        let hash = self.hash().await?;
        let request = AddTagsRequestBuilder::default()
            .add_hash(hash)
            .add_tags(service.0, tag_list_to_string_list(tags))
            .build();

        self.client.add_tags(request).await
    }

    /// Allows modification of tags by using the defined tag actions
    pub async fn modify_tags(
        &mut self,
        service: ServiceName,
        action: TagAction,
        tags: Vec<Tag>,
    ) -> Result<()> {
        let hash = self.hash().await?;
        let mut reqwest = AddTagsRequestBuilder::default().add_hash(hash);

        for tag in tags {
            reqwest =
                reqwest.add_tag_with_action(service.0.clone(), tag.to_string(), action.clone());
        }

        self.client.add_tags(reqwest.build()).await
    }
}
