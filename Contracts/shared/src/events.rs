#![no_std]

use soroban_sdk::{
    contracttype, Address, Env, Map, Symbol, Val, Vec, IntoVal,
};

/// Standardized event structure for consistent indexing
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StandardEvent {
    pub event_type: Symbol,
    pub contract_address: Address,
    pub user_address: Option<Address>,
    pub data: Vec<Val>,
    pub metadata: Map<Symbol, Vec<Val>>,
    pub timestamp: u64,
    pub version: u32,
}

/// Standard event topics for consistent naming
pub mod topics {
    pub const TRANSFER: Symbol = symbol_short!("transfer");
    pub const APPROVE: Symbol = symbol_short!("approve");
    pub const MINT: Symbol = symbol_short!("mint");
    pub const BURN: Symbol = symbol_short!("burn");
    pub const STAKE: Symbol = symbol_short!("stake");
    pub const UNSTAKE: Symbol = symbol_short!("unstake");
    pub const REWARDS_CLAIMED: Symbol = symbol_short!("rewards_claimed");
    pub const POOL_UPDATED: Symbol = symbol_short!("pool_updated");
    pub const VOTE: Symbol = symbol_short!("vote");
    pub const PROPOSAL_CREATED: Symbol = symbol_short!("propose");
    pub const PROPOSAL_EXECUTED: Symbol = symbol_short!("execute");
    pub const TRADE_EXECUTED: Symbol = symbol_short!("trade");
    pub const FEE_COLLECTED: Symbol = symbol_short!("fee");
    pub const ADMIN_CHANGED: Symbol = symbol_short!("admin_changed");
    pub const AUTHORIZATION_CHANGED: Symbol = symbol_short!("auth_changed");
}

/// Standardized event emitter utility
pub struct EventEmitter;

impl EventEmitter {
    pub const CURRENT_VERSION: u32 = 1;

    // Standard metadata keys
    pub const AMOUNT_KEY: Symbol = symbol_short!("amount");
    pub const FROM_KEY: Symbol = symbol_short!("from");
    pub const TO_KEY: Symbol = symbol_short!("to");
    pub const TOKEN_KEY: Symbol = symbol_short!("token");
    pub const PAIR_KEY: Symbol = symbol_short!("pair");
    pub const PRICE_KEY: Symbol = symbol_short!("price");
    pub const FEE_KEY: Symbol = symbol_short!("fee");
    pub const REASON_KEY: Symbol = symbol_short!("reason");
    pub const PROPOSAL_ID_KEY: Symbol = symbol_short!("proposal_id");
    pub const VOTE_TYPE_KEY: Symbol = symbol_short!("vote_type");
    pub const LOCK_PERIOD_KEY: Symbol = symbol_short!("lock_period");
    pub const REWARD_RATE_KEY: Symbol = symbol_short!("reward_rate");

    /// Emit a standardized event
    pub fn emit_standard(
        env: &Env,
        event_type: Symbol,
        user_address: Option<Address>,
        data: Vec<Val>,
        metadata: Map<Symbol, Vec<Val>>,
    ) {
        let event = StandardEvent {
            event_type,
            contract_address: env.current_contract_address(),
            user_address,
            data,
            metadata,
            timestamp: env.ledger().timestamp(),
            version: Self::CURRENT_VERSION,
        };

        env.events().publish(
            (symbol_short!("stellara_event"), event_type),
            event,
        );
    }

    /// Emit a transfer event using standardized format
    pub fn transfer(env: &Env, from: Address, to: Address, amount: i128, token: Address) {
        let mut data = Vec::new(env);
        data.push_back(amount.into_val(env));
        data.push_back(token.into_val(env));

        let mut metadata = Map::new(env);
        metadata.set(Self::AMOUNT_KEY, Vec::from_array(env, [amount.into_val(env)]));
        metadata.set(Self::FROM_KEY, Vec::from_array(env, [from.into_val(env)]));
        metadata.set(Self::TO_KEY, Vec::from_array(env, [to.into_val(env)]));
        metadata.set(Self::TOKEN_KEY, Vec::from_array(env, [token.into_val(env)]));

        Self::emit_standard(env, topics::TRANSFER, Some(from), data, metadata);
        
        // Also emit legacy event for backward compatibility
        env.events().publish(
            (topics::TRANSFER, from, to),
            amount,
        );
    }

