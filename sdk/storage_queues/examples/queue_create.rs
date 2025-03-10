#[macro_use]
extern crate log;
use azure_core::prelude::*;

use azure_storage::core::prelude::*;
use azure_storage_queues::prelude::*;
use chrono::{Duration, Utc};

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let queue_name = std::env::args()
        .nth(1)
        .expect("Please pass the queue name as first parameter");

    let http_client = azure_core::new_http_client();

    let storage_account =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &access_key);

    let queue = storage_account.queue_client(queue_name);

    trace!("creating queue");

    // this step is optional but here we show
    // how to add metadata to a new queue.
    let mut metadata = Metadata::new();
    metadata
        .as_mut()
        .insert("source".into(), "Azure SDK for Rust".into());
    metadata
        .as_mut()
        .insert("created".into(), format!("{:?}", Utc::now()).into());

    let response = queue
        .create()
        .metadata(metadata.clone())
        .into_future()
        .await?;
    println!("response == {:#?}", response);

    // let's add some more metadata
    metadata.insert("version".to_owned(), "TBD".to_owned());
    metadata.insert("updated".to_owned(), format!("{:?}", Utc::now()));

    println!("metadata == {:#?}", metadata);

    let response = queue.set_metadata(metadata).into_future().await?;
    println!("response == {:#?}", response);

    // let's get back the metadata
    let response = queue.get_metadata().into_future().await?;
    println!("response == {:#?}", response);

    // use two queue stored access policies
    let policies = vec![
        QueueStoredAccessPolicy::new(
            "first_sap_read_process",
            Utc::now() - Duration::hours(1),
            Utc::now() + Duration::days(1),
        )
        .enable_read()
        .enable_process(),
        QueueStoredAccessPolicy::new(
            "sap_admin",
            Utc::now() - chrono::Duration::hours(1),
            Utc::now() + chrono::Duration::hours(5),
        )
        .enable_all(),
    ];

    let response = queue.set_acl(policies).into_future().await?;
    println!("response == {:#?}", response);

    // get the queue ACL
    let response = queue.get_acl().into_future().await?;
    println!("response == {:#?}", response);

    // now let's delete it
    let response = queue.delete().into_future().await?;
    println!("response == {:#?}", response);

    Ok(())
}
