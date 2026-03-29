#[cfg(test)]
mod tests {
    use super::*;

    // 🔹 create_token_account()
    #[test]
    fn test_create_token_account() {
        let user = "user1";

        let token_account = create_token_account(user);

        assert_eq!(token_account.owner, user);
        assert_eq!(token_account.balance, 0);
    }

    // 🔹 mint_tokens()
    #[test]
    fn test_mint_tokens() {
        let user = "user1";
        let mut account = create_token_account(user);

        mint_tokens(&mut account, 1000);

        assert_eq!(account.balance, 1000);
    }

    // 🔹 transfer_tokens()
    #[test]
    fn test_transfer_tokens() {
        let user1 = "user1";
        let user2 = "user2";

        let mut acc1 = create_token_account(user1);
        let mut acc2 = create_token_account(user2);

        mint_tokens(&mut acc1, 1000);

        transfer_tokens(&mut acc1, &mut acc2, 500);

        assert_eq!(acc1.balance, 500);
        assert_eq!(acc2.balance, 500);
    }
}