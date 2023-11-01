use super::canister_config;
use crate::core::ic_cdk::{
    api::{id as self_canister_id, is_controller, trap},
    caller,
};
use crate::{models::AccessRole, repositories::AccountRepository};
use candid::Principal;

#[derive(Clone, Debug)]
pub struct CallContext {
    caller: Principal,
}

impl Default for CallContext {
    fn default() -> Self {
        Self {
            caller: Principal::anonymous(),
        }
    }
}

impl CallContext {
    pub fn new(caller: Principal) -> Self {
        Self { caller }
    }

    /// This method can only be used before any await has been called in the current call context,
    /// otherwise it will panic.
    pub fn get() -> Self {
        Self { caller: caller() }
    }

    pub fn caller(&self) -> Principal {
        self.caller
    }

    /// Checks if the caller is an admin.
    pub fn is_admin(&self) -> bool {
        if self.caller == self_canister_id() || is_controller(&self.caller) {
            return true;
        }

        let account: Option<crate::models::Account> =
            AccountRepository::default().find_account_by_identity(&self.caller);

        match account {
            Some(account) => account.access_roles.contains(&AccessRole::Admin),
            None => false,
        }
    }

    /// Checks if the caller has the required access role to perform the given action.
    pub fn check_access(&self, permission: &str) {
        if !self.is_admin() {
            check_access(permission, self.caller.to_owned())
        }
    }
}

/// This function checks if the user has the required access role to perform the given action.
fn check_access(permission: &str, caller: Principal) {
    let permissions = canister_config().permissions;
    let permission = permissions
        .iter()
        .find(|p| p.permission_id == permission)
        .unwrap_or_else(|| trap(format!("Permission {} not found", permission).as_str()));

    if permission.access_roles.contains(&AccessRole::Guest) {
        return;
    }

    let account = AccountRepository::default()
        .find_account_by_identity(&caller)
        .unwrap_or_else(|| {
            trap(
                format!(
                    "Access denied for user with principal `{}`",
                    caller.to_text()
                )
                .as_str(),
            )
        });

    if account.access_roles.contains(&AccessRole::Admin) {
        // Admins have access to everything
        return;
    }

    let user_has_access = permission
        .access_roles
        .iter()
        .any(|required_role| account.access_roles.contains(required_role));

    if !user_has_access {
        trap(
            format!(
                "Access denied for user with principal `{}`",
                caller.to_text()
            )
            .as_str(),
        );
    }
}

pub trait WithCallContext {
    fn with_call_context(call_context: CallContext) -> Self;
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{core::test_utils, models::Account};
    use ic_canister_core::{cdk::mocks::TEST_CANISTER_ID, repository::Repository};

    #[test]
    fn check_caller_is_not_admin() {
        let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let account = Account {
            id: [1; 16],
            identities: vec![caller],
            unconfirmed_identities: vec![],
            access_roles: vec![AccessRole::User],
            last_modification_timestamp: 0,
        };
        let account_repository = AccountRepository::default();
        account_repository.insert(account.to_key(), account.clone());

        let call_context = CallContext::new(caller);
        assert!(!call_context.is_admin());
    }

    #[test]
    fn check_caller_is_admin() {
        let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let account = Account {
            id: [1; 16],
            identities: vec![caller],
            unconfirmed_identities: vec![],
            access_roles: vec![AccessRole::Admin, AccessRole::User],
            last_modification_timestamp: 0,
        };
        let account_repository = AccountRepository::default();
        account_repository.insert(account.to_key(), account.clone());

        let call_context = CallContext::new(caller);
        assert!(call_context.is_admin());
    }

    #[test]
    fn check_self_canister_call_is_admin() {
        let call_context = CallContext::new(TEST_CANISTER_ID);
        assert!(call_context.is_admin());
    }

    #[test]
    fn admin_should_have_access_to_all() {
        let canister_config = test_utils::init_canister_config();
        let admin_permission = canister_config
            .permissions
            .iter()
            .find(|p| p.access_roles.contains(&AccessRole::Admin))
            .unwrap_or_else(|| panic!("Permission with admin requirement not found"));

        let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let account = Account {
            id: [1; 16],
            identities: vec![caller],
            unconfirmed_identities: vec![],
            access_roles: vec![AccessRole::Admin],
            last_modification_timestamp: 0,
        };
        let account_repository = AccountRepository::default();
        account_repository.insert(account.to_key(), account.clone());

        let call_context = CallContext::new(caller);
        call_context.check_access(admin_permission.permission_id.as_str());
    }

    #[test]
    #[should_panic]
    fn fail_user_has_access_to_admin_permission() {
        let canister_config = test_utils::init_canister_config();
        let admin_permission = canister_config
            .permissions
            .iter()
            .find(|p| p.access_roles.contains(&AccessRole::Admin))
            .unwrap_or_else(|| panic!("Permission with admin requirement not found"));

        let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let account = Account {
            id: [1; 16],
            identities: vec![caller],
            unconfirmed_identities: vec![],
            access_roles: vec![AccessRole::User],
            last_modification_timestamp: 0,
        };
        let account_repository = AccountRepository::default();
        account_repository.insert(account.to_key(), account.clone());

        let call_context = CallContext::new(caller);
        call_context.check_access(admin_permission.permission_id.as_str());
    }

    #[test]
    fn any_user_has_access_to_guest_permission() {
        let canister_config = test_utils::init_canister_config();
        let guest_permission = canister_config
            .permissions
            .iter()
            .find(|p| p.access_roles.contains(&AccessRole::Guest))
            .unwrap_or_else(|| panic!("Permission with guest requirement not found"));

        let caller = Principal::from_text("avqkn-guaaa-aaaaa-qaaea-cai").unwrap();
        let call_context = CallContext::new(caller);
        call_context.check_access(guest_permission.permission_id.as_str());
    }
}
