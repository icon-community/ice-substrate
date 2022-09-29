# Staking Rewards Smart Contract

## Abstract

This contract allows users to make deposits and to gain interest by doing so. It is
designed as a way to give users who trust the project that issued the token the
possibility of locking (staking) it for a fixed period of time and gaining more
tokens by doing so.

## Public endpoints

### The constructor

The ```new``` function acts as a constructor and is used to initialize a contract
by parametrizing it with the following values:

- ```max_deposit_value``` - The maximum value a user can send while making a deposit
- ```locking_duration``` - The duration after which the tokens and interest can be redeemed
- ```deposit_deadline``` - The deadline after which users cannot make further deposits
- ```base_interest```
- ```stakers_rate_permil```
- ```stakers_sample```
- ```liquidity_rate_permil```
- ```liquidity_sample```

The interest is dynamic and not constant. Its formula is the following:

```
interest = base_interest - num_stakers / stakers_sample * stakers_rate_permil / 1_000_000 - total_liquidity / liquidity_sample * liquidity_rate_permil / 1_000_000;
```

This formula allows early users to have a better interest than later users. That's
because the more stakers and the more liquidity it is, the interest rate diminishes.
The reason we want to incentivize early users is because they assume a greater risk.

The first user that stakes must have had a much greater trust in the project than 
the one that already sees a lot of other users that trust the project and chose to
support it by staking the tokens, with hope that the utility and value will increase
by the time the tokens are unlocked and the interest is redeemed.

It also uses samples to, for example, treat the first 100 users exactly the same when 
```stakers_sample``` is 100.

### Deposit

Deposit is the endpoint that the user must call in order to lock his tokens. The 
AccountId that calls this endpoint must not be the address itself, preventing any 
reentrancy attacks. The transferred value cannot be 0 or greater than the configured
max. Upon deposit, a ```LockBox``` is created and a new index is given. Based on the given id,
the user can either: redeem or withdraw the funds in the lock box.

### Redeem

Redeem is the endpoint that the user must call in order to unlock his initial tokens
and to also gain the reward interest. It has the same reentrancy check as deposit
endpoint and most importantly it must be called only after the ```release``` in the
```LockBox``` has passed. Redeem is done using a LockBox index and the caller must be
the owner of the box, aka same AccountId that deposited and is present as ```beneficiary```
in the struct.

### Early Withdraw

EarlyWithdraw is the endpoint that the user must call in order to immediately claim
his previously locked token. It has the same reentrancy check as the deposit endpoint
and it can be called at anytime. The most important thing is that no interest is given
when early withdrawing. A LockBox index is needed to use this endpoint and, aside from the ```release```
it has the same constraints as redeem endpoint.

### Refund

Refund is the endpoint that the ```owner``` of the contract can call in order to
make withdrawals from the contract. Be mindful that he can withdraw as much as it likes
but it is meant to be used to handle the cases where the owner accidentally sends more
tokens (that will be claimed by the users as reward interest) than he intended.
