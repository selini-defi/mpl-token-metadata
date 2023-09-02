//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct PrintV1 {
    /// New Metadata key (pda of ['metadata', program id, mint id])
    pub edition_metadata: solana_program::pubkey::Pubkey,
    /// New Edition (pda of ['metadata', program id, mint id, 'edition'])
    pub edition: solana_program::pubkey::Pubkey,
    /// Mint of new token - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub edition_mint: (solana_program::pubkey::Pubkey, bool),
    /// Owner of the token account of new token
    pub edition_token_account_owner: solana_program::pubkey::Pubkey,
    /// Token account of new token
    pub edition_token_account: solana_program::pubkey::Pubkey,
    /// Mint authority of new mint
    pub edition_mint_authority: solana_program::pubkey::Pubkey,
    /// Token record account
    pub edition_token_record: Option<solana_program::pubkey::Pubkey>,
    /// Master Record Edition V2 (pda of ['metadata', program id, master metadata mint id, 'edition'])
    pub master_edition: solana_program::pubkey::Pubkey,
    /// Edition pda to mark creation - will be checked for pre-existence. (pda of ['metadata', program id, master metadata mint id, 'edition', edition_number]) where edition_number is NOT the edition number you pass in args but actually edition_number = floor(edition/EDITION_MARKER_BIT_SIZE).
    pub edition_marker_pda: solana_program::pubkey::Pubkey,
    /// payer
    pub payer: solana_program::pubkey::Pubkey,
    /// owner of token account containing master token
    pub master_token_account_owner: solana_program::pubkey::Pubkey,
    /// token account containing token from master metadata mint
    pub master_token_account: solana_program::pubkey::Pubkey,
    /// Master record metadata account
    pub master_metadata: solana_program::pubkey::Pubkey,
    /// The update authority of the master edition.
    pub update_authority: solana_program::pubkey::Pubkey,
    /// Token program
    pub spl_token_program: solana_program::pubkey::Pubkey,
    /// SPL Associated Token Account program
    pub spl_ata_program: solana_program::pubkey::Pubkey,
    /// Instructions sysvar account
    pub sysvar_instructions: solana_program::pubkey::Pubkey,
    /// System program
    pub system_program: solana_program::pubkey::Pubkey,
}

