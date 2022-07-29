use solana_program_test::*;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::Transaction;
use solana_sdk::signer::Signer;
use solana_sdk::transport::TransportError;
use solana_sdk::signature::Keypair;
use solana_sdk::program_pack::Pack;
use solana_program::instruction::Instruction;
use solana_sdk::system_instruction;

pub fn coin98_dollar_mint_burn_program_test() -> ProgramTest {
    let mut program = ProgramTest::new("coin98_dollar_mint_burn", coin98_dollar_mint_burn::id(), None);
    program.add_program("chainlink_dfeed", chainlink_dfeed::id(), None);
    program
}

pub async fn airdrop(
    context: &mut ProgramTestContext,
    receiver: &Pubkey,
    amount: u64,
) -> Result<(), TransportError> {
    let tx = Transaction::new_signed_with_payer(
        &[system_instruction::transfer(
            &context.payer.pubkey(),
            receiver,
            amount,
        )],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await.unwrap();
    Ok(())
}

pub async fn process_transaction(context: &mut ProgramTestContext, instructions: &Vec<Instruction>, signers: &Vec<&Keypair>) -> Result<(), TransportError> {
    let tx = Transaction::new_signed_with_payer(
        instructions,
        Some(&signers[0].pubkey()),
        signers,
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(tx)
        .await
        .unwrap();

    Ok(())
}

pub async fn create_mint(
    context: &mut ProgramTestContext,
    mint: &Keypair,
    manager: &Pubkey,
    freeze_authority: Option<&Pubkey>,
) -> Result<(), TransportError> {
    let rent = context.banks_client.get_rent().await.unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &context.payer.pubkey(),
                &mint.pubkey(),
                rent.minimum_balance(spl_token::state::Mint::LEN),
                spl_token::state::Mint::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_mint(
                &spl_token::id(),
                &mint.pubkey(),
                &manager,
                freeze_authority,
                0,
            )
            .unwrap(),
        ],
        Some(&context.payer.pubkey()),
        &[&context.payer, &mint],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await
}


pub async fn create_associated_token_account(
    context: &mut ProgramTestContext,
    wallet: &Pubkey,
    token_mint: &Pubkey,
) -> Result<Pubkey, TransportError> {
    let recent_blockhash = context.last_blockhash;

    let tx = Transaction::new_signed_with_payer(
        &[
            spl_associated_token_account::create_associated_token_account(
                &context.payer.pubkey(),
                &wallet,
                token_mint,
            ),
        ],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        recent_blockhash,
    );

    // connection.send_and_confirm_transaction(&tx)?;
    context.banks_client.process_transaction(tx).await.unwrap();

    Ok(spl_associated_token_account::get_associated_token_address(
        &wallet,
        token_mint,
    ))
}

pub async fn mint_tokens(
    context: &mut ProgramTestContext,
    mint: &Pubkey,
    account: &Pubkey,
    amount: u64,
    owner: &Pubkey,
    additional_signer: Option<&Keypair>,
) -> Result<(), TransportError> {
    let mut signing_keypairs = vec![&context.payer];
    if let Some(signer) = additional_signer {
        signing_keypairs.push(signer);
    }

    let tx = Transaction::new_signed_with_payer(
        &[
            spl_token::instruction::mint_to(&spl_token::id(), mint, account, owner, &[], amount)
                .unwrap(),
        ],
        Some(&context.payer.pubkey()),
        &signing_keypairs,
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await
}

pub async fn transfer(
    context: &mut ProgramTestContext,
    mint: &Pubkey,
    from: &Keypair,
    to: &Pubkey,
) -> Result<(), TransportError> {
    let to_token_account = create_associated_token_account(context, to, mint).await?;

    let from_token_account =
        spl_associated_token_account::get_associated_token_address(&from.pubkey(), mint);

    let tx = Transaction::new_signed_with_payer(
        &[spl_token::instruction::transfer(
            &spl_token::id(),
            &from_token_account,
            &to_token_account,
            &from.pubkey(),
            &[&from.pubkey()],
            1,
        )
        .unwrap()],
        Some(&from.pubkey()),
        &[from],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await
}
