use adnl::common::KeyOption;
use ton_types::Result;
     
fn gen() -> Result<()> {
    let (private, public) = KeyOption::with_type_id(KeyOption::KEY_ED25519)?;
    let private = serde_json::to_value(private)?;
    println!("{:#}", serde_json::json!({
        "private": {
            "type_id": KeyOption::KEY_ED25519,
            "pvt_key": private["pvt_key"],
        },
        "public": {
            "type_id": KeyOption::KEY_ED25519,
            "pub_key": base64::encode(public.pub_key()?),
        },
        "keyhash": base64::encode(public.id().data()),
        "hex": {
            "secret": hex::encode(base64::decode(private["pvt_key"].as_str().unwrap())?),
            "public": hex::encode(public.pub_key()?)
        }
    }));
    Ok(())
} 

fn main() {
    gen().unwrap_or_else(|e| println!("Keypair generation error: {}", e))
}
