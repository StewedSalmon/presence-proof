# PresenceProof

## Problem
Instructors and students suffer from easily forged paper sign-in sheets, lost LMS data, and end-of-semester disputes over required participation grades.

## Solution
PresenceProof uses a Stellar smart contract to permanently log cryptographically verifiable, time-stamped attendance records that cannot be retroactively altered by either the student or the system administrator.

## Project Description
PresenceProof is a decentralized application (dApp) designed to securely and immutably track classroom or event attendance using the Stellar blockchain.
At its core, the project eliminates the vulnerabilities of traditional attendance tracking—like buddy sign-ins, forged paper sheets, or manipulated database records—by replacing them with cryptographically signed, timestamped transactions on a public ledger.
## Why Stellar
Stellar's 5-second finality and sub-cent transaction fees via Soroban make it financially viable to log high-volume, daily attendance data for hundreds of students without network bloat.

## Target User
University lecturers, teaching assistants managing large lectures, and independent tutors tracking session hours.

## Project Vision
PresenceProof aims to eliminate the vulnerabilities of centralized attendance tracking—such as buddy sign-ins, forged paper sheets, and manipulated database records. By anchoring attendance data directly to the Stellar blockchain, we create a transparent, immutable, and cryptographically verifiable system for universities, independent tutors, and event organizers. The vision is to bridge the physical classroom to decentralized ledgers, making indisputable proof of presence as simple and secure as scanning a QR code.
## Key Features
Cryptographic Verification: Students sign attendance transactions directly with their digital wallets (e.g., Freighter), mathematically proving their identity via require_auth().
Immutable Records: Attendance is logged to Soroban's persistent state and locked in with an unforgeable Unix consensus timestamp.
Tamper-Proof Design: The smart contract logic explicitly prevents duplicate sign-ins for the same session and blocks retroactive alterations by either students or system administrators.
Ultra-Low Cost: Leverages Stellar's 5-second finality and sub-cent transaction fees to securely process entire 100-person lecture halls for pennies.
## Live Demo
- Network: Stellar Testnet
- **Contract ID**: CBVUFVSCQK4E5THRXQEMRXV3NIZUPNPHJD7E5NFS23KF7IOEZIBGYOP3
- **Transaction**: 62fa64934e4af1e6decf6a27652544d1087b48ecd09c2782d2952df53c24
## Detailed Contract Details
<img width="1061" height="881" alt="screenshot" src="https://github.com/user-attachments/assets/8d74b0fa-1a75-441c-a35f-131e09571243" />

## Future Scope
Instructor Dashboard (React): A web application designed for classroom projectors that generates the rotating QR codes containing the day's specific session_id payload.
Student Scanner App: A mobile-friendly interface allowing students to scan the projector's QR code, intercept the transaction envelope, and seamlessly sign the payload via their connected wallet.
Automated Grading: Smart contract extensions to automatically calculate participation grades based on the ratio of attended sessions to the total semester ledger.
Proof of Attendance NFTs: Minting soulbound (non-transferable) tokens directly to student wallets as digital badges for achieving perfect attendance.
## How to Run
1. Clone: git clone https://github.com/yourusername/presence-proof.git
2. Build: stellar contract build
3. Test: cargo test
4. Deploy: stellar contract deploy --wasm target/wasm32-unknown-unknown/release/presence_proof.wasm --source-account student --network testnet

## Tech Stack
- Smart Contract: Rust / Soroban SDK v20.0.0
- Wallet: Freighter
- Network: Stellar Testnet

## Author Profile
Nguyễn Hoàng Tiến | tienavk13@gmail.com | HCMUS Year 2
