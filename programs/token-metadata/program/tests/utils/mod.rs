mod assert;
mod digital_asset;
mod edition_marker;
mod master_edition_v2;
mod metadata;
mod programmable;
mod rooster_manager;

use async_trait::async_trait;
pub use digital_asset::*;
pub use edition_marker::*;
pub use master_edition_v2::MasterEditionV2;
pub use metadata::{assert_collection_size, Metadata};
pub use programmable::create_default_metaplex_rule_set;
pub use rooster_manager::*;
use solana_program_test::*;
use solana_sdk::{
    account::{Account, AccountSharedData, ReadableAccount},
    program_pack::Pack,
    pubkey::Pubkey,
    signature::Signer,
    signer::keypair::Keypair,
    system_instruction,
    transaction::Transaction,
};
use spl_token::state::Mint;
pub use token_metadata::instruction;
use token_metadata::state::{
    CollectionDetails, MAX_EDITION_LEN, MAX_MASTER_EDITION_LEN, MAX_METADATA_LEN,
};

pub const DEFAULT_COLLECTION_DETAILS: Option<CollectionDetails> = {
    #[allow(deprecated)]
    Some(CollectionDetails::V1 { size: 0 })
};

pub const SOLANA_CREATE_FEE: u64 = 10_000_000;

pub fn program_test() -> ProgramTest {
    let mut program_test = ProgramTest::new("token_metadata", token_metadata::ID, None);
    program_test.add_program("spl_token_2022", spl_token_2022::ID, None);
    program_test
}

pub async fn get_account(context: &mut ProgramTestContext, pubkey: &Pubkey) -> Account {
    context
        .banks_client
        .get_account(*pubkey)
        .await
        .expect("account not found")
        .expect("account empty")
}

pub async fn get_mint(context: &mut ProgramTestContext, pubkey: &Pubkey) -> Mint {
    let account = get_account(context, pubkey).await;
    Mint::unpack(&account.data).unwrap()
}

