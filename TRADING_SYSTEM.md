# Low Latency High Performance Matching Engine built using Rust

## Objective

The objective of this project is to build a centralized crypto exchange Trading Platform using Rust. This is experimental project for educational purposes, not intended to become a production system.

Each component of the system will be built as a separate crate.

## Introduction

The first step in creating a complex system is to understand the requirements of the system. The requirements of a trading system can be broken down into several categories. These categories include the trading system, market information, accounting, onramps & offramps, and risk management. Each of these systems has its own set of requirements and each of these systems has its own set of architectural requirements.

## Architectural Requirements

### Trading system
    
A trading system in an exchange, encompassing the matching engine and the order book, is the core of the operational infrastructure designed to facilitate the buying and selling of financial instruments, such as stocks, bonds, derivatives, commodities, and currencies. A breakdown of its major components:

1. **Matching Engine**: The core component of a trading system, the matching engine is responsible for the execution of orders submitted by market participants. It operates by matching buy and sell orders based on predefined criteria, such as price and time priority. The matching engine processes orders in real-time or near-real-time, ensuring that trades are executed fairly and efficiently. Its primary functions include order matching, trade confirmation, and maintaining the integrity and transparency of the trading process.

2. **Order Book**: The order book is a digital storage system that records all active buy and sell orders for a particular financial instrument on the exchange. Orders are typically organized by price level, with buy orders arranged in descending order of price and sell orders in ascending order. The order book provides a real-time snapshot of market demand and supply for an asset, showing the depth of the market, which includes information on the number of shares or contracts available at different price levels. It is essential for market participants, as it helps them make informed trading decisions by assessing market liquidity, depth, and potential price movements.

The trading system is should ensure a fair, transparent, and efficient market. It supports various types of orders, such as market orders, limit orders, stop orders, and others, each with specific instructions on how they should be executed. The system's performance, including its speed, reliability, and capacity to handle high volumes of orders without significant delays or disruptions, is crucial for maintaining market integrity and trader confidence.

Moreover, the trading system is subject to rigorous regulatory standards to protect investors, prevent market manipulation, and ensure compliance with trading rules. This involves continuous monitoring, auditing, and updating of the system to address emerging challenges and technological advancements in the financial markets so the design must embrace the flexibility this requires.

### Market Information

The Market Information System within an exchange serves as a critical intermediary layer, tasked with aggregating, processing, and disseminating vital market data to participants in real-time or near-real-time. This system plays a pivotal role in ensuring transparency, facilitating informed decision-making, and maintaining a fair and efficient marketplace. Here’s an expanded definition incorporating its components, functionality, and significance:

1. **Data Aggregation and Processing**: The Market Information System collects data from various sources within the exchange, primarily from the trading system's matching engine and order book. This includes detailed information on executed trades, current open orders (buy and sell), order modifications, cancellations, and the real-time status of financial instruments being traded. The system processes this raw data to generate meaningful market insights, such as the current minimum ask price, maximum bid price, volume of orders at different price levels, the last traded price, and historical trade data.

2. **Real-Time Market Data Feeds**: At the heart of the system is its ability to provide a continuous stream of market data to users. This includes real-time quotes for securities, market depth information showing the volume of orders at various price levels, and transaction reports detailing executed trades. The immediacy of this information is crucial for traders and investors to make timely and informed decisions.

3. **Data Dissemination Methods**: To ensure broad accessibility and usability of market information, the system employs standardized protocols for data dissemination. REST APIs (Representational State Transfer Application Programming Interfaces) and WebSockets are commonly used for delivering data over the internet, offering users flexibility in accessing real-time or historical market data. The system may also support industry-standard protocols such as the Financial Information eXchange (FIX) protocol, which is widely used for real-time electronic exchange of securities transactions and market data.

4. **Data Visualization and Analytics Tools**: Beyond raw data feeds, the Market Information System often integrates advanced data visualization and analytics tools. These tools help users interpret market trends, analyze trading volumes and price movements, and visualize market sentiment. They can range from simple charting tools to sophisticated analytical platforms offering complex indicators and predictive modeling capabilities.

5. **Compliance and Security**: Ensuring the integrity, reliability, and security of market data is paramount. The system incorporates mechanisms to prevent unauthorized access, data manipulation, and other security threats. Compliance with regulatory standards is also critical, necessitating features for audit trails, data accuracy checks, and mechanisms to prevent the dissemination of sensitive information.

6. **User Accessibility and Customization**: Catering to a diverse user base, from individual traders to large institutional investors, the Market Information System offers customizable data feeds and interfaces. Users can select specific data points, customize their data streams to match their trading strategies, and integrate market data into their own trading platforms or applications.

7. **Latency and Scalability**: With traders relying on split-second timing, the system is optimized for low latency, ensuring that data is delivered as quickly as possible. Scalability is also crucial, as the system must handle peak trading volumes and surges in data requests without degradation in performance.

### Accounts, Onramps & Offramps

Accounts Management for an exchange is a highly complex system incorporating multifaceted functionality and playing a critical role in ensuring security, compliance, and operational efficiency:

