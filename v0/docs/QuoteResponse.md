# QuoteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timestamp** | **String** | Timestamp in ISO format that took part in the deposit address derivation | 
**signature** | **String** | Signature of the 1Click service confirming the quote for the specific deposit address. Must be saved on the client side (along with the whole quote) in order to resolve any disputes or mistakes. | 
**quote_request** | [**models::QuoteRequest**](QuoteRequest.md) | User request | 
**quote** | [**models::Quote**](Quote.md) | Response that contains the deposit address to send \"amount\" of `originAsset` and possible output amount. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


