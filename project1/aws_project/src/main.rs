#![allow(unused_imports)]
#![allow(dead_code)]
use aws_sdk_cloudwatch::{Client as CloudWatchClient, model::Dimension};
use aws_types::region::{Region};
use aws_config::meta::region::RegionProviderChain;

use aws_sdk_ec2::{Client as EC2Client, model::Filter};
use aws_sdk_sts::{Client as StsClient};

use std::error::Error;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let region_provider = RegionProviderChain::first_try(Region::new("us-east-2"))
        .or_default_provider()
        .or_else(Region::new("us-east-1"));
    // Your AWS credentials
    let shared_config = aws_config::from_env()
        .region(region_provider)
        .load()
        .await;


    // Set up CloudWatch, EC2, and STS clients
    //let cloudwatch = CloudWatchClient::new(&shared_config);
    let ec2 = EC2Client::new(&shared_config);
    //let sts = StsClient::new(&shared_config);



    let instances = ec2
        .describe_instances()
        //.filters(vec![instance_filter])
        .send()
        .await?;

    println!("Instances: {:?}", instances);

    // let instance_ids: Vec<String> = instances
    //     .reservations
    //     .unwrap_or_default()
    //     .into_iter()
    //     .flat_map(|reservation| reservation.instances.unwrap_or_default())
    //     .map(|instance| instance.instance_id.unwrap())
    //     .collect();
    //
    // for instance_id in instance_ids {
    //     println!("Fetching CPU metrics for instance: {}", instance_id);
    //
    //     let dimension = Dimension::builder()
    //         .name("InstanceId")
    //         .value(instance_id.clone())
    //         .build();
    //
    //
    // }

    Ok(())
}