impl PrintV1 {
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction(
        &self,
        args: PrintV1InstructionArgs,
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(18);
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.edition_metadata,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.edition,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.edition_mint.0,
            self.edition_mint.1,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.edition_token_account_owner,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.edition_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.edition_mint_authority,
            true,
        ));
        if let Some(edition_token_record) = self.edition_token_record {
            accounts.push(solana_program::instruction::AccountMeta::new(
                edition_token_record,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_TOKEN_METADATA_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.master_edition,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.edition_marker_pda,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.master_token_account_owner,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.master_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.master_metadata,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.update_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.spl_token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.spl_ata_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.sysvar_instructions,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        let mut data = PrintV1InstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::MPL_TOKEN_METADATA_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct PrintV1InstructionData {
    discriminator: u8,
    print_v1_discriminator: u8,
}

impl PrintV1InstructionData {
    fn new() -> Self {
        Self {
            discriminator: 55,
            print_v1_discriminator: 0,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrintV1InstructionArgs {
    pub edition_number: u64,
}

/// Instruction builder.
#[derive(Default)]
pub struct PrintV1Builder {
    edition_metadata: Option<solana_program::pubkey::Pubkey>,
    edition: Option<solana_program::pubkey::Pubkey>,
    edition_mint: Option<(solana_program::pubkey::Pubkey, bool)>,
    edition_token_account_owner: Option<solana_program::pubkey::Pubkey>,
    edition_token_account: Option<solana_program::pubkey::Pubkey>,
    edition_mint_authority: Option<solana_program::pubkey::Pubkey>,
    edition_token_record: Option<solana_program::pubkey::Pubkey>,
    master_edition: Option<solana_program::pubkey::Pubkey>,
    edition_marker_pda: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    master_token_account_owner: Option<solana_program::pubkey::Pubkey>,
    master_token_account: Option<solana_program::pubkey::Pubkey>,
    master_metadata: Option<solana_program::pubkey::Pubkey>,
    update_authority: Option<solana_program::pubkey::Pubkey>,
    spl_token_program: Option<solana_program::pubkey::Pubkey>,
    spl_ata_program: Option<solana_program::pubkey::Pubkey>,
    sysvar_instructions: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    edition_number: Option<u64>,
}

impl PrintV1Builder {
    pub fn new() -> Self {
        Self::default()
    }
    /// New Metadata key (pda of ['metadata', program id, mint id])
    #[inline(always)]
    pub fn edition_metadata(
        &mut self,
        edition_metadata: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.edition_metadata = Some(edition_metadata);
        self
    }
    /// New Edition (pda of ['metadata', program id, mint id, 'edition'])
    #[inline(always)]
    pub fn edition(&mut self, edition: solana_program::pubkey::Pubkey) -> &mut Self {
        self.edition = Some(edition);
        self
    }
    /// Mint of new token - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    #[inline(always)]
    pub fn edition_mint(
        &mut self,
        edition_mint: solana_program::pubkey::Pubkey,
        as_signer: bool,
    ) -> &mut Self {
        self.edition_mint = Some((edition_mint, as_signer));
        self
    }
    /// Owner of the token account of new token
    #[inline(always)]
    pub fn edition_token_account_owner(
        &mut self,
        edition_token_account_owner: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.edition_token_account_owner = Some(edition_token_account_owner);
        self
    }
    /// Token account of new token
    #[inline(always)]
    pub fn edition_token_account(
        &mut self,
        edition_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.edition_token_account = Some(edition_token_account);
        self
    }
    /// Mint authority of new mint
    #[inline(always)]
    pub fn edition_mint_authority(
        &mut self,
        edition_mint_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.edition_mint_authority = Some(edition_mint_authority);
        self
    }
    /// `[optional account]`
    /// Token record account
    #[inline(always)]
    pub fn edition_token_record(
        &mut self,
        edition_token_record: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.edition_token_record = Some(edition_token_record);
        self
    }
    /// Master Record Edition V2 (pda of ['metadata', program id, master metadata mint id, 'edition'])
    #[inline(always)]
    pub fn master_edition(&mut self, master_edition: solana_program::pubkey::Pubkey) -> &mut Self {
        self.master_edition = Some(master_edition);
        self
    }
    /// Edition pda to mark creation - will be checked for pre-existence. (pda of ['metadata', program id, master metadata mint id, 'edition', edition_number]) where edition_number is NOT the edition number you pass in args but actually edition_number = floor(edition/EDITION_MARKER_BIT_SIZE).
    #[inline(always)]
    pub fn edition_marker_pda(
        &mut self,
        edition_marker_pda: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.edition_marker_pda = Some(edition_marker_pda);
        self
    }
    /// payer
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// owner of token account containing master token
    #[inline(always)]
    pub fn master_token_account_owner(
        &mut self,
        master_token_account_owner: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.master_token_account_owner = Some(master_token_account_owner);
        self
    }
    /// token account containing token from master metadata mint
    #[inline(always)]
    pub fn master_token_account(
        &mut self,
        master_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.master_token_account = Some(master_token_account);
        self
    }
    /// Master record metadata account
    #[inline(always)]
    pub fn master_metadata(
        &mut self,
        master_metadata: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.master_metadata = Some(master_metadata);
        self
    }
    /// The update authority of the master edition.
    #[inline(always)]
    pub fn update_authority(
        &mut self,
        update_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.update_authority = Some(update_authority);
        self
    }
    /// Token program
    #[inline(always)]
    pub fn spl_token_program(
        &mut self,
        spl_token_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.spl_token_program = Some(spl_token_program);
        self
    }
    /// SPL Associated Token Account program
    #[inline(always)]
    pub fn spl_ata_program(
        &mut self,
        spl_ata_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.spl_ata_program = Some(spl_ata_program);
        self
    }
    /// Instructions sysvar account
    #[inline(always)]
    pub fn sysvar_instructions(
        &mut self,
        sysvar_instructions: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.sysvar_instructions = Some(sysvar_instructions);
        self
    }
    /// System program
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn edition_number(&mut self, edition_number: u64) -> &mut Self {
        self.edition_number = Some(edition_number);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn build(&self) -> solana_program::instruction::Instruction {
        let accounts = PrintV1 {
            edition_metadata: self.edition_metadata.expect("edition_metadata is not set"),
            edition: self.edition.expect("edition is not set"),
            edition_mint: self.edition_mint.expect("edition_mint is not set"),
            edition_token_account_owner: self
                .edition_token_account_owner
                .expect("edition_token_account_owner is not set"),
            edition_token_account: self
                .edition_token_account
                .expect("edition_token_account is not set"),
            edition_mint_authority: self
                .edition_mint_authority
                .expect("edition_mint_authority is not set"),
            edition_token_record: self.edition_token_record,
            master_edition: self.master_edition.expect("master_edition is not set"),
            edition_marker_pda: self
                .edition_marker_pda
                .expect("edition_marker_pda is not set"),
            payer: self.payer.expect("payer is not set"),
            master_token_account_owner: self
                .master_token_account_owner
                .expect("master_token_account_owner is not set"),
            master_token_account: self
                .master_token_account
                .expect("master_token_account is not set"),
            master_metadata: self.master_metadata.expect("master_metadata is not set"),
            update_authority: self.update_authority.expect("update_authority is not set"),
            spl_token_program: self.spl_token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            spl_ata_program: self.spl_ata_program.unwrap_or(solana_program::pubkey!(
                "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
            )),
            sysvar_instructions: self.sysvar_instructions.unwrap_or(solana_program::pubkey!(
                "Sysvar1nstructions1111111111111111111111111"
            )),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = PrintV1InstructionArgs {
            edition_number: self
                .edition_number
                .clone()
                .expect("edition_number is not set"),
        };

        accounts.instruction(args)
    }
}

/// `print_v1` CPI instruction.
pub struct PrintV1Cpi<'a> {
    /// The program to invoke.
    pub __program: &'a solana_program::account_info::AccountInfo<'a>,
    /// New Metadata key (pda of ['metadata', program id, mint id])
    pub edition_metadata: &'a solana_program::account_info::AccountInfo<'a>,
    /// New Edition (pda of ['metadata', program id, mint id, 'edition'])
    pub edition: &'a solana_program::account_info::AccountInfo<'a>,
    /// Mint of new token - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub edition_mint: (&'a solana_program::account_info::AccountInfo<'a>, bool),
    /// Owner of the token account of new token
    pub edition_token_account_owner: &'a solana_program::account_info::AccountInfo<'a>,
    /// Token account of new token
    pub edition_token_account: &'a solana_program::account_info::AccountInfo<'a>,
    /// Mint authority of new mint
    pub edition_mint_authority: &'a solana_program::account_info::AccountInfo<'a>,
    /// Token record account
    pub edition_token_record: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    /// Master Record Edition V2 (pda of ['metadata', program id, master metadata mint id, 'edition'])
    pub master_edition: &'a solana_program::account_info::AccountInfo<'a>,
    /// Edition pda to mark creation - will be checked for pre-existence. (pda of ['metadata', program id, master metadata mint id, 'edition', edition_number]) where edition_number is NOT the edition number you pass in args but actually edition_number = floor(edition/EDITION_MARKER_BIT_SIZE).
    pub edition_marker_pda: &'a solana_program::account_info::AccountInfo<'a>,
    /// payer
    pub payer: &'a solana_program::account_info::AccountInfo<'a>,
    /// owner of token account containing master token
    pub master_token_account_owner: &'a solana_program::account_info::AccountInfo<'a>,
    /// token account containing token from master metadata mint
    pub master_token_account: &'a solana_program::account_info::AccountInfo<'a>,
    /// Master record metadata account
    pub master_metadata: &'a solana_program::account_info::AccountInfo<'a>,
    /// The update authority of the master edition.
    pub update_authority: &'a solana_program::account_info::AccountInfo<'a>,
    /// Token program
    pub spl_token_program: &'a solana_program::account_info::AccountInfo<'a>,
    /// SPL Associated Token Account program
    pub spl_ata_program: &'a solana_program::account_info::AccountInfo<'a>,
    /// Instructions sysvar account
    pub sysvar_instructions: &'a solana_program::account_info::AccountInfo<'a>,
    /// System program
    pub system_program: &'a solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: PrintV1InstructionArgs,
}

impl<'a> PrintV1Cpi<'a> {
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(18);
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.edition_metadata.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.edition.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.edition_mint.0.key,
            self.edition_mint.1,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.edition_token_account_owner.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.edition_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.edition_mint_authority.key,
            true,
        ));
        if let Some(edition_token_record) = self.edition_token_record {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *edition_token_record.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_TOKEN_METADATA_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.master_edition.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.edition_marker_pda.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.master_token_account_owner.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.master_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.master_metadata.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.update_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.spl_token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.spl_ata_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.sysvar_instructions.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        let mut data = PrintV1InstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MPL_TOKEN_METADATA_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(18 + 1);
        account_infos.push(self.__program.clone());
        account_infos.push(self.edition_metadata.clone());
        account_infos.push(self.edition.clone());
        account_infos.push(self.edition_mint.0.clone());
        account_infos.push(self.edition_token_account_owner.clone());
        account_infos.push(self.edition_token_account.clone());
        account_infos.push(self.edition_mint_authority.clone());
        if let Some(edition_token_record) = self.edition_token_record {
            account_infos.push(edition_token_record.clone());
        }
        account_infos.push(self.master_edition.clone());
        account_infos.push(self.edition_marker_pda.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.master_token_account_owner.clone());
        account_infos.push(self.master_token_account.clone());
        account_infos.push(self.master_metadata.clone());
        account_infos.push(self.update_authority.clone());
        account_infos.push(self.spl_token_program.clone());
        account_infos.push(self.spl_ata_program.clone());
        account_infos.push(self.sysvar_instructions.clone());
        account_infos.push(self.system_program.clone());

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// `print_v1` CPI instruction builder.
pub struct PrintV1CpiBuilder<'a> {
    instruction: Box<PrintV1CpiBuilderInstruction<'a>>,
}

impl<'a> PrintV1CpiBuilder<'a> {
    pub fn new(program: &'a solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(PrintV1CpiBuilderInstruction {
            __program: program,
            edition_metadata: None,
            edition: None,
            edition_mint: None,
            edition_token_account_owner: None,
            edition_token_account: None,
            edition_mint_authority: None,
            edition_token_record: None,
            master_edition: None,
            edition_marker_pda: None,
            payer: None,
            master_token_account_owner: None,
            master_token_account: None,
            master_metadata: None,
            update_authority: None,
            spl_token_program: None,
            spl_ata_program: None,
            sysvar_instructions: None,
            system_program: None,
            edition_number: None,
        });
        Self { instruction }
    }
    /// New Metadata key (pda of ['metadata', program id, mint id])
    #[inline(always)]
    pub fn edition_metadata(
        &mut self,
        edition_metadata: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.edition_metadata = Some(edition_metadata);
        self
    }
    /// New Edition (pda of ['metadata', program id, mint id, 'edition'])
    #[inline(always)]
    pub fn edition(
        &mut self,
        edition: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.edition = Some(edition);
        self
    }
    /// Mint of new token - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    #[inline(always)]
    pub fn edition_mint(
        &mut self,
        edition_mint: &'a solana_program::account_info::AccountInfo<'a>,
        as_signer: bool,
    ) -> &mut Self {
        self.instruction.edition_mint = Some((edition_mint, as_signer));
        self
    }
    /// Owner of the token account of new token
    #[inline(always)]
    pub fn edition_token_account_owner(
        &mut self,
        edition_token_account_owner: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.edition_token_account_owner = Some(edition_token_account_owner);
        self
    }
    /// Token account of new token
    #[inline(always)]
    pub fn edition_token_account(
        &mut self,
        edition_token_account: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.edition_token_account = Some(edition_token_account);
        self
    }
    /// Mint authority of new mint
    #[inline(always)]
    pub fn edition_mint_authority(
        &mut self,
        edition_mint_authority: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.edition_mint_authority = Some(edition_mint_authority);
        self
    }
    /// `[optional account]`
    /// Token record account
    #[inline(always)]
    pub fn edition_token_record(
        &mut self,
        edition_token_record: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.edition_token_record = Some(edition_token_record);
        self
    }
    /// Master Record Edition V2 (pda of ['metadata', program id, master metadata mint id, 'edition'])
    #[inline(always)]
    pub fn master_edition(
        &mut self,
        master_edition: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.master_edition = Some(master_edition);
        self
    }
    /// Edition pda to mark creation - will be checked for pre-existence. (pda of ['metadata', program id, master metadata mint id, 'edition', edition_number]) where edition_number is NOT the edition number you pass in args but actually edition_number = floor(edition/EDITION_MARKER_BIT_SIZE).
    #[inline(always)]
    pub fn edition_marker_pda(
        &mut self,
        edition_marker_pda: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.edition_marker_pda = Some(edition_marker_pda);
        self
    }
    /// payer
    #[inline(always)]
    pub fn payer(&mut self, payer: &'a solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// owner of token account containing master token
    #[inline(always)]
    pub fn master_token_account_owner(
        &mut self,
        master_token_account_owner: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.master_token_account_owner = Some(master_token_account_owner);
        self
    }
    /// token account containing token from master metadata mint
    #[inline(always)]
    pub fn master_token_account(
        &mut self,
        master_token_account: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.master_token_account = Some(master_token_account);
        self
    }
    /// Master record metadata account
    #[inline(always)]
    pub fn master_metadata(
        &mut self,
        master_metadata: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.master_metadata = Some(master_metadata);
        self
    }
    /// The update authority of the master edition.
    #[inline(always)]
    pub fn update_authority(
        &mut self,
        update_authority: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.update_authority = Some(update_authority);
        self
    }
    /// Token program
    #[inline(always)]
    pub fn spl_token_program(
        &mut self,
        spl_token_program: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.spl_token_program = Some(spl_token_program);
        self
    }
    /// SPL Associated Token Account program
    #[inline(always)]
    pub fn spl_ata_program(
        &mut self,
        spl_ata_program: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.spl_ata_program = Some(spl_ata_program);
        self
    }
    /// Instructions sysvar account
    #[inline(always)]
    pub fn sysvar_instructions(
        &mut self,
        sysvar_instructions: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.sysvar_instructions = Some(sysvar_instructions);
        self
    }
    /// System program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn edition_number(&mut self, edition_number: u64) -> &mut Self {
        self.instruction.edition_number = Some(edition_number);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn build(&self) -> PrintV1Cpi<'a> {
        let args = PrintV1InstructionArgs {
            edition_number: self
                .instruction
                .edition_number
                .clone()
                .expect("edition_number is not set"),
        };

        PrintV1Cpi {
            __program: self.instruction.__program,

            edition_metadata: self
                .instruction
                .edition_metadata
                .expect("edition_metadata is not set"),

            edition: self.instruction.edition.expect("edition is not set"),

            edition_mint: self
                .instruction
                .edition_mint
                .expect("edition_mint is not set"),

            edition_token_account_owner: self
                .instruction
                .edition_token_account_owner
                .expect("edition_token_account_owner is not set"),

            edition_token_account: self
                .instruction
                .edition_token_account
                .expect("edition_token_account is not set"),

            edition_mint_authority: self
                .instruction
                .edition_mint_authority
                .expect("edition_mint_authority is not set"),

            edition_token_record: self.instruction.edition_token_record,

            master_edition: self
                .instruction
                .master_edition
                .expect("master_edition is not set"),

            edition_marker_pda: self
                .instruction
                .edition_marker_pda
                .expect("edition_marker_pda is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            master_token_account_owner: self
                .instruction
                .master_token_account_owner
                .expect("master_token_account_owner is not set"),

            master_token_account: self
                .instruction
                .master_token_account
                .expect("master_token_account is not set"),

            master_metadata: self
                .instruction
                .master_metadata
                .expect("master_metadata is not set"),

            update_authority: self
                .instruction
                .update_authority
                .expect("update_authority is not set"),

            spl_token_program: self
                .instruction
                .spl_token_program
                .expect("spl_token_program is not set"),

            spl_ata_program: self
                .instruction
                .spl_ata_program
                .expect("spl_ata_program is not set"),

            sysvar_instructions: self
                .instruction
                .sysvar_instructions
                .expect("sysvar_instructions is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
            __args: args,
        }
    }
}

struct PrintV1CpiBuilderInstruction<'a> {
    __program: &'a solana_program::account_info::AccountInfo<'a>,
    edition_metadata: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    edition: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    edition_mint: Option<(&'a solana_program::account_info::AccountInfo<'a>, bool)>,
    edition_token_account_owner: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    edition_token_account: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    edition_mint_authority: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    edition_token_record: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    master_edition: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    edition_marker_pda: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    master_token_account_owner: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    master_token_account: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    master_metadata: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    update_authority: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    spl_token_program: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    spl_ata_program: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    sysvar_instructions: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    edition_number: Option<u64>,
}
