use aws_sdk_iam::{Client as IamClient, Error, Region};
use aws_types::credentials::{EnvironmentProvider, ProvideCredentials};
use std::error::Error as StdError;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let region = Region::new("us-east-2");
    let credentials_provider = EnvironmentProvider::new();

    let config = aws_sdk_iam::Config::builder()
        .region(region.clone())
        .credentials_provider(credentials_provider)
        .build();

    let iam_client = IamClient::from_conf(config);

    let policy_arn = "arn:aws:iam::<AWS_ACCOUNT_ID>:policy/<POLICY_NAME>";
    let role_name = "<ROLE_NAME>";

    list_policy_versions(&iam_client, policy_arn).await?;
    get_policy(&iam_client, policy_arn).await?;
    list_attached_role_policies(&iam_client, role_name).await?;

    Ok(())
}

async fn list_policy_versions(
    client: &IamClient,
    policy_arn: &str,
) -> Result<(), Box<dyn StdError>> {
    let resp = client
        .list_policy_versions()
        .policy_arn(policy_arn)
        .send()
        .await?;

    println!("Policy Versions:");
    for version in resp.versions.unwrap_or_default() {
        let version_id = version.version_id.unwrap_or_default();
        println!(
            "Version ID: {}, Is Default Version: {}",
            version_id, version.is_default_version.unwrap_or(false),
        );
        get_policy_version(client, policy_arn, &version_id).await?;
    }

    Ok(())
}

async fn get_policy_version(
    client: &IamClient,
    policy_arn: &str,
    version_id: &str,
) -> Result<(), Box<dyn StdError>> {
    let resp = client
        .get_policy_version()
        .policy_arn(policy_arn)
        .version_id(version_id)
        .send()
        .await?;

    let policy_version = resp.policy_version.unwrap_or_default();
    println!(
        "Policy Version {} Document: {}",
        version_id,
        policy_version.document.unwrap_or_default()
    );

    Ok(())
}

async fn get_policy(
    client: &IamClient,
    policy_arn: &str,
) -> Result<(), Box<dyn StdError>> {
    let resp = client.get_policy().policy_arn(policy_arn).send().await?;

    let policy = resp.policy.unwrap_or_default();
    println!(
        "Policy ARN: {}, Policy Name: {}, Description: {}",
        policy_arn,
        policy.policy_name.unwrap_or_default(),
        policy.description.unwrap_or_default()
    );

    Ok(())
}

async fn list_attached_role_policies(
    client: &IamClient,
    role_name: &str,
) -> Result<(), Box<dyn StdError>> {
    let resp = client
        .list_attached_role_policies()
        .role_name(role_name)
        .send()
        .await?;

    println!("Policies attached to role {}:", role_name);
    for policy in resp.attached_policies.unwrap_or_default() {
        println!(
            "Policy ARN: {}, Policy Name: {}",
            policy.policy_arn.unwrap_or_default(),
            policy.policy_name.unwrap_or_default()
        );
    }

    Ok(())
}
