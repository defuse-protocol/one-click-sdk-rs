# QuoteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dry** | **bool** | Flag indicating whether this is a dry run request. If `true`, the response will **NOT** contain the following fields: - `depositAddress` - `timeWhenInactive` - `deadline` | 
**deposit_mode** | Option<**String**> | What deposit address mode you will get in the response, most chain supports only `SIMPLE` and some(for example `stellar`) only `MEMO`: - `SIMPLE` - usual deposit with only deposit address. - `MEMO` - some chains will **REQUIRE** the `memo` together with `depositAddress` for swap to work. | [optional][default to Simple]
**swap_type** | **String** | How to interpret `amount` when performing the swap:   - `EXACT_INPUT` - requests the output amount for an exact input.   - `EXACT_OUTPUT` - requests the input amount for an exact output. The `refundTo` address always receives any excess tokens after the swap is complete.   - `FLEX_INPUT` - a flexible input amount that allows for partial deposits and variable amounts. | 
**slippage_tolerance** | **f64** | Slippage tolerance for the swap. This value is in basis points (1/100th of a percent), e.g. 100 for 1% slippage. | 
**origin_asset** | **String** | ID of the origin asset. | 
**deposit_type** | **String** | Type of deposit address: - `ORIGIN_CHAIN` - deposit address on the origin chain. - `INTENTS` - the account ID within NEAR Intents to which you should transfer assets. | 
**destination_asset** | **String** | ID of the destination asset. | 
**amount** | **String** | Amount to swap as the base amount. It is interpreted as the input or output amount based on the `swapType` flag and is specified in the smallest unit of the currency (e.g., wei for ETH). | 
**refund_to** | **String** | Address used for refunds. | 
**refund_type** | **String** | Type of refund address: - `ORIGIN_CHAIN` - assets are refunded to the `refundTo` address on the origin chain. - `INTENTS` - assets are refunded to the `refundTo` Intents account. | 
**recipient** | **String** | Recipient address. The format must match `recipientType`. | 
**virtual_chain_recipient** | Option<**String**> | EVM address of a transfer recipient in a virtual chain | [optional]
**virtual_chain_refund_recipient** | Option<**String**> | EVM address of a refund recipient in a virtual chain | [optional]
**recipient_type** | **String** | Type of recipient address: - `DESTINATION_CHAIN` - assets are transferred to the chain of `destinationAsset`. - `INTENTS` - assets are transferred to an account inside Intents | 
**deadline** | **String** | Timestamp in ISO format that identifies when the user refund begins if the swap isn't completed by then. It must exceed the time required for the deposit transaction to be mined. For example, Bitcoin may require around one hour depending on the fees paid. | 
**referral** | Option<**String**> | Referral identifier (lowercase only). It will be reflected in the on-chain data and displayed on public analytics platforms. | [optional]
**quote_waiting_time_ms** | Option<**f64**> | Time in milliseconds the user is willing to wait for a quote from the relay. | [optional][default to 3000]
**app_fees** | Option<[**Vec<models::AppFee>**](AppFee.md)> | List of recipients and their fees | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


