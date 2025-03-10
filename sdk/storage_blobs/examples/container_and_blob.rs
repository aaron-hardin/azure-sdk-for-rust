use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use bytes::Bytes;
use futures::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let http_client = azure_core::new_http_client();
    let storage_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &access_key)
            .storage_client();
    let container_client = storage_client.container_client(&container_name);

    // create container
    let res = container_client
        .create()
        .public_access(PublicAccess::None)
        .into_future()
        .await?;
    println!("{:?}", res);

    let data = Bytes::from_static(b"something");

    // this is not mandatory but it helps preventing
    // spurious data to be uploaded.
    let hash = md5::compute(&data[..]);

    let res = container_client
        .blob_client("blob0.txt")
        .put_block_blob(data.clone())
        .content_type("text/plain")
        .hash(hash)
        .into_future()
        .await?;
    println!("{:?}", res);

    let res = container_client
        .blob_client("blob1.txt")
        .put_block_blob(data.clone())
        .content_type("text/plain")
        .hash(hash)
        .into_future()
        .await?;
    println!("{:?}", res);

    let res = container_client
        .blob_client("blob2.txt")
        .put_block_blob(data)
        .content_type("text/plain")
        .hash(hash)
        .into_future()
        .await?;
    println!("{:?}", res);

    // only get the first set of blobs in the list
    let res = container_client
        .list_blobs()
        .include_metadata(true)
        .into_stream()
        .next()
        .await
        .expect("stream failed")?;
    println!("{:?}", res);

    Ok(())
}