    /// Emit an approval event using standardized format
    pub fn approve(env: &Env, from: Address, spender: Address, amount: i128, token: Address) {
        let mut data = Vec::new(env);
        data.push_back(amount.into_val(env));
        data.push_back(token.into_val(env));

        let mut metadata = Map::new(env);
        metadata.set(Self::AMOUNT_KEY, Vec::from_array(env, [amount.into_val(env)]));
        metadata.set(Self::FROM_KEY, Vec::from_array(env, [from.into_val(env)]));
        metadata.set(Self::TO_KEY, Vec::from_array(env, [spender.into_val(env)]));
        metadata.set(Self::TOKEN_KEY, Vec::from_array(env, [token.into_val(env)]));

        Self::emit_standard(env, topics::APPROVE, Some(from), data, metadata);
        
        // Also emit legacy event for backward compatibility
        env.events().publish(
            (topics::APPROVE, from, spender),
            amount,
        );
    }

    /// Emit a mint event using standardized format
    pub fn mint(env: &Env, to: Address, amount: i128, token: Address, reason: Option<String>) {
        let mut data = Vec::new(env);
        data.push_back(amount.into_val(env));
        data.push_back(token.into_val(env));

        let mut metadata = Map::new(env);
        metadata.set(Self::AMOUNT_KEY, Vec::from_array(env, [amount.into_val(env)]));
        metadata.set(Self::TO_KEY, Vec::from_array(env, [to.into_val(env)]));
        metadata.set(Self::TOKEN_KEY, Vec::from_array(env, [token.into_val(env)]));
        
        if let Some(r) = reason {
            metadata.set(Self::REASON_KEY, Vec::from_array(env, [r.clone().into_val(env)]));
        }

        Self::emit_standard(env, topics::MINT, Some(to), data, metadata);
        
        // Also emit legacy event for backward compatibility
        env.events().publish(
            (topics::MINT, to),
            amount,
        );
    }

    /// Emit a burn event using standardized format
    pub fn burn(env: &Env, from: Address, amount: i128, token: Address) {
        let mut data = Vec::new(env);
        data.push_back(amount.into_val(env));
        data.push_back(token.into_val(env));

        let mut metadata = Map::new(env);
        metadata.set(Self::AMOUNT_KEY, Vec::from_array(env, [amount.into_val(env)]));
        metadata.set(Self::FROM_KEY, Vec::from_array(env, [from.into_val(env)]));
        metadata.set(Self::TOKEN_KEY, Vec::from_array(env, [token.into_val(env)]));

        Self::emit_standard(env, topics::BURN, Some(from), data, metadata);
        
        // Also emit legacy event for backward compatibility
        env.events().publish(
            (topics::BURN, from),
            amount,
        );
    }

    /// Emit an admin change event using standardized format
    pub fn admin_changed(env: &Env, old_admin: Address, new_admin: Address) {
        let mut data = Vec::new(env);
        data.push_back(new_admin.into_val(env));

        let mut metadata = Map::new(env);
        metadata.set(Self::FROM_KEY, Vec::from_array(env, [old_admin.into_val(env)]));
        metadata.set(Self::TO_KEY, Vec::from_array(env, [new_admin.into_val(env)]));

        Self::emit_standard(env, topics::ADMIN_CHANGED, Some(old_admin), data, metadata);
        
        // Also emit legacy event for backward compatibility
        env.events().publish(
            (topics::ADMIN_CHANGED, old_admin),
            new_admin,
        );
    }

    /// Emit an authorization change event using standardized format
    pub fn authorization_changed(env: &Env, user: Address, authorized: bool) {
        let mut data = Vec::new(env);
        data.push_back(authorized.into_val(env));

        let mut metadata = Map::new(env);
        metadata.set(Self::TO_KEY, Vec::from_array(env, [user.into_val(env)]));

        Self::emit_standard(env, topics::AUTHORIZATION_CHANGED, Some(user), data, metadata);
        
        // Also emit legacy event for backward compatibility
        env.events().publish(
            (topics::AUTHORIZATION_CHANGED, user),
            authorized,
        );
    }

