//! Storage helpers for TrustLink.
//!
//! This module provides optimized storage operations for the TrustLink contract,
//! including secondary indexing for improved performance of has_valid_claim queries.

use soroban_sdk::{Address, Env, String, Vec};

use crate::types::{
    Attestation, AuditEntry, Endorsement, ExpirationHook, FeeConfig, GlobalStats, IssuerMetadata,
    IssuerStats, IssuerTier, MultiSigProposal, TtlConfig,
};

/// Storage keys for contract state.
#[derive(Clone)]
enum DataKey {
    Admin,
    Version,
    FeeConfig,
    TtlConfig,
    Paused,
    GlobalStats,
    ClaimTypeList,
    ClaimType(String),
    IssuerList,
    Issuer(Address),
    IssuerTier(Address),
    IssuerMetadata(Address),
    IssuerStats(Address),
    BridgeList,
    Bridge(Address),
    ExpirationHook(Address),
    Attestation(String),
    SubjectAttestations(Address),
    IssuerAttestations(Address),
    AuditLog(String),
    Endorsements(String),
    MultisigProposal(String),
    SubjectClaimIndex(Address, String), // New secondary index
}

impl soroban_sdk::TryFromVal<Env, DataKey> for soroban_sdk::Val {
    type Error = soroban_sdk::ConversionError;

    fn try_from_val(env: &Env, v: &DataKey) -> Result<Self, Self::Error> {
        match v {
            DataKey::Admin => Ok(soroban_sdk::Val::from_str(env, "admin")),
            DataKey::Version => Ok(soroban_sdk::Val::from_str(env, "version")),
            DataKey::FeeConfig => Ok(soroban_sdk::Val::from_str(env, "fee_config")),
            DataKey::TtlConfig => Ok(soroban_sdk::Val::from_str(env, "ttl_config")),
            DataKey::Paused => Ok(soroban_sdk::Val::from_str(env, "paused")),
            DataKey::GlobalStats => Ok(soroban_sdk::Val::from_str(env, "global_stats")),
            DataKey::ClaimTypeList => Ok(soroban_sdk::Val::from_str(env, "claim_type_list")),
            DataKey::ClaimType(s) => Ok(soroban_sdk::Val::try_from_val(env, s)?),
            DataKey::IssuerList => Ok(soroban_sdk::Val::from_str(env, "issuer_list")),
            DataKey::Issuer(a) => Ok(soroban_sdk::Val::try_from_val(env, a)?),
            DataKey::IssuerTier(a) => Ok(soroban_sdk::Val::try_from_val(env, a)?),
            DataKey::IssuerMetadata(a) => Ok(soroban_sdk::Val::try_from_val(env, a)?),
            DataKey::IssuerStats(a) => Ok(soroban_sdk::Val::try_from_val(env, a)?),
            DataKey::BridgeList => Ok(soroban_sdk::Val::from_str(env, "bridge_list")),
            DataKey::Bridge(a) => Ok(soroban_sdk::Val::try_from_val(env, a)?),
            DataKey::ExpirationHook(a) => Ok(soroban_sdk::Val::try_from_val(env, a)?),
            DataKey::Attestation(s) => Ok(soroban_sdk::Val::try_from_val(env, s)?),
            DataKey::SubjectAttestations(a) => Ok(soroban_sdk::Val::try_from_val(env, a)?),
            DataKey::IssuerAttestations(a) => Ok(soroban_sdk::Val::try_from_val(env, a)?),
            DataKey::AuditLog(s) => Ok(soroban_sdk::Val::try_from_val(env, s)?),
            DataKey::Endorsements(s) => Ok(soroban_sdk::Val::try_from_val(env, s)?),
            DataKey::MultisigProposal(s) => Ok(soroban_sdk::Val::try_from_val(env, s)?),
            DataKey::SubjectClaimIndex(a, s) => {
                let addr_val: soroban_sdk::Val = soroban_sdk::Val::try_from_val(env, a)?;
                let claim_val: soroban_sdk::Val = soroban_sdk::Val::try_from_val(env, s)?;
                Ok(soroban_sdk::Val::try_from_val(env, &(addr_val, claim_val))?)
            }
        }
    }
}

impl soroban_sdk::IntoVal<Env, soroban_sdk::Val> for DataKey {
    fn into_val(self, env: &Env) -> soroban_sdk::Val {
        soroban_sdk::TryFromVal::try_from_val(env, &self).unwrap()
    }
}

