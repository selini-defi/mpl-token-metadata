//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::AuthorizationData;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct DelegateAuthorityItemV1 {
    /// Delegate record account
    pub delegate_record: Option<solana_program::pubkey::Pubkey>,
    /// Owner of the delegated account
    pub delegate: solana_program::pubkey::Pubkey,
    /// Metadata account
    pub metadata: solana_program::pubkey::Pubkey,
    /// Master Edition account
    pub master_edition: Option<solana_program::pubkey::Pubkey>,
    /// Token record account
    pub token_record: Option<solana_program::pubkey::Pubkey>,
    /// Mint of metadata
    pub mint: solana_program::pubkey::Pubkey,
    /// Token account of mint
    pub token: Option<solana_program::pubkey::Pubkey>,
    /// Update authority or token owner
    pub authority: solana_program::pubkey::Pubkey,
    /// Payer
    pub payer: solana_program::pubkey::Pubkey,
    /// System Program
    pub system_program: solana_program::pubkey::Pubkey,
    /// Instructions sysvar account
    pub sysvar_instructions: solana_program::pubkey::Pubkey,
    /// SPL Token Program
    pub spl_token_program: Option<solana_program::pubkey::Pubkey>,
    /// Token Authorization Rules Program
    pub authorization_rules_program: Option<solana_program::pubkey::Pubkey>,
    /// Token Authorization Rules account
    pub authorization_rules: Option<solana_program::pubkey::Pubkey>,
}

impl DelegateAuthorityItemV1 {
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction(
        &self,
        args: DelegateAuthorityItemV1InstructionArgs,
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(14);
        if let Some(delegate_record) = self.delegate_record {
            accounts.push(solana_program::instruction::AccountMeta::new(
                delegate_record,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_TOKEN_METADATA_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.delegate,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.metadata,
            false,
        ));
        if let Some(master_edition) = self.master_edition {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                master_edition,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_TOKEN_METADATA_ID,
                false,
            ));
        }
        if let Some(token_record) = self.token_record {
            accounts.push(solana_program::instruction::AccountMeta::new(
                token_record,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_TOKEN_METADATA_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.mint, false,
        ));
        if let Some(token) = self.token {
            accounts.push(solana_program::instruction::AccountMeta::new(token, false));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_TOKEN_METADATA_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.sysvar_instructions,
            false,
        ));
        if let Some(spl_token_program) = self.spl_token_program {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                spl_token_program,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_TOKEN_METADATA_ID,
                false,
            ));
        }
        if let Some(authorization_rules_program) = self.authorization_rules_program {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                authorization_rules_program,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_TOKEN_METADATA_ID,
                false,
            ));
        }
        if let Some(authorization_rules) = self.authorization_rules {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                authorization_rules,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_TOKEN_METADATA_ID,
                false,
            ));
        }
        let mut data = DelegateAuthorityItemV1InstructionData::new()
            .try_to_vec()
            .unwrap();
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
struct DelegateAuthorityItemV1InstructionData {
    discriminator: u8,
    delegate_authority_item_v1_discriminator: u8,
}

