# TrustLink SubjectClaimIndex Optimization TODO

## Step 1: Add StorageKey variant (types.rs)
- Add `SubjectClaimIndex(Address, String)` to `StorageKey` enum

## Step 2: Implement new storage functions (storage.rs)
- `get_subject_claim_attestations(env, subject: Address, claim_type: String) -> Vec<String>`
- `add_subject_claim_attestation(env, subject, claim_type, id)`
- `remove_subject_claim_attestation(env, subject, claim_type, id)` (optional for delete)

## Step 3: Update store_attestation (lib.rs)
- Call `add_subject_claim_attestation` in `store_attestation`

## Step 4: Optimize has_valid_claim (lib.rs)
- Replace `get_subject_attestations` with `get_subject_claim_attestations`

## Step 5: Update revoke_attestation (lib.rs)
- Decide: remove from index on revoke? (No - status check handles)

## Step 6: Add tests (test.rs)
- Index creation/maintenance
- has_valid_claim uses new index
- Benchmark: 100+ attestations perf improvement

## Step 7: Verify
- `cargo test`
- `cargo check`
- Manual benchmark timing

## Step 8: Git commit + PR
- Branch: `blackboxai/subject-claim-index`
- Commit changes
- `gh pr create`

**Progress: None (0/8 complete)**
