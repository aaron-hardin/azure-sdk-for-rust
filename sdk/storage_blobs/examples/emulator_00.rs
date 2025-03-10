use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();

    // this is how you use the emulator.
    let storage_account = StorageAccountClient::new_emulator_default().storage_client();
    let container_client = storage_account.container_client("emulcont");

    // create container
    let res = container_client
        .create()
        .public_access(PublicAccess::None)
        .into_future()
        .await?;
    println!("{:?}", res);

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