    /// Emit a staking event using standardized format
    pub fn stake(env: &Env, user: Address, amount: i128, lock_period: u64, token: Address) {
        let mut data = Vec::new(env);
        data.push_back(amount.into_val(env));
        data.push_back(lock_period.into_val(env));
        data.push_back(token.into_val(env));

        let mut metadata = Map::new(env);
        metadata.set(Self::AMOUNT_KEY, Vec::from_array(env, [amount.into_val(env)]));
        metadata.set(Self::LOCK_PERIOD_KEY, Vec::from_array(env, [lock_period.into_val(env)]));
        metadata.set(Self::TOKEN_KEY, Vec::from_array(env, [token.into_val(env)]));

        Self::emit_standard(env, topics::STAKE, Some(user), data, metadata);
        
        // Also emit legacy event for backward compatibility
        env.events().publish(
            (topics::STAKE, user),
            (amount, lock_period, env.ledger().timestamp()),
        );
    }

    /// Emit an unstaking event using standardized format
    pub fn unstake(env: &Env, user: Address, amount: i128, rewards: i128, fee: i128, token: Address) {
        let mut data = Vec::new(env);
        data.push_back(amount.into_val(env));
        data.push_back(rewards.into_val(env));
        data.push_back(fee.into_val(env));
        data.push_back(token.into_val(env));

        let mut metadata = Map::new(env);
        metadata.set(Self::AMOUNT_KEY, Vec::from_array(env, [amount.into_val(env)]));
        metadata.set(Self::FEE_KEY, Vec::from_array(env, [fee.into_val(env)]));
        metadata.set(Self::TOKEN_KEY, Vec::from_array(env, [token.into_val(env)]));

        Self::emit_standard(env, topics::UNSTAKE, Some(user), data, metadata);
        
        // Also emit legacy event for backward compatibility
        env.events().publish(
            (topics::UNSTAKE, user),
            (amount, rewards, fee, env.ledger().timestamp()),
        );
    }

    /// Emit a rewards claimed event using standardized format
    pub fn rewards_claimed(env: &Env, user: Address, base_rewards: i128, bonus_rewards: i128, token: Address) {
        let mut data = Vec::new(env);
        data.push_back(base_rewards.into_val(env));
        data.push_back(bonus_rewards.into_val(env));
        data.push_back(token.into_val(env));

        let mut metadata = Map::new(env);
        metadata.set(Self::AMOUNT_KEY, Vec::from_array(env, [(base_rewards + bonus_rewards).into_val(env)]));
        metadata.set(Self::TOKEN_KEY, Vec::from_array(env, [token.into_val(env)]));

        Self::emit_standard(env, topics::REWARDS_CLAIMED, Some(user), data, metadata);
        
        // Also emit legacy event for backward compatibility
        env.events().publish(
            (topics::REWARDS_CLAIMED, user),
            (base_rewards, bonus_rewards, env.ledger().timestamp()),
        );
    }

    /// Emit a voting event using standardized format
    pub fn vote(env: &Env, voter: Address, proposal_id: u64, vote_type: Symbol, voting_power: u128) {
        let mut data = Vec::new(env);
        data.push_back(proposal_id.into_val(env));
        data.push_back(vote_type.into_val(env));
        data.push_back(voting_power.into_val(env));

        let mut metadata = Map::new(env);
        metadata.set(Self::PROPOSAL_ID_KEY, Vec::from_array(env, [proposal_id.into_val(env)]));
        metadata.set(Self::VOTE_TYPE_KEY, Vec::from_array(env, [vote_type.into_val(env)]));

        Self::emit_standard(env, topics::VOTE, Some(voter), data, metadata);
        
        // Also emit legacy event for backward compatibility
        env.events().publish(
            (topics::VOTE, voter),
            (proposal_id, vote_type, voting_power, env.ledger().timestamp()),
        );
    }

    /// Emit a pool updated event using standardized format
    pub fn pool_updated(env: &Env, admin: Address, reward_rate: i128, bonus_multiplier: u32) {
        let mut data = Vec::new(env);
        data.push_back(reward_rate.into_val(env));
        data.push_back(bonus_multiplier.into_val(env));

        let mut metadata = Map::new(env);
        metadata.set(Self::REWARD_RATE_KEY, Vec::from_array(env, [reward_rate.into_val(env)]));

        Self::emit_standard(env, topics::POOL_UPDATED, Some(admin), data, metadata);
        
        // Also emit legacy event for backward compatibility
        env.events().publish(
            (topics::POOL_UPDATED, admin),
            (reward_rate, bonus_multiplier, env.ledger().timestamp()),
        );
    }

