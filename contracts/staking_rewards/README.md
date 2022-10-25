# Staking Rewards Smart Contract

## Abstract

This contract allows users to make deposits and to gain rewards by doing so. It is
designed as a way to give users who trust the project that issued the token the
possibility of locking (staking) it for a fixed period of time and gaining more
tokens by doing so.

## Public endpoints

### The constructor

The ```new``` function acts as a constructor and is used to initialize a contract
by parametrizing it with the following values:

- ```max_deposit_value``` - The maximum value a user can send while making a deposit
- ```min_deposit_value``` - The minimum value a user can send while making a deposit
- ```max_total_liquidity``` - The maximum total liquidity from deposits allowed to be held in this contract
- ```max_stakers``` - The maximum unique stakers allowed to deposit at once
- ```locking_duration``` - The duration after which the tokens and interest can be redeemed
- ```deposit_deadline``` - The deadline after which users cannot make further deposits
- ```base_interest_percent_permil``` - The base interest percent and also the max interest
- ```stakers_sample``` - The size of the chunk of stakers which can make the interest change
- ```liquidity_sample``` - The size of the chunk of tokens which can make the interest change. Keep in mind that this value should contain the decimals too
- ```negative_interest_multiplier_permil``` - The multiplier for the negative interest factor in dynamic interest formula

The interest is dynamic and not constant. Its formula is the following:

```
interest = base_interest - 
    negative_interest_multiplier_permil * 
        log2_permil(1 + total_liquidity / liquidity_sample + num_stakers / stakers_sample) / 
        1_000_000;
```

This formula allows early users to have a better interest than later users. That's
because the more stakers and the more liquidity it is, the interest rate diminishes.
The reason we want to incentive early users is that they assume a greater risk.

The first user that stakes must have had a much greater trust in the project than 
the one that already sees a lot of other users that trust the project and chose to
support it by staking the tokens, with hope that the utility and value will increase
by the time the tokens are unlocked and the interest is redeemed.

It also uses samples to, for example, treat the first 100 users exactly the same when 
```stakers_sample``` is 100.

Because the formula might be confusing, we can take a look at the following example:
Say we configure base_interest_permil as 10_000_000 (10%), stakers_sample 500, liquidity_sample 10_000_000
and negative_interest_multiplier_permil 1_000_000. This effectively means that the base interest is 10%
and with every 500 new stakers and every 10_000_000 tokens staked, the interest will decrease.

For testing, some default values might be: base_interest_percent_permil 10_000_000 (10%), stakers_sample 1,
liquidity_sample 1 and negative_interest_multiplier_permil 0. This will make the interest constant of 10%.

### Deposit

Deposit is the endpoint that the user must call in order to lock his tokens. The 
AccountId that calls this endpoint must not be the address itself, preventing any 
reentrancy attacks. The transferred value cannot be 0 or greater than the configured
max. Upon deposit, a ```LockBox``` is created and added under a user mapping.

### Get Lock Boxes

Returns all the lock boxes owned by a user.

### Redeem

Redeem is the endpoint that the user must call in order to unlock his initial tokens
and to also gain the reward interest. It has the same reentrancy check as deposit
endpoint and most importantly it must be called only after the ```release``` in the
```LockBox``` has passed. The desired box to be unlocked is identified using its unique id.

### Early Withdraw

EarlyWithdraw is the endpoint that the user must call in order to immediately claim
his previously locked token. It has the same reentrancy check as the deposit endpoint
and it can be called at anytime. The most important thing is that no interest is given
when early withdrawing.

### Refund

Refund is the endpoint that the ```owner``` of the contract can call in order to
make withdrawals from the contract. Be mindful that he can withdraw as much as it likes,
but it is meant to be used to handle the cases where the owner accidentally sends more
tokens (that will be claimed by the users as reward interest) than he intended.
