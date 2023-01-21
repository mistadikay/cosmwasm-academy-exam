# Bidding platform

[CosmWasm Academy](https://academy.cosmwasm.com/) final exam.

Create a smart contract for bidding procedure.

At instantiation, user opens a bid for some off-chain commodity. Bid will be happening using only single native token (e.g. `ATOM`). Contract owner is optionally provided by its creator - if missing, contract creator is considered its owner.

After contract is instantiated, any user other than the contract owner can raise their bid by sending tokens to the contract with the `bid {}` message. When the message is called, part of the tokens sent are immediately considered
bidding commission and should be transferred to contract owner. It is up to you to figure out how to calculate commission.

The total bid of the user is considered to be a sum of all bids performed minus all the commissions. When user raises their bid, it should succeed only if their total bid is the highest of all the other users bids. If it is less or the same as
the highest, bidding should fail.

Owner can `close {}` the bidding at any time. When the bidding is closed, address with the highest total bid is considered the bidding winner. The whole bidding is transferred to the contract owner.

After the bidding is closed, everyone who bid and didn't win the bidding, can `retract {}` all their funds. Additionally, the `retract {}` message should have an optional friend receiver being an address where the sender bids should be sent. So `retract {}` sends all senders bids (minus commissions) to their account. The `retract { "receiver": "addr" }` should send all the sender bids to the `"addr"` account.

Additionally - all the information kept on the contract should be queryable in reasonable manner. The most important queries are: the given addr total bid, the highest bid at the current time (who and how much), if the bidding is closed, who won the bid (if it is closed).

The contract should contain some tests using multitests framework, but I do not expect any particular coverage - 2-3 main flow tests should be enough.

## Example

- There is the bidding created at `bidding_contract` address.
- `alex` is sending `bid {}` message with `15 ATOM`.
- The highest bid right now is `15 ATOM` by `alex`.
- Now `ann` is sending `bid {}` message with `17 ATOM`.
- The highest bid is `17 ATOM` by `ann`, and total bid by `alex` is `15 ATOM`.
- Now `ann` is sending another `bid {}` message with `2 ATOM`.
- Now the highest bid is `19 ATOM` by `ann`, and total of `alex` is `15 ATOM`.
- Then `alex` sends `bid {}` message with `1 ATOM` - this message fails, as it would leave `alex` at `16 ATOM` bid total, which is not the highest right now. He has to send more than `5 ATOM`.
- `alex` sends another `bid {}` with `5 ATOM`.
- It makes the highest bid being `20 ATOM` by `alex`, and `ann` has total of `19 ATOM` bid.
- The `close {}` is sent by the contract owner - `alex` wins the bid, `20 ATOM` are send to bid owner from `bidding_contract`.
- `ann` can claim her ATOMs back calling `retract {}` message, optionally putting a receiver field there to point where funds should be sent back to.

## Hint

The `cw_storage_plus::Map<Key, Value>` utility would be a great tool to keep total bids.

## Implementation steps

- [ ] implement initiation with an optional contract owner
- [ ] add bid {} execute entry point allowing to submit a bid by anyone and stored in a contract state
- [ ] implement ability to increase a bid
- [ ] implement commission to the future bid winner when submitting a bid
- [ ] add close {} execute entry point allowing an owner to close the bid
- [ ] add retract {} execute entry point allowing to retract funds for those who didn't win the bid
- [ ] implement an optional receiver in retract {} to allow transferring funds to another person
- [ ] implement bid {} query returning current bid by address
- [ ] implement highest {} query returning the highest (winning) bid (who and how much)
- [ ] implement closed {} query returning if bidding is closed
- [ ] implement winner {} query returning the winner if the bidding is closed