impl DelegateAuthorityItemV1InstructionData {
    fn new() -> Self {
        Self {
            discriminator: 44,
            delegate_authority_item_v1_discriminator: 9,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DelegateAuthorityItemV1InstructionArgs {
    pub authorization_data: Option<AuthorizationData>,
}

/// Instruction builder.
#[derive(Default)]
pub struct DelegateAuthorityItemV1Builder {
    delegate_record: Option<solana_program::pubkey::Pubkey>,
    delegate: Option<solana_program::pubkey::Pubkey>,
    metadata: Option<solana_program::pubkey::Pubkey>,
    master_edition: Option<solana_program::pubkey::Pubkey>,
    token_record: Option<solana_program::pubkey::Pubkey>,
    mint: Option<solana_program::pubkey::Pubkey>,
    token: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    sysvar_instructions: Option<solana_program::pubkey::Pubkey>,
    spl_token_program: Option<solana_program::pubkey::Pubkey>,
    authorization_rules_program: Option<solana_program::pubkey::Pubkey>,
    authorization_rules: Option<solana_program::pubkey::Pubkey>,
    authorization_data: Option<AuthorizationData>,
}

impl DelegateAuthorityItemV1Builder {
    pub fn new() -> Self {
        Self::default()
    }
    /// `[optional account]`
    /// Delegate record account
    #[inline(always)]
    pub fn delegate_record(
        &mut self,
        delegate_record: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.delegate_record = Some(delegate_record);
        self
    }
    /// Owner of the delegated account
    #[inline(always)]
    pub fn delegate(&mut self, delegate: solana_program::pubkey::Pubkey) -> &mut Self {
        self.delegate = Some(delegate);
        self
    }
    /// Metadata account
    #[inline(always)]
    pub fn metadata(&mut self, metadata: solana_program::pubkey::Pubkey) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }
    /// `[optional account]`
    /// Master Edition account
    #[inline(always)]
    pub fn master_edition(&mut self, master_edition: solana_program::pubkey::Pubkey) -> &mut Self {
        self.master_edition = Some(master_edition);
        self
    }
    /// `[optional account]`
    /// Token record account
    #[inline(always)]
    pub fn token_record(&mut self, token_record: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_record = Some(token_record);
        self
    }
    /// Mint of metadata
    #[inline(always)]
    pub fn mint(&mut self, mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mint = Some(mint);
        self
    }
    /// `[optional account]`
    /// Token account of mint
    #[inline(always)]
    pub fn token(&mut self, token: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token = Some(token);
        self
    }
    /// Update authority or token owner
    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }
    /// Payer
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// System Program
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
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
    /// `[optional account]`
    /// SPL Token Program
    #[inline(always)]
    pub fn spl_token_program(
        &mut self,
        spl_token_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.spl_token_program = Some(spl_token_program);
        self
    }
    /// `[optional account]`
    /// Token Authorization Rules Program
    #[inline(always)]
    pub fn authorization_rules_program(
        &mut self,
        authorization_rules_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.authorization_rules_program = Some(authorization_rules_program);
        self
    }
    /// `[optional account]`
    /// Token Authorization Rules account
    #[inline(always)]
    pub fn authorization_rules(
        &mut self,
        authorization_rules: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.authorization_rules = Some(authorization_rules);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn authorization_data(&mut self, authorization_data: AuthorizationData) -> &mut Self {
        self.authorization_data = Some(authorization_data);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn build(&self) -> solana_program::instruction::Instruction {
        let accounts = DelegateAuthorityItemV1 {
            delegate_record: self.delegate_record,
            delegate: self.delegate.expect("delegate is not set"),
            metadata: self.metadata.expect("metadata is not set"),
            master_edition: self.master_edition,
            token_record: self.token_record,
            mint: self.mint.expect("mint is not set"),
            token: self.token,
            authority: self.authority.expect("authority is not set"),
            payer: self.payer.expect("payer is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            sysvar_instructions: self.sysvar_instructions.unwrap_or(solana_program::pubkey!(
                "Sysvar1nstructions1111111111111111111111111"
            )),
            spl_token_program: self.spl_token_program,
            authorization_rules_program: self.authorization_rules_program,
            authorization_rules: self.authorization_rules,
        };
        let args = DelegateAuthorityItemV1InstructionArgs {
            authorization_data: self.authorization_data.clone(),
        };

        accounts.instruction(args)
    }
}

/// `delegate_authority_item_v1` CPI instruction.
pub struct DelegateAuthorityItemV1Cpi<'a> {
    /// The program to invoke.
    pub __program: &'a solana_program::account_info::AccountInfo<'a>,
    /// Delegate record account
    pub delegate_record: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    /// Owner of the delegated account
    pub delegate: &'a solana_program::account_info::AccountInfo<'a>,
    /// Metadata account
    pub metadata: &'a solana_program::account_info::AccountInfo<'a>,
    /// Master Edition account
    pub master_edition: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    /// Token record account
    pub token_record: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    /// Mint of metadata
    pub mint: &'a solana_program::account_info::AccountInfo<'a>,
    /// Token account of mint
    pub token: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    /// Update authority or token owner
    pub authority: &'a solana_program::account_info::AccountInfo<'a>,
    /// Payer
    pub payer: &'a solana_program::account_info::AccountInfo<'a>,
    /// System Program
    pub system_program: &'a solana_program::account_info::AccountInfo<'a>,
    /// Instructions sysvar account
    pub sysvar_instructions: &'a solana_program::account_info::AccountInfo<'a>,
    /// SPL Token Program
    pub spl_token_program: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    /// Token Authorization Rules Program
    pub authorization_rules_program: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    /// Token Authorization Rules account
    pub authorization_rules: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    /// The arguments for the instruction.
    pub __args: DelegateAuthorityItemV1InstructionArgs,
}

impl<'a> DelegateAuthorityItemV1Cpi<'a> {
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(14);
        if let Some(delegate_record) = self.delegate_record {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *delegate_record.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_TOKEN_METADATA_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.delegate.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.metadata.key,
            false,
        ));
        if let Some(master_edition) = self.master_edition {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *master_edition.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_TOKEN_METADATA_ID,
                false,
            ));
        }
        if let Some(token_record) = self.token_record {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *token_record.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_TOKEN_METADATA_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.mint.key,
            false,
        ));
        if let Some(token) = self.token {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *token.key, false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_TOKEN_METADATA_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.sysvar_instructions.key,
            false,
        ));
        if let Some(spl_token_program) = self.spl_token_program {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *spl_token_program.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_TOKEN_METADATA_ID,
                false,
            ));
        }
        if let Some(authorization_rules_program) = self.authorization_rules_program {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *authorization_rules_program.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_TOKEN_METADATA_ID,
                false,
            ));
        }
        if let Some(authorization_rules) = self.authorization_rules {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *authorization_rules.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_TOKEN_METADATA_ID,
                false,
            ));
        }
        let mut data = DelegateAuthorityItemV1InstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MPL_TOKEN_METADATA_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(14 + 1);
        account_infos.push(self.__program.clone());
        if let Some(delegate_record) = self.delegate_record {
            account_infos.push(delegate_record.clone());
        }
        account_infos.push(self.delegate.clone());
        account_infos.push(self.metadata.clone());
        if let Some(master_edition) = self.master_edition {
            account_infos.push(master_edition.clone());
        }
        if let Some(token_record) = self.token_record {
            account_infos.push(token_record.clone());
        }
        account_infos.push(self.mint.clone());
        if let Some(token) = self.token {
            account_infos.push(token.clone());
        }
        account_infos.push(self.authority.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.sysvar_instructions.clone());
        if let Some(spl_token_program) = self.spl_token_program {
            account_infos.push(spl_token_program.clone());
        }
        if let Some(authorization_rules_program) = self.authorization_rules_program {
            account_infos.push(authorization_rules_program.clone());
        }
        if let Some(authorization_rules) = self.authorization_rules {
            account_infos.push(authorization_rules.clone());
        }

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// `delegate_authority_item_v1` CPI instruction builder.
pub struct DelegateAuthorityItemV1CpiBuilder<'a> {
    instruction: Box<DelegateAuthorityItemV1CpiBuilderInstruction<'a>>,
}

