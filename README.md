# rs-edunode-sdk
rs-edunode-sdk: Rust SDK for EduCerts by EduNode.org
This repository provides the Rust Software Development Kit (SDK) for the EduCerts, a decentralized digital certificate issuing and verification system. EduCerts is designed to be leveraged on the Soroban Smart Contract platform.

# Features
Minting: This SDK provides an easy interface to mint (or issue) EduCerts on the Soroban platform. This allows educational institutions to issue verifiable digital certificates with ease.
Verification: The SDK also provides functionalities for verifying the authenticity of issued EduCerts. This helps to ensure the credibility and verifiability of educational certificates.
Interoperability: The SDK is designed to be  compatible with the Soroban Smart Contract platform. However, we also plan to integrate other protocols that allow us to provide seamless integration and ease of use for our users.
Safety: Built with Rust's safety guarantees, the SDK ensures the security and integrity of all operations concerning EduCerts.

# Requirements
- Rust 1.54.0 or newer
- Soroban Smart Contract platform account

# Installation
To add the rs-edunode-sdk to your package, add the following to your Cargo.toml:

<code>[dependencies]
rs-edunode-sdk = "0.1.0"
</code>
Then run <code>cargo build</code> to download and compile the package.


# Usage
After adding the SDK as a dependency, import it using:
<code>
extern crate rs_edunode_sdk;
</code>

# Minting Certificates
To mint a new EduCert:

<code> let mut edunode = rs_edunode_sdk::EduNode::new(YOUR_SOROBAN_ACCOUNT);
let certificate = edunode.mint_certificate("Certificate ID", "Recipient Name", "Issuing Institution", "Course Name", "Issue Date", "Expiry Date");
</code>

# Verifying Certificates
To verify an existing EduCert:
<code>
let verification_result = edunode.verify_certificate("Certificate ID");
</code>

# Documentation
For more detailed information about the API, see the API Documentation.

# Contributions
We appreciate your contributions! Please scontact hi@edunode.org for details on how to contribute.

# License
This project is licensed under the MIT License.

# Contact
For any inquiries, please contact hi@edunode.org.

# Private Beta Testing Disclaimer
The rs-edunode-sdk is currently in its private beta testing phase.

While we have taken steps to ensure the SDK's stability and functionality, the software is provided "as is," without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and noninfringement. As this is a beta release, you may encounter bugs, errors, or inconsistencies that could interfere with its operation.

The purpose of this private beta phase is to identify and rectify any such issues before the software is officially released to the general public. During this phase, the software is intended for testing purposes only and not for production use. We appreciate your understanding and patience during this critical stage of development.

OG Technologies EU and the contributors to rs-edunode-sdk will not be held liable for any damages, loss of data, or unexpected issues that arise as a result of using the software during this beta testing phase.

We encourage users to report any bugs, feedback, or suggestions for improvement through our GitHub issue tracker or by contacting us directly at hi@edunode.org

By using the rs-edunode-sdk during its private beta testing phase, you acknowledge and accept the potential risks associated with using pre-release software.

Your involvement in the beta testing phase is invaluable to us and we appreciate your contribution to improving the rs-edunode-sdk.

OG Technologies EU © 2023