### Comprehensive Account Management System
- **Multi-Service Architecture**: The system encompasses a diverse range of services responsible for the management of user accounts, exchange accounts, and extended services such as third-party investment and staking accounts. This multi-service approach facilitates a wide spectrum of financial activities, enhancing the exchange's value proposition to users.
- **Security and Authentication**: A cornerstone of the system, implementing state-of-the-art security protocols, including multi-factor authentication (MFA) and encryption, to safeguard user data and financial assets. The system is designed to prevent unauthorized access and ensure the confidentiality and integrity of all transactions.
- **Real-Time Balance Updates and Fund Flows**: Integrates closely with the exchange's matching engine to provide immediate updates to users' available trading balances, reflecting the dynamic nature of trading activities. The architecture supports unidirectional fund transfers from trading to non-trading accounts, enforcing a controlled flow of funds to enhance security and meet regulatory standards.

### Robust Onramps and Offramps
- **Seamless External Integration**: Features comprehensive interfaces for connecting with banking systems, payment gateways, and various blockchain networks, facilitating smooth and efficient deposits and withdrawals. Supports multiple fiat currencies and cryptocurrencies, ensuring broad market accessibility and liquidity.
- **Liquidity and Balance Management**: Incorporates mechanisms for managing the exchange's own balances across different assets and currencies, crucial for maintaining operational liquidity and facilitating user transactions without delays.

### Regulatory Compliance and Surveillance
- **Integrated Compliance Framework**: Embedded surveillance and compliance systems monitor the exchange for suspicious activities, aligning with regulatory requirements in real-time. This includes automated reporting to regulatory bodies and adherence to KYC/AML standards, crucial for operating within regulated jurisdictions.
- **Data Privacy and Protection**: Adheres to strict data privacy regulations, ensuring that user information is collected, stored, and processed securely, with mechanisms in place for data accuracy and integrity.

### Performance, Scalability, and User Experience
- **High Availability and Low Latency**: Designed for high availability and low-latency operations to ensure that user transactions are processed efficiently, reflecting the fast-paced nature of financial markets.
- **Scalability**: Capable of scaling horizontally to manage increases in user numbers and transaction volumes, maintaining performance without compromising on service quality.
- **User-Centric Interfaces**: Offers intuitive user interfaces for account management, including dashboards for viewing balances, transaction histories, and performing deposits and withdrawals, enriched with notifications and alerts to keep users informed.

This architectural presents a holistic view of the Accounts Management system. By integrating direct interactions with the trading system, ensuring regulatory compliance through integrated surveillance, and supporting a wide range of financial activities, the system is positioned as a foundational pillar of the exchange's infrastructure, essential for its operational integrity, user trust, and market competitiveness.

### Risk Management

Risk management for an exchange involves the identification, assessment, and prioritization of risks followed by coordinated and economical application of resources to minimize, monitor, and control the probability or impact of unforeseen events or to maximize the realization of opportunities. In the context of an exchange, which can be a platform for trading financial instruments like stocks, bonds, currencies, derivatives, commodities, etc., risk management takes on several specific forms:

1. **Market Risk Management**: Involves managing risks related to fluctuations in market prices, interest rates, currency exchange rates, and commodity prices. The exchange might use hedging strategies and set position limits to mitigate these risks.

2. **Credit Risk Management**: Addresses the risk of loss arising from a counterparty failing to fulfill its financial obligations. The exchange ensures that mechanisms like margin requirements, collateral management, and counterparty risk assessments are in place.

3. **Liquidity Risk Management**: Ensures that the exchange can meet its short-term obligations without incurring unacceptable losses. This involves monitoring the liquidity of instruments traded on the exchange and having policies for stress testing and liquidity buffers.

4. **Operational Risk Management**: Involves managing risks associated with exchange operations, including system failures, fraud, human errors, and external events. Strategies include robust IT systems, internal controls, and business continuity plans.

5. **Legal and Compliance Risk Management**: Focuses on ensuring that the exchange operates within the legal framework and complies with regulations. This includes managing risks related to regulatory changes, legal disputes, and compliance with trading rules.

6. **Reputation Risk Management**: Deals with the risk of damage to the exchange’s reputation, which can affect its credibility and the confidence of market participants. Effective communication, transparent operations, and ethical conduct are key to managing this risk.

7. **Strategic Risk Management**: Involves identifying and managing risks that could impact the exchange’s long-term objectives and strategy, including technological changes, competitive pressures, and shifts in market demand.

An exchange's risk management framework typically includes policies, procedures, and tools for risk measurement and reporting, as well as governance structures to ensure that risk management processes are integrated into the organization's overall strategy and operations.

### Data Warehousing

All of these systems require data storage of different types. This includes the order book, user balances, user information, market information, and many other data. The data warehouse should be separated by concern but needs to be able to communicate with operations and be monitored.

## External Entities (WIP)

- Users
- Regulators
- Auditors
- Institutions & Exchanges
- Liquidity Providers
- Market Makers
- Custodians
- Data Providers
- Data Consumers
- Administrators
- Engineers