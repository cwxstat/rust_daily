# Rust AWS SDK 1

See day29 for a complete example.

```rust,editable


use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sqs::{Client, Error, Region, PKG_VERSION};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// Which queue to use. If not provided, uses the first queue found.
    #[structopt(short, long)]
    queue: Option<String>,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

#[derive(Debug)]
struct SQSMessage {
    body: String,
    group: String,
}

async fn find_substr_in_queue(client: &Client, substr: &str) -> Result<String, Error> {
    let queues = client.list_queues().send().await?;
    let queue_urls = queues.queue_urls().unwrap_or_default();

    let found: String = queue_urls
        .iter()
        .filter(|s| s.contains(substr))
        .map(|s| s.to_string())
        .collect();

    if found.is_empty() {
        println!(
            "No queues found with '{}' in the name. Create a queue to proceed.",
            substr
        );
        std::process::exit(1);
    } else {
        Ok(found)
    }
}


async fn send(client: &Client, queue_url: &String, message: &SQSMessage) -> Result<(), Error> {
    println!("Sending message to queue with URL: {}", queue_url);

    let rsp = client
        .send_message()
        .queue_url(queue_url)
        .message_body(&message.body)
        .message_group_id(&message.group)
        // If the queue is FIFO, you need to set .message_deduplication_id
        // or configure the queue for ContentBasedDeduplication.
        .send()
        .await?;

    println!("Send message to the queue: {:#?}", rsp);

    Ok(())
}

// Pump a queue for up to 10 outstanding messages.
// snippet-start:[sqs.rust.sqs-receive]
async fn receive(client: &Client, queue_url: &String) -> Result<(), Error> {
    let rcv_message_output = client.receive_message().queue_url(queue_url).send().await?;

    println!("Messages from queue with url: {}", queue_url);

    for message in rcv_message_output.messages.unwrap_or_default() {
        println!("Got the message: {:#?}", message);
    }

    Ok(())
}
// snippet-end:[sqs.rust.sqs-receive]

/// Sends a message to and receives the message from a queue in the Region.
/// /// # Arguments
///
/// * `[-r REGION]` - The Region in which the client is created.
///    If not supplied, uses the value of the **AWS_REGION** environment variable.
///    If the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display additional information.
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let Opt {
        region,
        queue,
        verbose,
    } = Opt::from_args();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));

    println!();
    if verbose {
        println!("SQS client version: {}", PKG_VERSION);
        println!(
            "Region:             {}",
            region_provider.region().await.unwrap().as_ref()
        );
        println!();
    }

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);
    let last_queue_url = find_substr_in_queue(&client, "spud.fifo").await?;
    let queue_url = queue.unwrap_or(last_queue_url);

    let message = SQSMessage {
        body: "hello from my queue".to_owned(),
        group: "MyGroup".to_owned(),
    };

    send(&client, &queue_url, &message).await?;
    receive(&client, &queue_url).await?;

    Ok(())
}





```