/// Pagination helper for large lists.
pub fn paginate<T>(env: &Env, items: Vec<T>, start: u32, limit: u32) -> Vec<T>
where
    T: Clone,
{
    let total = items.len();
    if start >= total {
        return Vec::new(env);
    }

    let end = if start + limit > total {
        total
    } else {
        start + limit
    };

    let mut result = Vec::new(env);
    for i in start..end {
        if let Some(item) = items.get(i) {
            result.push_back(item);
        }
    }

    result
}

/// Contract administration storage.
pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().persistent().set(&DataKey::Admin, admin);
}

pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().persistent().get(&DataKey::Admin)
}

pub fn has_admin(env: &Env) -> bool {
    env.storage().persistent().has(&DataKey::Admin)
}

pub fn set_version(env: &Env, version: &String) {
    env.storage().persistent().set(&DataKey::Version, version);
}

pub fn get_version(env: &Env) -> Option<String> {
    env.storage().persistent().get(&DataKey::Version)
}

/// Fee configuration storage.
pub fn set_fee_config(env: &Env, config: &FeeConfig) {
    env.storage().persistent().set(&DataKey::FeeConfig, config);
}

pub fn get_fee_config(env: &Env) -> Option<FeeConfig> {
    env.storage().persistent().get(&DataKey::FeeConfig)
}

/// TTL configuration storage.
pub fn set_ttl_config(env: &Env, config: &TtlConfig) {
    env.storage().persistent().set(&DataKey::TtlConfig, config);
}

pub fn get_ttl_config(env: &Env) -> Option<TtlConfig> {
    env.storage().persistent().get(&DataKey::TtlConfig)
}

/// Contract pause state storage.
pub fn set_paused(env: &Env, paused: bool) {
    env.storage().persistent().set(&DataKey::Paused, &paused);
}

pub fn is_paused(env: &Env) -> bool {
    env.storage().persistent().get(&DataKey::Paused).unwrap_or(false)
}

/// Global statistics storage.
pub fn get_global_stats(env: &Env) -> GlobalStats {
    env.storage()
        .persistent()
        .get(&DataKey::GlobalStats)
        .unwrap_or(GlobalStats {
            total_attestations: 0,
            total_revocations: 0,
            total_issuers: 0,
        })
}

pub fn increment_total_attestations(env: &Env, count: u64) {
    let mut stats = get_global_stats(env);
    stats.total_attestations += count;
    env.storage().persistent().set(&DataKey::GlobalStats, &stats);
}

pub fn increment_total_revocations(env: &Env, count: u64) {
    let mut stats = get_global_stats(env);
    stats.total_revocations += count;
    env.storage().persistent().set(&DataKey::GlobalStats, &stats);
}

pub fn increment_total_issuers(env: &Env) {
    let mut stats = get_global_stats(env);
    stats.total_issuers += 1;
    env.storage().persistent().set(&DataKey::GlobalStats, &stats);
}

pub fn decrement_total_issuers(env: &Env) {
    let mut stats = get_global_stats(env);
    if stats.total_issuers > 0 {
        stats.total_issuers -= 1;
        env.storage().persistent().set(&DataKey::GlobalStats, &stats);
    }
}

/// Claim type registry storage.
pub fn set_claim_type(env: &Env, info: &crate::types::ClaimTypeInfo) {
    env.storage()
        .persistent()
        .set(&DataKey::ClaimType(info.claim_type.clone()), info);
    
    // Add to claim type list if not already present
    let mut claim_types = get_claim_type_list(env);
    let mut found = false;
    for existing in claim_types.iter() {
        if existing == info.claim_type {
            found = true;
            break;
        }
    }
    if !found {
        claim_types.push_back(info.claim_type.clone());
        env.storage().persistent().set(&DataKey::ClaimTypeList, &claim_types);
    }
}

pub fn get_claim_type(env: &Env, claim_type: &String) -> Option<crate::types::ClaimTypeInfo> {
    env.storage().persistent().get(&DataKey::ClaimType(claim_type.clone()))
}

pub fn get_claim_type_list(env: &Env) -> Vec<String> {
    env.storage()
        .persistent()
        .get(&DataKey::ClaimTypeList)
        .unwrap_or_else(|| Vec::new(env))
}

