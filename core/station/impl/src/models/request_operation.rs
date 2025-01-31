use super::{
    permission::{Allow, AuthScope},
    request_policy_rule::{RequestPolicyRule, RequestPolicyRuleInput},
    request_specifier::RequestSpecifier,
    resource::Resource,
    AccountId, AddressBookEntryId, Blockchain, BlockchainStandard, ChangeMetadata, MetadataItem,
    UserGroupId, UserId, UserStatus,
};
use crate::core::validation::EnsureExternalCanister;
use crate::errors::ValidationError;
use crate::models::Metadata;
use candid::Principal;
use orbit_essentials::cdk::api::management_canister::main::{self as mgmt};
use orbit_essentials::model::{ModelValidator, ModelValidatorResult};
use orbit_essentials::{storable, types::UUID};
use std::fmt::Display;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RequestOperation {
    Transfer(TransferOperation),
    AddAccount(AddAccountOperation),
    EditAccount(EditAccountOperation),
    AddAddressBookEntry(AddAddressBookEntryOperation),
    EditAddressBookEntry(EditAddressBookEntryOperation),
    RemoveAddressBookEntry(RemoveAddressBookEntryOperation),
    AddUser(AddUserOperation),
    EditUser(EditUserOperation),
    EditPermission(EditPermissionOperation),
    AddUserGroup(AddUserGroupOperation),
    EditUserGroup(EditUserGroupOperation),
    RemoveUserGroup(RemoveUserGroupOperation),
    ChangeCanister(ChangeCanisterOperation),
    ChangeExternalCanister(ChangeExternalCanisterOperation),
    CreateExternalCanister(CreateExternalCanisterOperation),
    CallExternalCanister(CallExternalCanisterOperation),
    AddRequestPolicy(AddRequestPolicyOperation),
    EditRequestPolicy(EditRequestPolicyOperation),
    RemoveRequestPolicy(RemoveRequestPolicyOperation),
    ManageSystemInfo(ManageSystemInfoOperation),
}

