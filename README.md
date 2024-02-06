
[<img src="https://github.com/soar-robotics/soarchain-insurance-smart-contracts/blob/main/resources/8IONDt.png?raw=true" width="250">](https://www.soarchain.com/)

-----------------------

## Overview

This repository is dedicated to showcasing the revolutionary capabilities of smart contracts, meticulously crafted for the realm of vehicle insurance. With a primary emphasis on usage-based policies, this resource provides an exploration of cutting-edge approaches, bridging the gap between traditional practices and the unleashed potential of Usage-Based Insurance (UBI) Smart Contracts. Using smart contracts empower companies and business owners in embracing innovative solutions that redefine the landscape of insurance services.

The templates provided in this repository serve as a comprehensive toolkit for implementing your business logic seamlessly. We have prioritized simplicity in deploying, initiating, creating, withdrawing, renewing, and terminating policies to facilitate a smooth and efficient process.

This repository is designed to cover the entire lifecycle of insurance policies, including
**Deploying**, **Initiating**, **Creating Policies**, **Renewing**, and **Terminating** them. It provides thorough documentation and explanations for each transaction, catering to both insurance companies and self-creation business developers.

### Insurance Workflow
##
The workflow of issuance and renewal of vehicle insurance based on a blockchain network involves multiple steps to ensure transparency, security, and efficiency. Below is a general outline of the 3 workflow:
##
**User Registration**

Participants, including insurance companies, insured parties, and various stakeholders, begin their journey on the blockchain network by creating accounts and securing cryptographic key pairs.

**Prerequisites**: 
For insured parties operating as motus drivers, a seamless integration into the soarchain network is essential. This involves a series of steps that each insured party should follow:
* Purchase a Motus Device (Motus mini) - For further details, please visit [Soarchain Shop](https://shop.soarchain.com/)
* Complete registration by installing the Soarchai mobile app - Additional information is available at [Soarchain Official Website](https://www.soarchain.com/)

**Smart Contract Deployment**

Insurance companies deploy smart contracts on the blockchain to manage insurance policies. These smart contracts define the rules, terms, and conditions of insurance policies.

**Issuance of Insurance Policy**

When an insurance company needs to initiate a new insurance policy for a vehicle owner, they seamlessly interact with the smart contract. The smart contract, in turn, meticulously records vital policy details, including coverage specifics, premium amounts, and the duration of the policy.

**Payment and Premium Collection**

The insured party makes a payment using cryptocurrency, and the smart contract verifies the payment. Once payment is confirmed, the smart contract updates the policy status to active.

**Renewal Request**

The insured party initiates a renewal request by interacting with the smart contract. The smart contract may calculate a new premium based on various factors, such as the vehicle's history, market conditions, and any changes in coverage.

**Premium Adjustment and Payment**

If there are changes in the premium amount, the smart contract calculates the adjusted premium. The insured party makes the payment, and the smart contract verifies the transaction.

**Policy Renewal Confirmation**

After successful payment, the smart contract updates the policy details, extending the policy duration. The renewed policy is now active.

**Termination or Cancellation**

The smart contract allows for termination or cancellation based on predefined conditions. For example, if an insured party fails to make payments or violates terms, the smart contract may terminate the policy.

**Transparency and Auditability**

All transactions and policy-related activities are recorded on the blockchain, providing transparency and auditability. Participants can verify the history of a policy or transaction at any time.

**Smart Contract Upgrades**

Insurance companies may upgrade smart contracts to introduce new features, improve efficiency, or comply with regulatory changes.

It's essential to note that the specific steps and features of the workflow can vary based on the blockchain platform, smart contract language, and regulatory requirements. Additionally, security considerations, privacy, and compliance with local laws should be carefully addressed during the development and deployment of insurance-related smart contracts.


-----------------------


## Introduction to Technical Aspects

This guide is meticulously crafted to empower you with the essential know-how and detailed instructions to seamlessly deploy and manage cutting-edge insurance smart contracts.

Throughout this comprehensive guide, you'll gain the expertise and hands-on skills needed to craft robust insurance smart contracts that align with the dynamic demands of the insurance landscape. Join us on this exciting journey of innovation and transformation, and let's revolutionize the future of insurance together!

## Configuring Your Development Environment

Embark on your smart contract development journey by configuring a development environment tailored for optimal efficiency. Our step-by-step guide ensures a hassle-free setup, covering essential tools, platforms, and dependencies.
For detailed instructions, refer to our documentation website [here](https://docs.soarchain.com/smart-contracts/before-Starting). Get ready to dive into the world of seamless smart contract development!

## Smart Contract Demo

Experience a simulated interaction between two users, <code>Allianz (Insurance Company)</code> and <code>Bob (Insured Party)</code>, engaging with our insurance smart contract. Here's a breakdown of the process:

**Policy Creation**:

Allianz, representing the insurance company, initiates the policy creation process. Premium details are defined for Bob, who acts as the insured party.

**Payment and Activation**:

After policy creation, Bob makes a payment using cryptocurrency to activate the policy.

**Termination or Cancellation**:

The smart contract allows for termination or cancellation based on predefined conditions. Allianz can call the Terminate function if the policy is inactive and unpaid.

**Renewal**:

Bob has the option to renew the policy using the Renewal function, extending the contract duration.
This demo illustrates a seamless and dynamic interaction within the insurance smart contract, showcasing various functionalities like policy creation, payment, termination, and renewal. Step into the world of efficient and secure insurance contract management!

Here, we will walk through the happy case, where Allianzthe created a policy for Bob based on his request for Bob's already registered vehicle.

1) Implementing your actual needs into the smart contract templates ensures that the resulting system aligns perfectly with your business processes and requirements. Take advantage of the flexibility and adaptability of these templates to create a solution tailored to your specific use case.

2) Allianz grants executable permissions to all scripts

```bash
make make-scripts-executable
```

3) Allianz compiles all smart contracts

```bash
make compile-insurance
```

4) Allianz starts the local SoarChain node

```bash
make start-node
```

5) Allianz generates a cryptographic key for the insurance company named  `allianz` using soarchaind:

```bash
make add-key
```

**Note**: Bob's account has already been created, and a Motus client has been registered for him. You can verify his vehicle information and request related data from us (SoarChain) using the pre-existing queries

The account names mentioned in the scripts are utilized for testing token transactions.

6) Allianz initiates a token transfer to the designated key

```bash
make send-token
```

7) Allianz deploys the smart contract

```bash
make deploy-insurance
```

8) Allianz initiates the smart contract

```bash
make init-insurance
```

9) Allianz generates a new insurance policy

```bash
make create-policy
```

10) Bob completes the payment using cryptocurrency

```bash
make withdraw-premium
```

11) Allianz and Bob check the insurance status and details

```bash
make details-insurance
```

12) Allianz reviews Bob's registration and vehicle information

```bash
make details-motus-profile
```

You can execute the stop script at any time to halt the node

```bash
make stop-node
```

In the repository, you'll discover additional scripts designed to streamline various commands, reducing the need for extensive manual efforts. We encourage you to explore and leverage these scripts to simplify interactions with the chain and contracts. Keep your focus on advancing your business while we handle the technical intricacies.

