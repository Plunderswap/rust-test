use web3::types::H256;

extern crate web3;

fn main() {
  let _ = main2();
}

#[tokio::main]
async fn main2() -> web3::Result<()> {

  let transport = web3::transports::Http::new("https://dev-api.zilliqa.com")?;
  let web3 = web3::Web3::new(transport);

  println!("Calling transaction_receipt.");

  let a: H256 = "0x0b8706d30bcc5b6e5efc47ccd4041b771b68f9b866cf9c2c4869f69059d15991".parse().unwrap();
  println!("a: {:?}",a);
  let transaction_receipt = web3.eth().transaction_receipt(a).await;

  println!("transaction_receipt: {:?}", transaction_receipt);

  Ok(())
}