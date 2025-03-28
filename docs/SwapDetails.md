# SwapDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**intent_hashes** | **Vec<String>** | All intent hashes that took part in this swap | 
**near_tx_hashes** | **Vec<String>** | All Near transactions executed for this swap | 
**amount_in** | **String** | Exact amount of **originToken** after trade was settled | 
**amount_in_formatted** | **String** | Exact amount of **originToken** after trade was settled in readable format | 
**amount_in_usd** | **String** | Exact amount of **originToken** equivalent in USD | 
**amount_out** | **String** | Exact amount of **destinationToken** after trade was settled | 
**amount_out_formatted** | **String** | Exact amount of **destinationToken** in readable format | 
**amount_out_usd** | **String** | Exact amount of **destinationToken** equivalent in USD | 
**slippage** | **f64** | Actual slippage | 
**origin_chain_tx_hashes** | [**Vec<models::TransactionDetails>**](TransactionDetails.md) | Hashes and explorer URLs for all transactions on origin chain | 
**destination_chain_tx_hashes** | [**Vec<models::TransactionDetails>**](TransactionDetails.md) | Hashes and explorer URLs for all transactions on destination chain | 
**refunded_amount** | Option<**String**> | Amount of **originAsset** that got transferred to **refundTo** | [optional]
**refunded_amount_formatted** | Option<**String**> | Refunded amount in readable format | [optional]
**refunded_amount_usd** | Option<**String**> | Refunded amount equivalent in USD | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


