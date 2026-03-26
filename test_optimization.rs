//! Simple test to verify SubjectClaimIndex optimization works correctly.
//!
//! This test verifies that the has_valid_claim function works correctly
//! with the new SubjectClaimIndex optimization.

use soroban_sdk::{Address, Env, String, Vec};
use trustlink::{TrustLinkContract, TrustLinkContractClient};

#[test]
fn test_subject_claim_index_optimization() {
    let env = Env::default();
    env.mock_all_auths();
    
    // Set up contract
    let admin = Address::generate(&env);
    let contract_id = env.register_contract(None, TrustLinkContract);
    let client = TrustLinkContractClient::new(&env, &contract_id);
    
    client.initialize(&admin, &Some(30));
    
    // Register issuer
    let issuer = Address::generate(&env);
    client.register_issuer(&admin, &issuer);
    
    // Test subject
    let subject = Address::generate(&env);
    let claim_type = String::from_str(&env, "KYC");
    let other_claim_type = String::from_str(&env, "AML");
    
    // Create multiple attestations for the subject with different claim types
    for i in 0..10 {
        let current_claim_type = if i % 2 == 0 { claim_type.clone() } else { other_claim_type.clone() };
        client.create_attestation(
            &issuer,
            &subject,
            &current_claim_type,
            &None,
            &None,
            &None,
        );
    }
    
    // Test has_valid_claim for the target claim type
    // This should work correctly with the SubjectClaimIndex optimization
    let result = client.has_valid_claim(&subject, &claim_type);
    assert!(result, "Should find valid claim for KYC");
    
    // Test has_valid_claim for the other claim type
    let result2 = client.has_valid_claim(&subject, &other_claim_type);
    assert!(result2, "Should find valid claim for AML");
    
    // Test has_valid_claim for a non-existent claim type
    let non_existent_claim = String::from_str(&env, "NON_EXISTENT");
    let result3 = client.has_valid_claim(&subject, &non_existent_claim);
    assert!(!result3, "Should not find claim for non-existent claim type");
    
    println!("SubjectClaimIndex optimization test passed!");
}

#[test]
fn test_subject_claim_index_maintenance() {
    let env = Env::default();
    env.mock_all_auths();
    
    // Set up contract
    let admin = Address::generate(&env);
    let contract_id = env.register_contract(None, TrustLinkContract);
    let client = TrustLinkContractClient::new(&env, &contract_id);
    
    client.initialize(&admin, &Some(30));
    
    // Register issuer
    let issuer = Address::generate(&env);
    client.register_issuer(&admin, &issuer);
    
    // Test subject
    let subject = Address::generate(&env);
    let claim_type = String::from_str(&env, "KYC");
    
    // Create an attestation
    let attestation_id = client.create_attestation(
        &issuer,
        &subject,
        &claim_type,
        &None,
        &None,
        &None,
    );
    
    // Verify has_valid_claim works
    assert!(client.has_valid_claim(&subject, &claim_type));
    
    // Revoke the attestation
    client.revoke_attestation(&issuer, &attestation_id, &None);
    
    // Verify has_valid_claim no longer finds it
    assert!(!client.has_valid_claim(&subject, &claim_type));
    
    println!("SubjectClaimIndex maintenance test passed!");
}

fn main() {
    test_subject_claim_index_optimization();
    test_subject_claim_index_maintenance();
    println!("All tests passed!");
}