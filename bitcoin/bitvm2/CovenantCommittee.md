![Fig-6](./fig-6.png)

How to protect UTXOs through presigning of covenant committee at setup time? The security is ensured by the 1-out-n honesty of covenant committee, means that at least one of the covenant committee members must abandon the `vk` after presigning. Ensuring that new covenant committee signatures for a specific *PegIn* instance will not come out suddenly sometime in the future at runtime (after setup time).

# PegIn

- Spender must unlock this UTXO through `CheckCovenant` script which includes the `PubKey` of covenant committee.
    - In this case, spender must provide a valid signature from covenant committee. 
    - This signature must be valid one, since `OP_CHECKSIGVERIFY` will check whether it is consistent with its preimage (`CheckCovenant` script + specific parts of Transaction body depends on `HashType`).

- Spender must use a valid transaction which points to this UTXO. 

In summary, covenant committee may have presigned many *PayoutOptimistic* transactions to spend the *PegIn* UTXO for each operator. In this case, only one of them is qualified to spend this UTXO, any other transactions are blocked due to above two constraints.

# Claim

## First UTXO

- Spender must unlock this UTXO through `RelTimelock` script which specified a fixed block number.
    - In this case, the spending transaction can be anyone which includes `RelTimelock` script and points to this UTXO.

-  This spending transaction must be presigned by covenant committee, including the `RelTimelock` script.


## Second UTXO

# Assert