# 🔥 Token Burn Extension (Solana + Anchor)

This project extends a basic token smart contract by adding a burn mechanism that allows tokens to be permanently removed from circulation.

## 🎯 Purpose

To demonstrate how token supply can be reduced by implementing a burn function in a Solana smart contract.

## ⚙️ Functionality

### 🔥 burn_tokens(amount)
Burns a specified amount of tokens from a user's token account and reduces total supply.

## 📦 Accounts Structure

```rust
pub struct BurnTokens<'info> {
    pub mint: Account<'info, Mint>,
    pub token_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
