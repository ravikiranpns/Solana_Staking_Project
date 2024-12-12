
use anchor_lang::prelude::*;
use staking_project::*;

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::ToAccountInfo;
    use solana_program_test::*;
    use solana_sdk::{
        signature::{Keypair, Signer},
        transport::TransportError,
    };

    #[tokio::test]
    async fn test_initialize() -> Result<(), TransportError> {
        let program_id = id();
        let mut program_test = ProgramTest::new(
            "staking_project",
            program_id,
            processor!(staking_project::entry),
        );

        let user = Keypair::new();

        let context = program_test.start_with_context().await;
        let user_account = context.banks_client.get_account(user.pubkey()).await?;
        assert!(user_account.is_some(), "User account is not initialized properly.");

        Ok(())
    }
}