impl<'a> DelegateAuthorityItemV1CpiBuilder<'a> {
    pub fn new(program: &'a solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(DelegateAuthorityItemV1CpiBuilderInstruction {
            __program: program,
            delegate_record: None,
            delegate: None,
            metadata: None,
            master_edition: None,
            token_record: None,
            mint: None,
            token: None,
            authority: None,
            payer: None,
            system_program: None,
            sysvar_instructions: None,
            spl_token_program: None,
            authorization_rules_program: None,
            authorization_rules: None,
            authorization_data: None,
        });
        Self { instruction }
    }
    /// `[optional account]`
    /// Delegate record account
    #[inline(always)]
    pub fn delegate_record(
        &mut self,
        delegate_record: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.delegate_record = Some(delegate_record);
        self
    }
    /// Owner of the delegated account
    #[inline(always)]
    pub fn delegate(
        &mut self,
        delegate: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.delegate = Some(delegate);
        self
    }
    /// Metadata account
    #[inline(always)]
    pub fn metadata(
        &mut self,
        metadata: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.metadata = Some(metadata);
        self
    }
    /// `[optional account]`
    /// Master Edition account
    #[inline(always)]
    pub fn master_edition(
        &mut self,
        master_edition: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.master_edition = Some(master_edition);
        self
    }
    /// `[optional account]`
    /// Token record account
    #[inline(always)]
    pub fn token_record(
        &mut self,
        token_record: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_record = Some(token_record);
        self
    }
    /// Mint of metadata
    #[inline(always)]
    pub fn mint(&mut self, mint: &'a solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.mint = Some(mint);
        self
    }
    /// `[optional account]`
    /// Token account of mint
    #[inline(always)]
    pub fn token(&mut self, token: &'a solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.token = Some(token);
        self
    }
    /// Update authority or token owner
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority = Some(authority);
        self
    }
    /// Payer
    #[inline(always)]
    pub fn payer(&mut self, payer: &'a solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// System Program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
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
    /// `[optional account]`
    /// SPL Token Program
    #[inline(always)]
    pub fn spl_token_program(
        &mut self,
        spl_token_program: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.spl_token_program = Some(spl_token_program);
        self
    }
    /// `[optional account]`
    /// Token Authorization Rules Program
    #[inline(always)]
    pub fn authorization_rules_program(
        &mut self,
        authorization_rules_program: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authorization_rules_program = Some(authorization_rules_program);
        self
    }
    /// `[optional account]`
    /// Token Authorization Rules account
    #[inline(always)]
    pub fn authorization_rules(
        &mut self,
        authorization_rules: &'a solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authorization_rules = Some(authorization_rules);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn authorization_data(&mut self, authorization_data: AuthorizationData) -> &mut Self {
        self.instruction.authorization_data = Some(authorization_data);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn build(&self) -> DelegateAuthorityItemV1Cpi<'a> {
        let args = DelegateAuthorityItemV1InstructionArgs {
            authorization_data: self.instruction.authorization_data.clone(),
        };

        DelegateAuthorityItemV1Cpi {
            __program: self.instruction.__program,

            delegate_record: self.instruction.delegate_record,

            delegate: self.instruction.delegate.expect("delegate is not set"),

            metadata: self.instruction.metadata.expect("metadata is not set"),

            master_edition: self.instruction.master_edition,

            token_record: self.instruction.token_record,

            mint: self.instruction.mint.expect("mint is not set"),

            token: self.instruction.token,

            authority: self.instruction.authority.expect("authority is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            sysvar_instructions: self
                .instruction
                .sysvar_instructions
                .expect("sysvar_instructions is not set"),

            spl_token_program: self.instruction.spl_token_program,

            authorization_rules_program: self.instruction.authorization_rules_program,

            authorization_rules: self.instruction.authorization_rules,
            __args: args,
        }
    }
}

struct DelegateAuthorityItemV1CpiBuilderInstruction<'a> {
    __program: &'a solana_program::account_info::AccountInfo<'a>,
    delegate_record: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    delegate: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    metadata: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    master_edition: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    token_record: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    mint: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    token: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    sysvar_instructions: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    spl_token_program: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    authorization_rules_program: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    authorization_rules: Option<&'a solana_program::account_info::AccountInfo<'a>>,
    authorization_data: Option<AuthorizationData>,
}