pub async fn airdrop(
    context: &mut ProgramTestContext,
    receiver: &Pubkey,
    amount: u64,
) -> Result<(), BanksClientError> {
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

pub async fn burn(
    context: &mut ProgramTestContext,
    metadata: Pubkey,
    owner: &Keypair,
    mint: Pubkey,
    token: Pubkey,
    edition: Pubkey,
    collection_metadata: Option<Pubkey>,
) -> Result<(), BanksClientError> {
    let tx = Transaction::new_signed_with_payer(
        &[instruction::burn_nft(
            token_metadata::ID,
            metadata,
            owner.pubkey(),
            mint,
            token,
            edition,
            spl_token::ID,
            collection_metadata,
        )],
        Some(&owner.pubkey()),
        &[owner],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn burn_edition(
    context: &mut ProgramTestContext,
    metadata: Pubkey,
    owner: &Keypair,
    print_edition_mint: Pubkey,
    master_edition_mint: Pubkey,
    print_edition_token: Pubkey,
    master_edition_token: Pubkey,
    master_edition: Pubkey,
    print_edition: Pubkey,
    edition_marker: Pubkey,
) -> Result<(), BanksClientError> {
    let tx = Transaction::new_signed_with_payer(
        &[instruction::burn_edition_nft(
            token_metadata::ID,
            metadata,
            owner.pubkey(),
            print_edition_mint,
            master_edition_mint,
            print_edition_token,
            master_edition_token,
            master_edition,
            print_edition,
            edition_marker,
            spl_token::ID,
        )],
        Some(&owner.pubkey()),
        &[owner],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await?;

    Ok(())
}

pub async fn mint_tokens(
    context: &mut ProgramTestContext,
    mint: &Pubkey,
    account: &Pubkey,
    amount: u64,
    owner: &Pubkey,
    additional_signer: Option<&Keypair>,
    spl_token_program: &Pubkey,
) -> Result<(), BanksClientError> {
    let mut signing_keypairs = vec![&context.payer];
    if let Some(signer) = additional_signer {
        signing_keypairs.push(signer);
    }

    let tx = Transaction::new_signed_with_payer(
        &[spl_token_2022::instruction::mint_to(
            spl_token_program,
            mint,
            account,
            owner,
            &[],
            amount,
        )
        .unwrap()],
        Some(&context.payer.pubkey()),
        &signing_keypairs,
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await
}

pub async fn create_token_account(
    context: &mut ProgramTestContext,
    account: &Keypair,
    mint: &Pubkey,
    manager: &Pubkey,
    spl_token_program: &Pubkey,
) -> Result<(), BanksClientError> {
    let rent = context.banks_client.get_rent().await.unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &context.payer.pubkey(),
                &account.pubkey(),
                rent.minimum_balance(spl_token_2022::state::Account::LEN),
                spl_token_2022::state::Account::LEN as u64,
                spl_token_program,
            ),
            spl_token_2022::instruction::initialize_account(
                spl_token_program,
                &account.pubkey(),
                mint,
                manager,
            )
            .unwrap(),
        ],
        Some(&context.payer.pubkey()),
        &[&context.payer, account],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await
}

pub async fn create_mint(
    context: &mut ProgramTestContext,
    mint: &Keypair,
    manager: &Pubkey,
    freeze_authority: Option<&Pubkey>,
    decimals: u8,
    spl_token_program: &Pubkey,
) -> Result<(), BanksClientError> {
    let rent = context.banks_client.get_rent().await.unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[
            system_instruction::create_account(
                &context.payer.pubkey(),
                &mint.pubkey(),
                rent.minimum_balance(spl_token::state::Mint::LEN),
                spl_token::state::Mint::LEN as u64,
                spl_token_program,
            ),
            spl_token_2022::instruction::initialize_mint(
                spl_token_program,
                &mint.pubkey(),
                manager,
                freeze_authority,
                decimals,
            )
            .unwrap(),
        ],
        Some(&context.payer.pubkey()),
        &[&context.payer, mint],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await
}

pub trait DirtyClone {
    fn dirty_clone(&self) -> Self;
}

impl DirtyClone for Keypair {
    fn dirty_clone(&self) -> Self {
        Keypair::from_bytes(&self.to_bytes()).unwrap()
    }
}

pub async fn warp100(context: &mut ProgramTestContext) {
    let current_slot = context.banks_client.get_root_slot().await.unwrap();
    println!("Warping to slot: {}", current_slot + 100);
    context.warp_to_slot(current_slot + 100).unwrap();
}

#[async_trait]
pub trait Airdrop {
    async fn airdrop(
        &self,
        context: &mut ProgramTestContext,
        lamports: u64,
    ) -> Result<(), BanksClientError>;
}

#[async_trait]
impl Airdrop for Keypair {
    async fn airdrop(
        &self,
        context: &mut ProgramTestContext,
        lamports: u64,
    ) -> Result<(), BanksClientError> {
        let tx = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(
                &context.payer.pubkey(),
                &self.pubkey(),
                lamports,
            )],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await
    }
}

pub async fn upsize_metadata(context: &mut ProgramTestContext, address: &Pubkey) {
    let account = get_account(context, address).await;
    assert_eq!(
        account.lamports, 15115600,
        "Original lamports must be 15115600"
    );
    let mut account_shared = AccountSharedData::from(account);
    let original = account_shared.data();
    assert_eq!(
        original.len(),
        MAX_METADATA_LEN,
        "Original metadata size must be MAX_METADATA_LEN bytes"
    );

    let mut extended = vec![0u8; 679];
    extended[..(original.len() - 1)].copy_from_slice(&original[..(original.len() - 1)]);
    // Copy the fee flag from the original metadata.
    extended[678] = original[606];

    account_shared.set_data(extended);
    context.set_account(address, &account_shared);
    airdrop(context, address, 15616720 - 15115600)
        .await
        .unwrap();
    let account = get_account(context, address).await;
    assert_eq!(
        account.lamports, 15616720,
        "Extended lamports must be 15616720"
    );
    assert_eq!(
        account.data.len(),
        679,
        "Extended metadata size must be 679 bytes"
    );
}

pub async fn upsize_master_edition(context: &mut ProgramTestContext, address: &Pubkey) {
    let account = get_account(context, address).await;
    assert_eq!(
        account.lamports, 1030080,
        "Original lamports must be 1030080"
    );
    let mut account_shared = AccountSharedData::from(account);
    let original = account_shared.data();
    assert_eq!(
        original.len(),
        MAX_MASTER_EDITION_LEN,
        "Original metadata size must be MAX_MASTER_EDITION_LEN bytes"
    );

    let mut extended = vec![0u8; 282];
    extended[..(original.len() - 1)].copy_from_slice(&original[..(original.len() - 1)]);
    // Copy the fee flag from the original metadata.
    extended[281] = original[19];
    extended[280] = original[18];

    account_shared.set_data(extended);
    context.set_account(address, &account_shared);
    airdrop(context, address, 2853600 - 1030080).await.unwrap();
    let account = get_account(context, address).await;
    assert_eq!(
        account.lamports, 2853600,
        "Extended lamports must be 2853600"
    );
    assert_eq!(
        account.data.len(),
        282,
        "Extended metadata size must be 282 bytes"
    );
}

pub async fn upsize_edition(context: &mut ProgramTestContext, address: &Pubkey) {
    let account: Account = get_account(context, address).await;
    assert_eq!(
        account.lamports, 1183200,
        "Original lamports must be 1183200"
    );
    let mut account_shared = AccountSharedData::from(account);
    let original = account_shared.data();
    assert_eq!(
        original.len(),
        MAX_EDITION_LEN,
        "Original metadata size must be MAX_EDITION_LEN bytes"
    );

    let mut extended = vec![0u8; 241];
    extended[..(original.len() - 1)].copy_from_slice(&original[..(original.len() - 1)]);
    // Copy the fee flag from the original metadata.
    extended[240] = original[41];

    account_shared.set_data(extended);
    context.set_account(address, &account_shared);
    airdrop(context, address, 2568240 - 1183200).await.unwrap();
    let account = get_account(context, address).await;
    assert_eq!(
        account.lamports, 2568240,
        "Extended lamports must be 2568240"
    );
    assert_eq!(
        account.data.len(),
        241,
        "Extended metadata size must be 241 bytes"
    );
}
