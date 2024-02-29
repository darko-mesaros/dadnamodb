use clap::Parser;

use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_types::region::Region;

use std::env;

// ======================================== ARGUMENT PARSING
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub table_name: String,
    #[arg(short,long, default_value_t = false)]
    pub initialize: bool,
    #[arg(short,long, default_value_t = false)]
    pub rm: bool,
    #[arg(short,long, default_value_t = false)]
    pub populate: bool,
    #[arg(short,long)]
    pub query: Option<String>,
}
// ======================================== END ARGUMENT PARSING

//======================================== AWS
pub async fn configure_aws(s: String) -> aws_config::SdkConfig {
    let provider = RegionProviderChain::first_try(env::var("AWS_DEFAULT_REGION").ok().map(Region::new))
        .or_default_provider()
        .or_else(Region::new(s));

    aws_config::defaults(BehaviorVersion::latest())
        .region(provider)
        .load()
        .await
}
//======================================== END AWS
