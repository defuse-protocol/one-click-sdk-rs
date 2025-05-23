# Quote

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deposit_address** | Option<**String**> | The deposit address on the chain of `originAsset` in case if `depositType` is `ORIGIN_CHAIN`.  The deposit address inside of near intents (the verifier smart contract) in case if `depositType` is `INTENTS`. | [optional]
**amount_in** | **String** | Amount of the origin asset | 
**amount_in_formatted** | **String** | Amount of the origin asset in readable format | 
**amount_in_usd** | **String** | Amount of the origin assets equivalent in USD | 
**min_amount_in** | **String** | Minimum amount of the origin asset that will be used for swap | 
**amount_out** | **String** | Amount of the destination asset | 
**amount_out_formatted** | **String** | Amount of the destination asset in readable format | 
**amount_out_usd** | **String** | Amount of the destination asset equivalent in USD | 
**min_amount_out** | **String** | Minimum amount with slippage taken into account | 
**deadline** | Option<**String**> | Time when the deposit address will become inactive and funds might be lost | [optional]
**time_when_inactive** | Option<**String**> | Time when the deposit address will become cold and swap processing will take more time | [optional]
**time_estimate** | **f64** | Estimated time in seconds for swap to be executed after the deposit transaction is confirmed | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


