# PresenceProof

## Problem
Instructors and students suffer from easily forged paper sign-in sheets, lost LMS data, and end-of-semester disputes over required participation grades.

## Solution
PresenceProof uses a Stellar smart contract to permanently log cryptographically verifiable, time-stamped attendance records that cannot be retroactively altered by either the student or the system administrator.

## Why Stellar
Stellar's 5-second finality and sub-cent transaction fees via Soroban make it financially viable to log high-volume, daily attendance data for hundreds of students without network bloat.

## Target User
University students, teaching assistants managing large lectures, and independent tutors tracking session hours.

## Live Demo
- Network: Stellar Testnet

- **Contract ID**: CBVUFVSCQK4E5THRXQEMRXV3NIZUPNPHJD7E5NFS23KF7IOEZIBGYOP3
- **Transaction**: 62fa64934e4af1e6decf6a27652544d1087b48ecd09c2782d2952df53c24

## How to Run
1. Clone: git clone https://github.com/yourusername/presence-proof.git
2. Build: cargo build --target wasm32-unknown-unknown --release
3. Test: cargo test
4. Deploy: stellar contract deploy --wasm target/wasm32-unknown-unknown/release/presence_proof.wasm --source-account student --network testnet

## Tech Stack
- Smart Contract: Rust / Soroban SDK v20.0.0
- Wallet: Freighter
- Network: Stellar Testnet

## Team
Nguyễn Hoàng Tiến | tienavk13@gmail.com | HCMUS Year 2