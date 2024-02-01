# Low Latency High Performance Matching Engine built using Rust

## Objective

The objective of this project is to build a crypto exchange Trading Platform using Rust.

### Trading system
    
 The order book is the storage location of active orders. This includes the price, quantity, and side of the order. The requirements here include accepting orders, matching orders, cancelling orders and the communication of the current state of the market.

### Market Information

The market information system is a middle layer responsible for providing the current state of the market to the users. This includes the current minimum ask price and maximum bid, the volume of orders on the order book, and the last trade price. The requirements here include providing the current state of the market to the users. This is typically done via standardized protocol like REST API or Websocket but may utilize industry standard protocols such as FIX.

### Accounting, Onramps & Offramps

The accounting system is responsible for the management of user accounts. This includes the management of user balances, deposits, withdrawals, and the management of the exchange's own balances. This is typically done via a database and a set of APIs. The requirements are significantly less focused on performance and more focused on security, reliability and regulatory compliance.

## Online Systems Architectural Requirements (WIP)

The architecture of these systems is typically modular with each using a set of microservices. The general requirements of each system is as follows:

### Matching Engines

Each trading pair should have its own instance of the matching engine that is updatable, scalable, secure and reliable.

### Risk Management

The risk management system is a service that is responsible for the management of the risk for exchange.

### Market Information

The market information system is a service that is responsible for the communication of the current state of the market. This should have no impact on the performance of the matching engine nor should it be impacted by the performance of the matching engine. The market information system should be able to communicate the current state of the market to a large number of users. It should be able to do this in a secure, reliable and scalable manner with no access to user information and read-only access to the matching engine data or access to a data store that the matching engine can only write or update but not read from. It will also require its own authentication mechanism. The communication should be done via a standardized protocol such as REST API or Websocket but may utilize industry standard protocols such as FIX. 

### Accounting, Onramps & Offramps

The accounting system is may comprise two or more services that are responsible for the management of user accounts, the exchange accounts and other types of accounts (e.g. third party investment, staking). Security is of primary interest as this is the system that interfaces with external services. The accounting system should be able to communicate with the matching engine to update user's available trading balances. The withdrawl of funds from the trading accounts into non-trading accounts may need to be unidirectional from the trading account.  The accounting system also needs the ability to update the exchange's own balances although this will likely be a separate system.

The accounting system could also be seen to include surveillance and compliance systems. These systems are responsible for the monitoring of the exchange for suspicious activity and the reporting of this activity to the appropriate regulatory bodies. This is a requirement for any exchange that is operating in a regulated jurisdiction.

### Data Warehousing

All of these systems require data storage of different types. This includes the order book, user balances, user information, market information, and many other data. The data warehouse should be separated by concern but needs to be able to communicate with operations and be monitored.

## External Entities (WIP)

### Users

### Regulators

### Auditors

### Institutions & Exchanges

### Liquidity Providers

### Market Makers

### Custodians

### Data Providers

### Data Consumers