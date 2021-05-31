#[tokio::main(flavor = "current_thread")]
async fn main() -> blez::Result<()> {
    let session = blez::Session::new().await?;
    let adapter_names = session.adapter_names().await?;
    for adapter_name in adapter_names {
        println!("Bluetooth adapter {}:", &adapter_name);
        let adapter = session.adapter(&adapter_name)?;
        println!("    Address:                    {}", adapter.address().await?);
        println!("    Address type:               {}", adapter.address_type().await?);
        println!("    Friendly name:              {}", adapter.alias().await?);
        println!("    Modalias:                   {:?}", adapter.modalias().await?);
        println!("    Powered:                    {:?}", adapter.is_powered().await?);
        println!("    Discoverabe:                {:?}", adapter.is_discoverable().await?);
        println!("    Pairable:                   {:?}", adapter.is_pairable().await?);
        println!("    UUIDs:                      {:?}", adapter.uuids().await?);
        println!();
        println!("    Active adv. instances:      {}", adapter.active_advertising_instances().await?);
        println!("    Supp.  adv. instances:      {}", adapter.supported_advertising_instances().await?);
        println!("    Supp.  adv. includes:       {:?}", adapter.supported_advertising_system_includes().await?);
        println!("    Adv. capabilites:           {:?}", adapter.supported_advertising_capabilities().await?);
        println!("    Adv. features:              {:?}", adapter.supported_advertising_features().await?);
        println!();
    }
    Ok(())
}