impl Display for RequestOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestOperation::Transfer(_) => write!(f, "transfer"),
            RequestOperation::AddAccount(_) => write!(f, "add_account"),
            RequestOperation::EditAccount(_) => write!(f, "edit_account"),
            RequestOperation::AddAddressBookEntry(_) => write!(f, "add_address_book_entry"),
            RequestOperation::EditAddressBookEntry(_) => write!(f, "edit_address_book_entry"),
            RequestOperation::RemoveAddressBookEntry(_) => write!(f, "remove_address_book_entry"),
            RequestOperation::AddUser(_) => write!(f, "add_user"),
            RequestOperation::EditUser(_) => write!(f, "edit_user"),
            RequestOperation::EditPermission(_) => write!(f, "edit_permission"),
            RequestOperation::AddUserGroup(_) => write!(f, "add_user_group"),
            RequestOperation::EditUserGroup(_) => write!(f, "adit_user_group"),
            RequestOperation::RemoveUserGroup(_) => write!(f, "remove_user_group"),
            RequestOperation::ChangeCanister(_) => write!(f, "change_canister"),
            RequestOperation::ChangeExternalCanister(_) => write!(f, "change_external_canister"),
            RequestOperation::CreateExternalCanister(_) => write!(f, "create_external_canister"),
            RequestOperation::CallExternalCanister(_) => write!(f, "call_external_canister"),
            RequestOperation::AddRequestPolicy(_) => write!(f, "add_request_policy"),
            RequestOperation::EditRequestPolicy(_) => write!(f, "edit_request_policy"),
            RequestOperation::RemoveRequestPolicy(_) => write!(f, "remove_request_policy"),
            RequestOperation::ManageSystemInfo(_) => write!(f, "manage_system_info"),
        }
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferOperation {
    pub transfer_id: Option<UUID>,
    pub input: TransferOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferOperationInput {
    pub from_account_id: AccountId,
    pub to: String,
    pub amount: candid::Nat,
    pub metadata: Metadata,
    pub network: String,
    pub fee: Option<candid::Nat>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAccountOperation {
    /// The account id is only available after the operation is executed.
    pub account_id: Option<AccountId>,
    pub input: AddAccountOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAccountOperationInput {
    pub name: String,
    pub blockchain: Blockchain,
    pub standard: BlockchainStandard,
    pub metadata: Metadata,
    pub read_permission: Allow,
    pub configs_permission: Allow,
    pub transfer_permission: Allow,
    pub configs_request_policy: Option<RequestPolicyRule>,
    pub transfer_request_policy: Option<RequestPolicyRule>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAccountOperation {
    pub input: EditAccountOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAccountOperationInput {
    pub account_id: AccountId,
    pub name: Option<String>,
    pub read_permission: Option<Allow>,
    pub configs_permission: Option<Allow>,
    pub transfer_permission: Option<Allow>,
    pub configs_request_policy: Option<RequestPolicyRuleInput>,
    pub transfer_request_policy: Option<RequestPolicyRuleInput>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAddressBookEntryOperation {
    /// The address book entry id is only available after the operation is executed.
    pub address_book_entry_id: Option<AddressBookEntryId>,
    pub input: AddAddressBookEntryOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddAddressBookEntryOperationInput {
    pub address_owner: String,
    pub address: String,
    pub blockchain: Blockchain,
    pub standard: BlockchainStandard,
    pub metadata: Vec<MetadataItem>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAddressBookEntryOperation {
    pub input: EditAddressBookEntryOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditAddressBookEntryOperationInput {
    pub address_book_entry_id: AddressBookEntryId,
    pub address_owner: Option<String>,
    pub change_metadata: Option<ChangeMetadata>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveAddressBookEntryOperation {
    pub input: RemoveAddressBookEntryOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveAddressBookEntryOperationInput {
    pub address_book_entry_id: AddressBookEntryId,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddUserOperation {
    pub user_id: Option<UUID>,
    pub input: AddUserOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddUserOperationInput {
    pub name: String,
    pub identities: Vec<Principal>,
    pub groups: Vec<UUID>,
    pub status: UserStatus,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditUserOperation {
    pub input: EditUserOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditUserOperationInput {
    pub user_id: UUID,
    pub name: Option<String>,
    pub identities: Option<Vec<Principal>>,
    pub groups: Option<Vec<UUID>>,
    pub status: Option<UserStatus>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddUserGroupOperation {
    pub user_group_id: Option<UUID>,
    pub input: AddUserGroupOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddUserGroupOperationInput {
    pub name: String,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditUserGroupOperation {
    pub input: EditUserGroupOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditUserGroupOperationInput {
    pub user_group_id: UUID,
    pub name: String,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveUserGroupOperation {
    pub input: RemoveUserGroupOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveUserGroupOperationInput {
    pub user_group_id: UUID,
}

#[storable]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChangeCanisterTarget {
    UpgradeStation,
    UpgradeUpgrader,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChangeCanisterOperationInput {
    pub target: ChangeCanisterTarget,
    pub module: Vec<u8>,
    pub arg: Option<Vec<u8>>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChangeCanisterOperation {
    pub module_checksum: Vec<u8>,
    pub arg_checksum: Option<Vec<u8>>,
    pub input: ChangeCanisterOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CanisterInstallModeArgs {}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CanisterReinstallModeArgs {}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CanisterUpgradeModeArgs {}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CanisterInstallMode {
    Install(CanisterInstallModeArgs),
    Reinstall(CanisterReinstallModeArgs),
    Upgrade(CanisterUpgradeModeArgs),
}

impl From<CanisterInstallMode> for mgmt::CanisterInstallMode {
    fn from(mode: CanisterInstallMode) -> Self {
        match mode {
            CanisterInstallMode::Install(_) => mgmt::CanisterInstallMode::Install,
            CanisterInstallMode::Reinstall(_) => mgmt::CanisterInstallMode::Reinstall,
            CanisterInstallMode::Upgrade(_) => mgmt::CanisterInstallMode::Upgrade(None),
        }
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChangeExternalCanisterOperationInput {
    pub canister_id: Principal,
    pub mode: CanisterInstallMode,
    pub module: Vec<u8>,
    pub arg: Option<Vec<u8>>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChangeExternalCanisterOperation {
    pub module_checksum: Vec<u8>,
    pub arg_checksum: Option<Vec<u8>>,
    pub input: ChangeExternalCanisterOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CreateExternalCanisterOperationInput {}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CreateExternalCanisterOperation {
    pub canister_id: Option<Principal>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CanisterMethod {
    pub canister_id: Principal,
    pub method_name: String,
}

impl ModelValidator<ValidationError> for CanisterMethod {
    fn validate(&self) -> ModelValidatorResult<ValidationError> {
        EnsureExternalCanister::is_external_canister(self.canister_id)?;

        Ok(())
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CallExternalCanisterOperationInput {
    pub validation_method: Option<CanisterMethod>,
    pub execution_method: CanisterMethod,
    pub arg: Option<Vec<u8>>,
    pub execution_method_cycles: Option<u64>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CallExternalCanisterOperation {
    pub input: CallExternalCanisterOperationInput,
    pub arg_checksum: Option<Vec<u8>>,
    pub arg_rendering: Option<String>,
    pub execution_method_reply: Option<Vec<u8>>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditPermissionOperationInput {
    pub resource: Resource,
    pub auth_scope: Option<AuthScope>,
    pub users: Option<Vec<UserId>>,
    pub user_groups: Option<Vec<UserGroupId>>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditPermissionOperation {
    pub input: EditPermissionOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddRequestPolicyOperationInput {
    pub specifier: RequestSpecifier,
    pub rule: RequestPolicyRule,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AddRequestPolicyOperation {
    pub policy_id: Option<UUID>,
    pub input: AddRequestPolicyOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditRequestPolicyOperationInput {
    pub policy_id: UUID,
    pub specifier: Option<RequestSpecifier>,
    pub rule: Option<RequestPolicyRule>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EditRequestPolicyOperation {
    pub input: EditRequestPolicyOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveRequestPolicyOperationInput {
    pub policy_id: UUID,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RemoveRequestPolicyOperation {
    pub input: RemoveRequestPolicyOperationInput,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ManageSystemInfoOperationInput {
    pub name: Option<String>,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ManageSystemInfoOperation {
    pub input: ManageSystemInfoOperationInput,
}
