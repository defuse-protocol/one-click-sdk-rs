# QuoteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dry** | **bool** | Flag indicating whether this is a dry run request. If `true`, the response will **NOT** contain the following fields: - `depositAddress` - `timeWhenInactive` - `deadline` | 
**deposit_mode** | Option<**String**> | What deposit address mode you will get in the response, most chain supports only `SIMPLE` and some(for example `stellar`) only `MEMO`: - `SIMPLE` - usual deposit with only deposit address. - `MEMO` - some chains will **REQUIRE** the `memo` together with `depositAddress` for swap to work. | [optional][default to Simple]
**swap_type** | **String** | How to interpret `amount` (and refunds) when performing the swap:  - `EXACT_INPUT` — requests the output amount for an exact input.   - If deposit is less than `amountIn`, the deposit is refunded by deadline.   - If deposit is above than `amountIn`, the swap is processed and the excess is refunded to `refundTo` address after swap is complete.  - `EXACT_OUTPUT` — requests the input amount for an exact output.   - The quote response would have two fields `minAmountIn` and `maxAmountIn`.   - If the input is above than `maxAmountIn` the swap is processed and the excess is refunded to `refundTo` address after swap is complete.   - If the input is less than  `minAmountIn`, the deposit is refunded by deadline.  - `FLEX_INPUT` — a flexible input amount that allows for partial deposits and variable amounts.   - `slippage` applies both to `amountOut` and `amountIn` and defines an acceptable range (`minAmountIn` and `minAmountOut`).   - Any amount higher than `minAmountIn` is accepted and converted to the output asset as long as `minAmountOut` is met.   - The `amountIn` can be less, as long as the 'slippage + 1%' constraint is met. If the total received by the deadline is below the lower bound, the deposit is refunded.   - If deposits exceed the upper bound, the swap is still processed | 
**slippage_tolerance** | **f64** | Slippage tolerance for the swap. This value is in basis points (1/100th of a percent), e.g. 100 for 1% slippage. | 
**origin_asset** | **String** | ID of the origin asset. | 
**deposit_type** | **String** | Type of deposit address: - `ORIGIN_CHAIN` - deposit address on the origin chain. - `INTENTS` - the account ID within NEAR Intents to which you should transfer assets. | 
**destination_asset** | **String** | ID of the destination asset. | 
**amount** | **String** | Amount to swap as the base amount. It is interpreted as the input or output amount based on the `swapType` flag and is specified in the smallest unit of the currency (e.g., wei for ETH). | 
**refund_to** | **String** | Address used for refunds. | 
**refund_type** | **String** | Type of refund address: - `ORIGIN_CHAIN` - assets are refunded to the `refundTo` address on the origin chain. - `INTENTS` - assets are refunded to the `refundTo` Intents account. | 
**recipient** | **String** | Recipient address. The format must match `recipientType`. | 
**connected_wallets** | Option<**Vec<String>**> | Addresses of connected wallets. | [optional]
**session_id** | Option<**String**> | Unique client session identifier for 1Click. | [optional]
**virtual_chain_recipient** | Option<**String**> | EVM address of a transfer recipient in a virtual chain | [optional]
**virtual_chain_refund_recipient** | Option<**String**> | EVM address of a refund recipient in a virtual chain | [optional]
**custom_recipient_msg** | Option<**String**> | **HIGHLY EXPERIMENTAL** Message to pass to `ft_transfer_call` when withdrawing assets to NEAR.  Otherwise, `ft_transfer` will be used.  **WARNING**: Funds will be lost if used with non NEP-141 tokens, in case of insufficient `storage_deposit` or if the recipient does not implement `ft_on_transfer` method. | [optional]
**recipient_type** | **String** | Type of recipient address: - `DESTINATION_CHAIN` - assets are transferred to the chain of `destinationAsset`. - `INTENTS` - assets are transferred to an account inside Intents | 
**deadline** | **String** | Timestamp in ISO format that identifies when the user refund begins if the swap isn't completed by then. It must exceed the time required for the deposit transaction to be mined. For example, Bitcoin may require around one hour depending on the fees paid. | 
**referral** | Option<**String**> | Referral identifier (lowercase only). It will be reflected in the on-chain data and displayed on public analytics platforms. | [optional]
**quote_waiting_time_ms** | Option<**f64**> | Time in milliseconds the user is willing to wait for a quote from the relay. **If you want to receive the fastest quote - use `0` as a value**  | [optional][default to 3000]
**app_fees** | Option<[**Vec<models::AppFee>**](AppFee.md)> | List of recipients and their fees | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


