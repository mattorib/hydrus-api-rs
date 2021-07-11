use super::super::common;
use hydrus_api::page::PageIdentifier;
use hydrus_api::service::ServiceName;
use hydrus_api::tag::Tag;
use hydrus_api::url::Url;

async fn get_url() -> Url {
    let mut hydrus = common::get_hydrus();
    hydrus
        .url("https://www.pixiv.net/member_illust.php?illust_id=83406361&mode=medium")
        .await
        .unwrap()
}

#[tokio::test]
async fn it_imports() {
    let mut url = get_url().await;

    url.import()
        .page(PageIdentifier::name("Rusty Import"))
        .add_additional_tag(ServiceName::my_tags(), Tag::from("character:megumin"))
        .run()
        .await
        .unwrap();
}
