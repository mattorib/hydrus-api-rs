use super::super::common;
use hydrus_api::builders::import_builder::FileImport;
use hydrus_api::page::PageIdentifier;
use hydrus_api::service::ServiceName;
use hydrus_api::tag::Tag;
use hydrus_api::url::UrlType;

#[tokio::test]
async fn it_imports_file_paths() {
    let mut hydrus = common::get_hydrus();
    let result = hydrus
        .import()
        .file(FileImport::path("/does/not/exist/sadly"))
        .run()
        .await;

    assert!(result.is_err()) // file does not exist
}

#[tokio::test]
async fn it_imports_binary_files() {
    let mut hydrus = common::get_hydrus();
    let bytes = [0u8, 0u8, 0u8, 0u8];
    let result = hydrus
        .import()
        .file(FileImport::binary(&mut &bytes[..]))
        .run()
        .await;

    assert!(result.is_err()) // return status should be 4
}

#[tokio::test]
async fn it_imports_urls() {
    let mut hydrus = common::get_hydrus();

    let result = hydrus
        .import()
        .url("https://www.pixiv.net/member_illust.php?illust_id=83406361&mode=medium")
        .page(PageIdentifier::name("Rusty Import"))
        .show_page(true)
        .add_additional_tag(ServiceName::my_tags(), Tag::from("ark mage"))
        .add_additional_tag(ServiceName::my_tags(), Tag::from("character:megumin"))
        .run()
        .await
        .unwrap();

    assert!(result.normalised_url.len() > 0);
    assert_eq!(result.url_type, UrlType::Post)
}