/// Issuer registry storage.
pub fn add_issuer(env: &Env, issuer: &Address) {
    env.storage().persistent().set(&DataKey::Issuer(issuer.clone()), &true);
    
    // Add to issuer list if not already present
    let mut issuers = get_issuer_list(env);
    let mut found = false;
    for existing in issuers.iter() {
        if existing == *issuer {
            found = true;
            break;
        }
    }
    if !found {
        issuers.push_back(issuer.clone());
        env.storage().persistent().set(&DataKey::IssuerList, &issuers);
    }
}

pub fn remove_issuer(env: &Env, issuer: &Address) {
    env.storage().persistent().remove(&DataKey::Issuer(issuer.clone()));
    
    // Remove from issuer list
    let mut issuers = get_issuer_list(env);
    let mut new_issuers = Vec::new(env);
    for existing in issuers.iter() {
        if existing != *issuer {
            new_issuers.push_back(existing);
        }
    }
    env.storage().persistent().set(&DataKey::IssuerList, &new_issuers);
}

pub fn is_issuer(env: &Env, issuer: &Address) -> bool {
    env.storage().persistent().get(&DataKey::Issuer(issuer.clone())).unwrap_or(false)
}

pub fn get_issuer_list(env: &Env) -> Vec<Address> {
    env.storage()
        .persistent()
        .get(&DataKey::IssuerList)
        .unwrap_or_else(|| Vec::new(env))
}

/// Issuer tier storage.
pub fn set_issuer_tier(env: &Env, issuer: &Address, tier: &IssuerTier) {
    env.storage().persistent().set(&DataKey::IssuerTier(issuer.clone()), tier);
}

pub fn get_issuer_tier(env: &Env, issuer: &Address) -> Option<IssuerTier> {
    env.storage().persistent().get(&DataKey::IssuerTier(issuer.clone()))
}

/// Issuer metadata storage.
pub fn set_issuer_metadata(env: &Env, issuer: &Address, metadata: &IssuerMetadata) {
    env.storage().persistent().set(&DataKey::IssuerMetadata(issuer.clone()), metadata);
}

pub fn get_issuer_metadata(env: &Env, issuer: &Address) -> Option<IssuerMetadata> {
    env.storage().persistent().get(&DataKey::IssuerMetadata(issuer.clone()))
}

/// Issuer statistics storage.
pub fn set_issuer_stats(env: &Env, issuer: &Address, stats: &IssuerStats) {
    env.storage().persistent().set(&DataKey::IssuerStats(issuer.clone()), stats);
}

pub fn get_issuer_stats(env: &Env, issuer: &Address) -> IssuerStats {
    env.storage()
        .persistent()
        .get(&DataKey::IssuerStats(issuer.clone()))
        .unwrap_or(IssuerStats { total_issued: 0 })
}

/// Bridge registry storage.
pub fn add_bridge(env: &Env, bridge: &Address) {
    env.storage().persistent().set(&DataKey::Bridge(bridge.clone()), &true);
}

pub fn is_bridge(env: &Env, bridge: &Address) -> bool {
    env.storage().persistent().get(&DataKey::Bridge(bridge.clone())).unwrap_or(false)
}

/// Expiration hook storage.
pub fn set_expiration_hook(env: &Env, subject: &Address, hook: &ExpirationHook) {
    env.storage().persistent().set(&DataKey::ExpirationHook(subject.clone()), hook);
}

pub fn get_expiration_hook(env: &Env, subject: &Address) -> Option<ExpirationHook> {
    env.storage().persistent().get(&DataKey::ExpirationHook(subject.clone()))
}

/// Attestation storage.
pub fn set_attestation(env: &Env, attestation: &Attestation) {
    env.storage().persistent().set(&DataKey::Attestation(attestation.id.clone()), attestation);
}

pub fn get_attestation(env: &Env, attestation_id: &String) -> Result<Attestation, soroban_sdk::Error> {
    env.storage()
        .persistent()
        .get(&DataKey::Attestation(attestation_id.clone()))
        .ok_or(soroban_sdk::Error::from_contract_error(1000))
}

pub fn has_attestation(env: &Env, attestation_id: &String) -> bool {
    env.storage().persistent().has(&DataKey::Attestation(attestation_id.clone()))
}

/// Subject attestations index.
pub fn add_subject_attestation(env: &Env, subject: &Address, attestation_id: &String) {
    let mut attestations = get_subject_attestations(env, subject);
    attestations.push_back(attestation_id.clone());
    env.storage().persistent().set(&DataKey::SubjectAttestations(subject.clone()), &attestations);
}

