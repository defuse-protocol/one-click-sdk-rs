# QuoteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timestamp** | **String** | Timestamp in ISO format that took part in deposit address derivation | 
**signature** | **String** | Signature that means 1click service commit to quote and deposit address | 
**quote_request** | [**models::QuoteRequest**](QuoteRequest.md) | User request | 
**quote** | [**models::Quote**](Quote.md) | Response that contains deposit address for user to send \"amount\" of **originAsset** and possible output amount | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


