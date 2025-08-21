# Quote

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deposit_address** | Option<**String**> | The deposit address on the chain of `originAsset` when `depositType` is `ORIGIN_CHAIN`.  The deposit address inside NEAR Intents (the verifier smart contract) when `depositType` is `INTENTS`. | [optional]
**deposit_memo** | Option<**String**> | Some of the deposit addresses **REQUIRE** to also include the `memo` for the deposit to be processed | [optional]
**amount_in** | **String** | Amount of the origin asset | 
**amount_in_formatted** | **String** | Amount of the origin asset in readable format | 
**amount_in_usd** | **String** | Amount of the origin assets equivalent in USD | 
**min_amount_in** | **String** | Minimum amount of the origin asset that will be used for the swap | 
**amount_out** | **String** | Amount of the destination asset | 
**amount_out_formatted** | **String** | Amount of the destination asset in readable format | 
**amount_out_usd** | **String** | Amount of the destination asset equivalent in USD | 
**min_amount_out** | **String** | Minimum output amount after slippage is applied | 
**deadline** | Option<**String**> | Time when the deposit address becomes inactive and funds may be lost | [optional]
**time_when_inactive** | Option<**String**> | Time when the deposit address becomes cold, causing swap processing to take longer | [optional]
**time_estimate** | **f64** | Estimated time in seconds for the swap to be executed after the deposit transaction is confirmed | 
**virtual_chain_recipient** | Option<**String**> | EVM address of a transfer recipient in a virtual chain | [optional]
**virtual_chain_refund_recipient** | Option<**String**> | EVM address of a refund recipient in a virtual chain | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


