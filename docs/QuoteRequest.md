# QuoteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**dry** | **bool** |    Flag indicating whether this is a dry run request.    If **true**, the response will **NOT** contain the following fields:       **depositAddress**           **timeWhenInactive**           **timeEstimate**           **deadline**    | 
**swap_type** | **String** | Whether to use the amount as the output or the input for the basis of the swap.     EXACT_OUTPUT: the **refundTo** address will always receive excess tokens back even after the swap is complete.    | 
**slippage_tolerance** | **f64** | Slippage tolerance for the swap. This value is in basis points (1/100th of a percent), e.g. 100 for 1% slippage. | 
**origin_asset** | **String** | ID of origin asset | 
**deposit_type** | **String** | Type of deposit address      ORIGIN_CHAIN - deposit address on origin chain      INTENTS - **accountId** inside near intents to which you should transfer assets inside intents. | 
**destination_asset** | **String** | ID of destination asset | 
**amount** | **String** | Amount to swap as the base amount (can be switched to exact input/output using the dedicated flag), denoted in the smallest unit of the specified currency (e.g., wei for ETH) | 
**refund_to** | **String** | Address for user refund | 
**refund_type** | **String** | Type of refund address       ORIGIN_CHAIN - assets will be refunded to **refundTo** address on origin chain        INTENTS - assets will be refunded to **refundTo** intents account | 
**recipient** | **String** | Recipient address, format should match **recipientType** | 
**recipient_type** | **String** | Type of recipient address       DESTINATION_CHAIN - assets will be transferred to chain of **destinationAsset**        INTENTS - assets will be transferred to account inside intents | 
**deadline** | **String** | Timestamp in ISO format, that identifies when user refund will begin if swap was`t completed by then | 
**referral** | Option<**String**> | Referral identifier | [optional]
**quote_waiting_time_ms** | Option<**f64**> | Time in milliseconds user is willing to wait for quote from relay | [optional][default to 3000]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


