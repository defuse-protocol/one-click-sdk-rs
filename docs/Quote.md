# Quote

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deposit_address** | Option<**String**> | Deposit address on chain of **originAsset** in case **depositType** is **ORIGIN_CHAIN**.       Deposit address inside near intents in case **depositType** is **INTENTS**. | [optional]
**amount_in** | **String** | Amount of origin asset | 
**amount_in_formatted** | **String** | Amount of origin asset in readable format | 
**amount_in_usd** | **String** | Amount of origin assets equivalent in USD | 
**min_amount_in** | **String** | Minimum amount of origin asset that will be used for swap | 
**amount_out** | **String** | Amount of destination asset | 
**amount_out_formatted** | **String** | Amount of destination asset in readable format | 
**amount_out_usd** | **String** | Amount of destination asset equivalent in USD | 
**min_amount_out** | **String** | Minimum amount with slippage taken into account | 
**deadline** | Option<**String**> | Time when deposit address will become inactive and funds might be lost | [optional]
**time_when_inactive** | Option<**String**> | Time when deposit address will become cold and swap processing will take more time | [optional]
**time_estimate** | Option<**f64**> | Estimated time in seconds for swap to be executed after user deposit transaction is confirmed | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


