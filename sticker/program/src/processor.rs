//! Program state processor

use crate::{
    error::StickerProgramError,
    instruction::{AddressType, CreateNewSticker, RegisterArtist, StickerInstruction},
    state::{Artist, Sticker, StickerFactory},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::next_account_info,
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
    sysvar::rent::Rent,
    sysvar::Sysvar,
};
use spl_nft_erc_721::state::Mint;
use spl_token::state::Account;

/// Program state handler.
pub struct Processor {}
impl Processor {
    /// ARTIST_SEED
    pub const ARTIST_SEED: &'static str = "artist";
    /// STICKER_SEED
    pub const STICKER_SEED: &'static str = "sticker";

    /// Transfer tokens with authority
    fn transfer<'a>(
        token_program_id: AccountInfo<'a>,
        source_account: AccountInfo<'a>,
        destination_account: AccountInfo<'a>,
        user_authority_account: AccountInfo<'a>,
        amount: u64,
    ) -> ProgramResult {
        invoke(
            &spl_token::instruction::transfer(
                token_program_id.key,
                source_account.key,
                destination_account.key,
                user_authority_account.key,
                &[user_authority_account.key],
                amount,
            )
            .unwrap(),
            &[
                token_program_id,
                user_authority_account,
                source_account,
                destination_account,
            ],
        )
    }

    /// Initialize sticker mint
    fn initialize_nft_mint<'a>(
        nft_program_id: AccountInfo<'a>,
        mint: AccountInfo<'a>,
        authority: AccountInfo<'a>,
        symbol: [u8; 8],
        name: [u8; 32],
        sticker_key: &Pubkey,
        bump_seed: u8,
    ) -> ProgramResult {
        let authority_signature_seeds = [&sticker_key.to_bytes()[..32], &[bump_seed]];
        let signers = &[&authority_signature_seeds[..]];
        let mint_data = spl_nft_erc_721::instruction::MintData { symbol, name };
        invoke_signed(
            &spl_nft_erc_721::instruction::NftInstruction::initialize_mint(
                mint.key,
                mint_data,
                authority.key,
            ),
            &[nft_program_id, mint, authority],
            signers,
        )
    }

    /// Mint new sticker
    #[allow(clippy::too_many_arguments)]
    fn create_new_sticker<'a>(
        nft_program_id: AccountInfo<'a>,
        token_account: AccountInfo<'a>,
        token_data_account: AccountInfo<'a>,
        mint_account: AccountInfo<'a>,
        owner_account: AccountInfo<'a>,
        mint_authority: AccountInfo<'a>,
        sticker_key: &Pubkey,
        bump_seed: u8,
        hash: Pubkey,
        uri: [u8; 256],
    ) -> ProgramResult {
        let authority_signature_seeds = [&sticker_key.to_bytes()[..32], &[bump_seed]];
        let signers = &[&authority_signature_seeds[..]];
        let token_data_args = spl_nft_erc_721::instruction::TokenDataArgs { hash, uri };
        invoke_signed(
            &spl_nft_erc_721::instruction::NftInstruction::initialize_token(
                token_account.key,
                token_data_account.key,
                mint_account.key,
                owner_account.key,
                token_data_args,
                mint_authority.key,
            ),
            &[
                token_account,
                token_data_account,
                mint_account,
                owner_account,
                mint_authority,
                nft_program_id,
            ],
            signers,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn create_account<'a>(
        program_id: &Pubkey,
        funder: AccountInfo<'a>,
        account_to_create: AccountInfo<'a>,
        sticker_factory_account: &Pubkey,
        base: AccountInfo<'a>,
        seed: &str,
        required_lamports: u64,
        space: u64,
        owner: &Pubkey,
        index: u64,
    ) -> ProgramResult {
        let (program_base_address, bump_seed) =
            Pubkey::find_program_address(&[&sticker_factory_account.to_bytes()[..32]], program_id);
        if program_base_address != *base.key {
            return Err(ProgramError::InvalidSeeds);
        }

        let full_seed = format!("{:?}{}", index, seed);

        let generated_address_to_create =
            Pubkey::create_with_seed(&program_base_address, &full_seed, program_id)?;
        if generated_address_to_create != *account_to_create.key {
            return Err(ProgramError::InvalidSeeds);
        }
        let signature = &[&sticker_factory_account.to_bytes()[..32], &[bump_seed]];

        invoke_signed(
            &system_instruction::create_account_with_seed(
                &funder.key,
                &account_to_create.key,
                &base.key,
                &full_seed,
                required_lamports,
                space,
                owner,
            ),
            &[funder.clone(), account_to_create.clone(), base.clone()],
            &[signature],
        )
    }

    /// Register new artist
    pub fn process_register_artist_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        args: RegisterArtist,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let user_account_info = next_account_info(account_info_iter)?;
        let user_token_account_info = next_account_info(account_info_iter)?;
        let artist_to_create_account_info = next_account_info(account_info_iter)?;
        let sticker_factory_owner_account_info = next_account_info(account_info_iter)?;
        let sticker_factory_account_info = next_account_info(account_info_iter)?;
        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;

        let user_token = Account::unpack_from_slice(&user_token_account_info.data.borrow())?;
        if !user_token.is_native() {
            return Err(StickerProgramError::WrongTokenMint.into());
        }

        let mut sticker_factory =
            StickerFactory::try_from_slice(&sticker_factory_account_info.data.borrow())?;
        if !sticker_factory.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        if *sticker_factory_owner_account_info.key != sticker_factory.owner {
            return Err(StickerProgramError::WrongStickerFactoryOwner.into());
        }

        if !sticker_factory_owner_account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let (base, _) = Pubkey::find_program_address(
            &[&sticker_factory_account_info.key.to_bytes()[..32]],
            program_id,
        );
        let generated_artist_key = Pubkey::create_with_seed(
            &base,
            &format!("{:?}{}", sticker_factory.artist_count, Self::ARTIST_SEED),
            program_id,
        )?;
        if generated_artist_key != *artist_to_create_account_info.key {
            return Err(ProgramError::InvalidSeeds);
        }

        if !rent.is_exempt(
            artist_to_create_account_info.lamports(),
            artist_to_create_account_info.data_len(),
        ) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        let mut artist = Artist::try_from_slice(&artist_to_create_account_info.data.borrow())?;
        if artist.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        artist.user = *user_account_info.key;
        artist.user_token_acc = *user_token_account_info.key;
        artist.name = args.name;
        artist.signature = args.signature;
        artist.description = args.description;

        sticker_factory.artist_count = sticker_factory
            .artist_count
            .checked_add(1)
            .ok_or_else(|| StickerProgramError::CalculationError)?;

        artist.serialize(&mut *artist_to_create_account_info.data.borrow_mut())?;
        sticker_factory
            .serialize(&mut *sticker_factory_account_info.data.borrow_mut())
            .map_err(|e| e.into())
    }

    /// Create new sticker
    pub fn process_create_new_sticker_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        args: CreateNewSticker,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let sticker_account_info = next_account_info(account_info_iter)?;
        let sticker_factory_account_info = next_account_info(account_info_iter)?;
        let mint_account_info = next_account_info(account_info_iter)?;
        let artist_account_info = next_account_info(account_info_iter)?;
        let user_account_info = next_account_info(account_info_iter)?;
        let mint_authority_account_info = next_account_info(account_info_iter)?;
        let nft_token_program_id = next_account_info(account_info_iter)?;
        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;

        if *nft_token_program_id.key != spl_nft_erc_721::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut sticker_factory =
            StickerFactory::try_from_slice(&sticker_factory_account_info.data.borrow())?;
        if !sticker_factory.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        let (base, _) = Pubkey::find_program_address(
            &[&sticker_factory_account_info.key.to_bytes()[..32]],
            program_id,
        );
        let generated_sticker_key = Pubkey::create_with_seed(
            &base,
            &format!(
                "{:?}{}",
                sticker_factory.sticker_count,
                Self::STICKER_SEED
            ),
            program_id,
        )?;
        if generated_sticker_key != *sticker_account_info.key {
            return Err(ProgramError::InvalidSeeds);
        }

        let nft_mint = Mint::try_from_slice(&mint_account_info.data.borrow())?;
        if nft_mint.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        let (generated_mint_auth, bump_seed) =
            Pubkey::find_program_address(&[&sticker_account_info.key.to_bytes()[..32]], program_id);
        if generated_mint_auth != *mint_authority_account_info.key {
            return Err(StickerProgramError::WrongTokenMintAuthority.into());
        }

        Self::initialize_nft_mint(
            nft_token_program_id.clone(),
            mint_account_info.clone(),
            mint_authority_account_info.clone(),
            args.symbol,
            args.name,
            sticker_account_info.key,
            bump_seed,
        )?;

        let artist = Artist::try_from_slice(&artist_account_info.data.borrow())?;
        if !artist.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        if *user_account_info.key != artist.user || !user_account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !rent.is_exempt(
            sticker_account_info.lamports(),
            sticker_account_info.data_len(),
        ) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        let mut sticker = Sticker::try_from_slice(&sticker_account_info.data.borrow())?;
        if sticker.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        sticker.creator = artist.user;
        sticker.supply = 0;
        sticker.max_supply = args.max_supply;
        sticker.price = args.price;
        sticker.mint = *mint_account_info.key;
        sticker.uri = args.uri;

        sticker_factory.sticker_count = sticker_factory
            .sticker_count
            .checked_add(1)
            .ok_or_else(|| StickerProgramError::CalculationError)?;

        sticker_factory.serialize(&mut *sticker_factory_account_info.data.borrow_mut())?;
        sticker
            .serialize(&mut *sticker_account_info.data.borrow_mut())
            .map_err(|e| e.into())
    }

    /// Create new sticker
    pub fn process_create_new_sticker_factory_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let sticker_factory_account_info = next_account_info(account_info_iter)?;
        let sticker_factory_owner_account_info = next_account_info(account_info_iter)?;
        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;

        let mut sticker_factory =
            StickerFactory::try_from_slice(&sticker_factory_account_info.data.borrow())?;
        if sticker_factory.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        if !rent.is_exempt(
            sticker_factory_account_info.lamports(),
            sticker_factory_account_info.data_len(),
        ) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        if !sticker_factory_owner_account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        sticker_factory.artist_count = 0;
        sticker_factory.sticker_count = 0;
        sticker_factory.owner = *sticker_factory_owner_account_info.key;

        sticker_factory
            .serialize(&mut *sticker_factory_account_info.data.borrow_mut())
            .map_err(|e| e.into())
    }

    /// Buy sticker
    pub fn process_buy_sticker_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let sticker_to_buy_account_info = next_account_info(account_info_iter)?;
        let artist_account_info = next_account_info(account_info_iter)?;
        let artist_token_account_info = next_account_info(account_info_iter)?;
        let buyer_token_account_info = next_account_info(account_info_iter)?;
        let buyer_transfer_authority_account_info = next_account_info(account_info_iter)?;
        let mint_authority = next_account_info(account_info_iter)?;
        let nft_token_account_info = next_account_info(account_info_iter)?;
        let nft_token_data_account_info = next_account_info(account_info_iter)?;
        let nft_token_mint_account_info = next_account_info(account_info_iter)?;
        let nft_token_owner_account_info = next_account_info(account_info_iter)?;
        let token_program_id = next_account_info(account_info_iter)?;
        let nft_token_program_id = next_account_info(account_info_iter)?;
        // Need in Rent account because we call NFT program instruction which uses it
        let _rent_account_info = next_account_info(account_info_iter)?;

        if *token_program_id.key != spl_token::id()
            || *nft_token_program_id.key != spl_nft_erc_721::id()
        {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut sticker = Sticker::try_from_slice(&sticker_to_buy_account_info.data.borrow())?;
        if !sticker.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        let artist = Artist::try_from_slice(&artist_account_info.data.borrow())?;
        if !artist.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        if sticker.creator != artist.user || *artist_token_account_info.key != artist.user_token_acc
        {
            return Err(StickerProgramError::WrongStickerCreator.into());
        }

        let (generated_mint_auth, bump_seed) = Pubkey::find_program_address(
            &[&sticker_to_buy_account_info.key.to_bytes()[..32]],
            program_id,
        );
        if generated_mint_auth != *mint_authority.key {
            return Err(StickerProgramError::WrongTokenMintAuthority.into());
        }

        if sticker.supply == sticker.max_supply {
            return Err(StickerProgramError::NoTokensToMint.into());
        }

        Self::transfer(
            token_program_id.clone(),
            buyer_token_account_info.clone(),
            artist_token_account_info.clone(),
            buyer_transfer_authority_account_info.clone(),
            sticker.price,
        )?;

        Self::create_new_sticker(
            nft_token_program_id.clone(),
            nft_token_account_info.clone(),
            nft_token_data_account_info.clone(),
            nft_token_mint_account_info.clone(),
            nft_token_owner_account_info.clone(),
            mint_authority.clone(),
            sticker_to_buy_account_info.key,
            bump_seed,
            *sticker_to_buy_account_info.key,
            sticker.uri,
        )?;

        sticker.supply = sticker
            .supply
            .checked_add(1)
            .ok_or_else(|| StickerProgramError::CalculationError)?;

        sticker
            .serialize(&mut *sticker_to_buy_account_info.data.borrow_mut())
            .map_err(|e| e.into())
    }

    /// Change sticker price
    pub fn process_change_sticker_price_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        new_price: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let sticker_account_info = next_account_info(account_info_iter)?;
        let creator_account_info = next_account_info(account_info_iter)?;

        let mut sticker = Sticker::try_from_slice(&sticker_account_info.data.borrow())?;
        if !sticker.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        if *creator_account_info.key != sticker.creator || !creator_account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        sticker.price = new_price;

        sticker
            .serialize(&mut *sticker_account_info.data.borrow_mut())
            .map_err(|e| e.into())
    }

    /// Create new account
    pub fn process_create_account_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        address_type: AddressType,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer_account_info = next_account_info(account_info_iter)?;
        let sticker_factory_account_info = next_account_info(account_info_iter)?;
        let base_address_account_info = next_account_info(account_info_iter)?;
        let account_to_create_account_info = next_account_info(account_info_iter)?;
        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;
        // Need in System Program account because we call create_account_with_seed instruction which requires it
        let _system_program = next_account_info(account_info_iter)?;

        let sticker_factory =
            StickerFactory::try_from_slice(&sticker_factory_account_info.data.borrow())?;
        if !sticker_factory.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        match address_type {
            AddressType::Artist => {
                Self::create_account(
                    program_id,
                    payer_account_info.clone(),
                    account_to_create_account_info.clone(),
                    sticker_factory_account_info.key,
                    base_address_account_info.clone(),
                    Self::ARTIST_SEED,
                    rent.minimum_balance(Artist::LEN),
                    Artist::LEN as u64,
                    program_id,
                    sticker_factory.artist_count,
                )?;
            }
            AddressType::Sticker => {
                Self::create_account(
                    program_id,
                    payer_account_info.clone(),
                    account_to_create_account_info.clone(),
                    sticker_factory_account_info.key,
                    base_address_account_info.clone(),
                    Self::STICKER_SEED,
                    rent.minimum_balance(Sticker::LEN),
                    Sticker::LEN as u64,
                    program_id,
                    sticker_factory.sticker_count,
                )?;
            }
        }

        Ok(())
    }

    /// Processes an instruction
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction = StickerInstruction::try_from_slice(input)?;
        match instruction {
            StickerInstruction::RegisterArtist(args) => {
                msg!("Instruction: RegisterArtist");
                Self::process_register_artist_instruction(program_id, accounts, args)
            }
            StickerInstruction::CreateNewSticker(args) => {
                msg!("Instruction: CreateNewSticker");
                Self::process_create_new_sticker_instruction(program_id, accounts, args)
            }
            StickerInstruction::BuySticker => {
                msg!("Instruction: BuySticker");
                Self::process_buy_sticker_instruction(program_id, accounts)
            }
            StickerInstruction::ChangeStickerPrice(new_price) => {
                msg!("Instruction: ChangeStickerPrice");
                Self::process_change_sticker_price_instruction(program_id, accounts, new_price)
            }
            StickerInstruction::CreateAccount(account_type) => {
                msg!("Instruction: CreateAccount");
                Self::process_create_account_instruction(program_id, accounts, account_type)
            }
            StickerInstruction::CreateStickerFactory => {
                msg!("Instruction: CreateStickerFactory");
                Self::process_create_new_sticker_factory_instruction(program_id, accounts)
            }
        }
    }
}
