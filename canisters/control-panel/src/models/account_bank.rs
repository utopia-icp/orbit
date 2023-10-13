use crate::{core::MAX_BYTE_SIZE_PRINCIPAL, errors::AccountError};
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_canister_core::model::{ModelValidator, ModelValidatorResult};
use ic_canister_macros::stable_object;

#[stable_object(size = AccountBank::MAX_BYTE_SIZE)]
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct AccountBank {
    pub canister_id: Principal,
    pub name: Option<String>,
}

impl AccountBank {
    /// The maximum size of each field in stable memory.
    pub const MAX_BYTE_SIZE_CANISTER_ID: u32 = MAX_BYTE_SIZE_PRINCIPAL;
    pub const MAX_BYTE_SIZE_NAME: u32 = 150;

    /// The maximum size of the Bank information in stable memory.
    pub const MAX_BYTE_SIZE: u32 = 256;

    /// The number of bytes that are not used by the account and could be used to add more fields to the account
    /// without breaking the stable memory layout, if this overflows then the stable memory layout will be broken.
    pub const SPARE_BYTES: u32 =
        Self::MAX_BYTE_SIZE - Self::MAX_BYTE_SIZE_CANISTER_ID - Self::MAX_BYTE_SIZE_NAME;
}

pub struct AccountBankValidator<'model> {
    model: &'model AccountBank,
}

impl<'model> AccountBankValidator<'model> {
    pub const NAME_LEN_RANGE: (u8, u8) = (1, 150);

    pub fn new(model: &'model AccountBank) -> Self {
        Self { model }
    }

    pub fn validate_name(&self) -> ModelValidatorResult<AccountError> {
        if let Some(name) = &self.model.name {
            if (name.len() < Self::NAME_LEN_RANGE.0 as usize)
                || (name.len() > Self::NAME_LEN_RANGE.1 as usize)
            {
                return Err(AccountError::ValidationError {
                    info: format!(
                        "Bank name length must be between {} and {}",
                        Self::NAME_LEN_RANGE.0,
                        Self::NAME_LEN_RANGE.1
                    ),
                });
            }
        }

        Ok(())
    }

    pub fn validate(&self) -> ModelValidatorResult<AccountError> {
        self.validate_name()?;

        Ok(())
    }
}

impl ModelValidator<AccountError> for AccountBank {
    fn validate(&self) -> ModelValidatorResult<AccountError> {
        AccountBankValidator::new(self).validate()
    }
}