    /// Emit a trade executed event using standardized format
    pub fn trade_executed(
        env: &Env,
        trader: Address,
        pair: Symbol,
        amount: i128,
        price: i128,
        is_buy: bool,
        fee_amount: i128,
        fee_token: Address,
    ) {
        let mut data = Vec::new(env);
        data.push_back(pair.into_val(env));
        data.push_back(amount.into_val(env));
        data.push_back(price.into_val(env));
        data.push_back(is_buy.into_val(env));
        data.push_back(fee_amount.into_val(env));
        data.push_back(fee_token.into_val(env));

        let mut metadata = Map::new(env);
        metadata.set(Self::PAIR_KEY, Vec::from_array(env, [pair.into_val(env)]));
        metadata.set(Self::AMOUNT_KEY, Vec::from_array(env, [amount.into_val(env)]));
        metadata.set(Self::PRICE_KEY, Vec::from_array(env, [price.into_val(env)]));
        metadata.set(Self::FEE_KEY, Vec::from_array(env, [fee_amount.into_val(env)]));
        metadata.set(Self::TOKEN_KEY, Vec::from_array(env, [fee_token.into_val(env)]));

        Self::emit_standard(env, topics::TRADE_EXECUTED, Some(trader), data, metadata);
    }

    /// Emit a fee collected event using standardized format
    pub fn fee_collected(env: &Env, payer: Address, recipient: Address, amount: i128, token: Address) {
        let mut data = Vec::new(env);
        data.push_back(amount.into_val(env));
        data.push_back(token.into_val(env));

        let mut metadata = Map::new(env);
        metadata.set(Self::FROM_KEY, Vec::from_array(env, [payer.into_val(env)]));
        metadata.set(Self::TO_KEY, Vec::from_array(env, [recipient.into_val(env)]));
        metadata.set(Self::AMOUNT_KEY, Vec::from_array(env, [amount.into_val(env)]));
        metadata.set(Self::TOKEN_KEY, Vec::from_array(env, [token.into_val(env)]));

        Self::emit_standard(env, topics::FEE_COLLECTED, Some(payer), data, metadata);
    }

    /// Emit a proposal created event using standardized format
    pub fn proposal_created(env: &Env, proposer: Address, proposal_id: u64, title: String, proposal_type: Symbol) {
        let mut data = Vec::new(env);
        data.push_back(proposal_id.into_val(env));
        data.push_back(title.clone().into_val(env));
        data.push_back(proposal_type.into_val(env));

        let mut metadata = Map::new(env);
        metadata.set(Self::PROPOSAL_ID_KEY, Vec::from_array(env, [proposal_id.into_val(env)]));

        Self::emit_standard(env, topics::PROPOSAL_CREATED, Some(proposer), data, metadata);
        
        // Also emit legacy event for backward compatibility
        env.events().publish(
            (topics::PROPOSAL_CREATED, proposer),
            (proposal_id, title, proposal_type, env.ledger().timestamp()),
        );
    }

    /// Emit a proposal executed event using standardized format
    pub fn proposal_executed(env: &Env, executor: Address, proposal_id: u64, success: bool) {
        let mut data = Vec::new(env);
        data.push_back(proposal_id.into_val(env));
        data.push_back(success.into_val(env));

        let mut metadata = Map::new(env);
        metadata.set(Self::PROPOSAL_ID_KEY, Vec::from_array(env, [proposal_id.into_val(env)]));

        Self::emit_standard(env, topics::PROPOSAL_EXECUTED, Some(executor), data, metadata);
        
        // Also emit legacy event for backward compatibility
        env.events().publish(
            (topics::PROPOSAL_EXECUTED, executor),
            (proposal_id, success, env.ledger().timestamp()),
        );
    }
}

/// Event schema versioning utilities
pub struct EventSchema;

impl EventSchema {
    /// Get current schema version
    pub fn current_version() -> u32 {
        EventEmitter::CURRENT_VERSION
    }

    /// Validate event schema compatibility
    pub fn is_compatible(version: u32) -> bool {
        version <= Self::current_version()
    }
}
