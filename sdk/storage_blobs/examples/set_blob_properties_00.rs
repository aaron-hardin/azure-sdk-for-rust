#[macro_use]
extern crate log;
use azure_core::{
    error::{ErrorKind, ResultExt},
    prelude::*,
};
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");
    let blob = std::env::args()
        .nth(2)
        .expect("please specify blob name as command line parameter");

    let http_client = azure_core::new_http_client();
    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);

    // this is how you would use the SAS token:
    // let storage_account_client = StorageAccountClient::new_sas_token(http_client.clone(), &account,
    //      "sv=2018-11-09&ss=b&srt=o&se=2021-01-15T12%3A09%3A01Z&sp=r&st=2021-01-15T11%3A09%3A01Z&spr=http,https&sig=some_signature")?;

    let storage_client = storage_account_client.as_storage_client();
    let blob_client = storage_client
        .as_container_client(&container)
        .as_blob_client(&blob);

    trace!("Requesting blob");

    // this is a single call that retrieves the first 1KB of the blob (or less if the blob is
    // smaller). The range(...) call is optional.
    let response = blob_client
        .get()
        .range(Range::new(0, 1024))
        .execute()
        .await?;

    println!("{:#?}", response);

    trace!("Requesting blob properties");

    let properties = blob_client
        .get_properties()
        .execute()
        .await?
        .blob
        .properties;

    blob_client
        .set_properties()
        .set_from_blob_properties(&properties)
        .content_md5(md5::compute("howdy"))
        .execute()
        .await?;

    Ok(())
}
