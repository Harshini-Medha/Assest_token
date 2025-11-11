#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, Address, symbol_short};

// Structure to track asset-backed token details
#[contracttype]
#[derive(Clone)]
pub struct AssetToken {
    pub token_id: u64,
    pub asset_type: String,      // e.g., "Gold", "Real Estate", "Commodity"
    pub asset_value: u64,         // Value in base units (e.g., grams for gold)
    pub tokens_issued: u64,       // Number of tokens created
    pub owner: Address,           // Token owner
    pub verified: bool,           // Admin verification status
    pub timestamp: u64,           // Creation timestamp
}

// Structure to track platform statistics
#[contracttype]
#[derive(Clone)]
pub struct PlatformStats {
    pub total_tokens: u64,
    pub verified_tokens: u64,
    pub total_asset_value: u64,
}

// Storage keys
const TOKEN_COUNT: Symbol = symbol_short!("T_COUNT");
const PLATFORM_STATS: Symbol = symbol_short!("P_STATS");

// Mapping token_id to AssetToken
#[contracttype]
pub enum TokenBook {
    Token(u64)
}

#[contract]
pub struct AssetBackedTokenContract;

#[contractimpl]
impl AssetBackedTokenContract {
    
    // Function 1: Create a new asset-backed token
    pub fn create_token(
        env: Env, 
        owner: Address, 
        asset_type: String, 
        asset_value: u64, 
        tokens_issued: u64
    ) -> u64 {
        // Require owner authentication
        owner.require_auth();
        
        // Get and increment token count
        let mut token_count: u64 = env.storage().instance().get(&TOKEN_COUNT).unwrap_or(0);
        token_count += 1;
        
        // Get current timestamp
        let timestamp = env.ledger().timestamp();
        
        // Create new token record
        let new_token = AssetToken {
            token_id: token_count,
            asset_type: asset_type.clone(),
            asset_value,
            tokens_issued,
            owner: owner.clone(),
            verified: false,
            timestamp,
        };
        
        // Update platform statistics
        let mut stats = Self::get_platform_stats(env.clone());
        stats.total_tokens += 1;
        stats.total_asset_value += asset_value;
        
        // Store token and updated stats
        env.storage().instance().set(&TokenBook::Token(token_count), &new_token);
        env.storage().instance().set(&TOKEN_COUNT, &token_count);
        env.storage().instance().set(&PLATFORM_STATS, &stats);
        
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Asset-Backed Token Created with Token-ID: {}", token_count);
        token_count
    }
    
    // Function 2: Verify asset-backed token (Admin only)
    pub fn verify_token(env: Env, token_id: u64, admin: Address) {
        // Require admin authentication
        admin.require_auth();
        
        // Get token record
        let mut token = Self::view_token(env.clone(), token_id);
        
        // Verify token exists and is not already verified
        if token.token_id == 0 {
            log!(&env, "Token not found");
            panic!("Token does not exist");
        }
        
        if token.verified {
            log!(&env, "Token already verified");
            panic!("Token is already verified");
        }
        
        // Update verification status
        token.verified = true;
        
        // Update platform statistics
        let mut stats = Self::get_platform_stats(env.clone());
        stats.verified_tokens += 1;
        
        // Store updated token and stats
        env.storage().instance().set(&TokenBook::Token(token_id), &token);
        env.storage().instance().set(&PLATFORM_STATS, &stats);
        
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Token-ID: {} has been verified", token_id);
    }
    
    // Function 3: View specific token details
    pub fn view_token(env: Env, token_id: u64) -> AssetToken {
        let key = TokenBook::Token(token_id);
        
        env.storage().instance().get(&key).unwrap_or(AssetToken {
            token_id: 0,
            asset_type: String::from_str(&env, "Not_Found"),
            asset_value: 0,
            tokens_issued: 0,
            owner: Address::from_string(&String::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF")),
            verified: false,
            timestamp: 0,
        })
    }
    
    // Function 4: Get platform statistics
    pub fn get_platform_stats(env: Env) -> PlatformStats {
        env.storage().instance().get(&PLATFORM_STATS).unwrap_or(PlatformStats {
            total_tokens: 0,
            verified_tokens: 0,
            total_asset_value: 0,
        })
    }
}