pub fn remove_subject_attestation(env: &Env, subject: &Address, attestation_id: &String) {
    let mut attestations = get_subject_attestations(env, subject);
    let mut new_attestations = Vec::new(env);
    for existing in attestations.iter() {
        if existing != *attestation_id {
            new_attestations.push_back(existing);
        }
    }
    env.storage().persistent().set(&DataKey::SubjectAttestations(subject.clone()), &new_attestations);
}

pub fn get_subject_attestations(env: &Env, subject: &Address) -> Vec<String> {
    env.storage()
        .persistent()
        .get(&DataKey::SubjectAttestations(subject.clone()))
        .unwrap_or_else(|| Vec::new(env))
}

/// Issuer attestations index.
pub fn add_issuer_attestation(env: &Env, issuer: &Address, attestation_id: &String) {
    let mut attestations = get_issuer_attestations(env, issuer);
    attestations.push_back(attestation_id.clone());
    env.storage().persistent().set(&DataKey::IssuerAttestations(issuer.clone()), &attestations);
}

pub fn get_issuer_attestations(env: &Env, issuer: &Address) -> Vec<String> {
    env.storage()
        .persistent()
        .get(&DataKey::IssuerAttestations(issuer.clone()))
        .unwrap_or_else(|| Vec::new(env))
}

/// Subject claim index (NEW OPTIMIZATION).
/// Maps (subject, claim_type) to a list of attestation IDs for that specific claim type.
pub fn add_subject_claim_attestation(env: &Env, subject: &Address, claim_type: &String, attestation_id: &String) {
    let key = DataKey::SubjectClaimIndex(subject.clone(), claim_type.clone());
    let mut attestations = env.storage().persistent().get(&key).unwrap_or_else(|| Vec::new(env));
    attestations.push_back(attestation_id.clone());
    env.storage().persistent().set(&key, &attestations);
}

pub fn remove_subject_claim_attestation(env: &Env, subject: &Address, claim_type: &String, attestation_id: &String) {
    let key = DataKey::SubjectClaimIndex(subject.clone(), claim_type.clone());
    let mut attestations = env.storage().persistent().get(&key).unwrap_or_else(|| Vec::new(env));
    let mut new_attestations = Vec::new(env);
    for existing in attestations.iter() {
        if existing != *attestation_id {
            new_attestations.push_back(existing);
        }
    }
    env.storage().persistent().set(&key, &new_attestations);
}

pub fn get_subject_claim_attestations(env: &Env, subject: &Address, claim_type: &String) -> Vec<String> {
    env.storage()
        .persistent()
        .get(&DataKey::SubjectClaimIndex(subject.clone(), claim_type.clone()))
        .unwrap_or_else(|| Vec::new(env))
}

/// Audit log storage.
pub fn append_audit_entry(env: &Env, attestation_id: &String, entry: &AuditEntry) {
    let mut log = get_audit_log(env, attestation_id);
    log.push_back(entry.clone());
    env.storage().persistent().set(&DataKey::AuditLog(attestation_id.clone()), &log);
}

pub fn get_audit_log(env: &Env, attestation_id: &String) -> Vec<AuditEntry> {
    env.storage()
        .persistent()
        .get(&DataKey::AuditLog(attestation_id.clone()))
        .unwrap_or_else(|| Vec::new(env))
}

/// Endorsement storage.
pub fn add_endorsement(env: &Env, endorsement: &Endorsement) {
    let mut endorsements = get_endorsements(env, &endorsement.attestation_id);
    endorsements.push_back(endorsement.clone());
    env.storage().persistent().set(&DataKey::Endorsements(endorsement.attestation_id.clone()), &endorsements);
}

pub fn get_endorsements(env: &Env, attestation_id: &String) -> Vec<Endorsement> {
    env.storage()
        .persistent()
        .get(&DataKey::Endorsements(attestation_id.clone()))
        .unwrap_or_else(|| Vec::new(env))
}

/// Multi-sig proposal storage.
pub fn set_multisig_proposal(env: &Env, proposal: &MultiSigProposal) {
    env.storage().persistent().set(&DataKey::MultisigProposal(proposal.id.clone()), proposal);
}

pub fn get_multisig_proposal(env: &Env, proposal_id: &String) -> Result<MultiSigProposal, soroban_sdk::Error> {
    env.storage()
        .persistent()
        .get(&DataKey::MultisigProposal(proposal_id.clone()))
        .ok_or(soroban_sdk::Error::from_contract_error(1001))
}
