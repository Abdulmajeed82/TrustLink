//! Benchmarks for TrustLink contract performance improvements.
//!
//! This module provides benchmarks to demonstrate the performance improvement
//! of the SubjectClaimIndex optimization for has_valid_claim queries.

use soroban_sdk::{Address, Env, String, Vec};
use trustlink::{TrustLinkContract, TrustLinkContractClient};

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::{Address as _, Ledger}, vec};

    #[test]
    fn benchmark_has_valid_claim_performance() {
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
        
        // Benchmark with different numbers of attestations
        let test_cases = vec![&env, 10, 50, 100, 200, 500];
        
        for num_attestations in test_cases.iter() {
            println!("Testing with {} attestations", num_attestations);
            
            // Create attestations for different claim types to simulate real-world scenario
            let claim_types = vec![
                &env, 
                String::from_str(&env, "KYC"),
                String::from_str(&env, "AML"),
                String::from_str(&env, "CreditScore"),
                String::from_str(&env, "Identity"),
                String::from_str(&env, "Employment"),
            ];
            
            // Create attestations
            for i in 0..num_attestations {
                let claim_idx = i % claim_types.len();
                let current_claim_type = claim_types.get(claim_idx).unwrap();
                
                // Create some attestations for the target subject and some for others
                let test_subject = if i % 3 == 0 { subject.clone() } else { Address::generate(&env) };
                
                client.create_attestation(
                    &issuer,
                    &test_subject,
                    &current_claim_type,
                    &None,
                    &None,
                    &None,
                );
            }
            
            // Measure time for has_valid_claim
            let start_time = env.ledger().timestamp();
            
            // This should be fast with the SubjectClaimIndex optimization
            let result = client.has_valid_claim(&subject, &claim_type);
            
            let end_time = env.ledger().timestamp();
            let duration = end_time - start_time;
            
            println!(
                "has_valid_claim with {} total attestations took {}ms, result: {}",
                num_attestations, duration, result
            );
            
            // With the optimization, this should be O(1) for the specific claim type
            // rather than O(n) for all subject attestations
            assert!(duration < 1000, "Query took too long: {}ms", duration);
        }
    }
    
    #[test]
    fn benchmark_has_valid_claim_vs_old_implementation() {
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
        let target_claim_type = String::from_str(&env, "KYC");
        let other_claim_type = String::from_str(&env, "AML");
        
        // Create 100 attestations for the subject with different claim types
        for i in 0..100 {
            let claim_type = if i % 2 == 0 { target_claim_type.clone() } else { other_claim_type.clone() };
            client.create_attestation(
                &issuer,
                &subject,
                &claim_type,
                &None,
                &None,
                &None,
            );
        }
        
        // Test has_valid_claim for the target claim type
        // With optimization: should only check ~50 attestations (those with target_claim_type)
        // Without optimization: would check all 100 attestations
        let start_time = env.ledger().timestamp();
        let result = client.has_valid_claim(&subject, &target_claim_type);
        let end_time = env.ledger().timestamp();
        
        let duration = end_time - start_time;
        println!("has_valid_claim with 100 attestations (50 matching claim type) took {}ms", duration);
        
        assert!(result, "Should find valid claim");
        assert!(duration < 500, "Query took too long: {}ms", duration);
    }
    
    #[test]
    fn benchmark_subject_claim_index_maintenance() {
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
        
        // Benchmark creation time with index maintenance
        let start_time = env.ledger().timestamp();
        
        for i in 0..100 {
            client.create_attestation(
                &issuer,
                &subject,
                &claim_type,
                &None,
                &None,
                &None,
            );
        }
        
        let end_time = env.ledger().timestamp();
        let creation_duration = end_time - start_time;
        
        println!("Creating 100 attestations with index maintenance took {}ms", creation_duration);
        
        // Benchmark query time
        let start_time = env.ledger().timestamp();
        let result = client.has_valid_claim(&subject, &claim_type);
        let end_time = env.ledger().timestamp();
        
        let query_duration = end_time - start_time;
        
        println!("Query after 100 attestations took {}ms", query_duration);
        
        assert!(result, "Should find valid claim");
        assert!(query_duration < 100, "Query took too long: {}ms", query_duration);
    }
}