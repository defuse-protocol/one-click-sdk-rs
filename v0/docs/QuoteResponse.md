# QuoteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**correlation_id** | **String** | Unique identifier for request tracing and debugging | 
**timestamp** | **String** | Timestamp in ISO format that was used to derive the deposit address | 
**signature** | **String** | Signature of the 1Click service confirming the quote for the specific deposit address. Must be saved on the client side (along with the whole quote) in order to resolve any disputes or mistakes. | 
**quote_request** | [**models::QuoteRequest**](QuoteRequest.md) | User request | 
**quote** | [**models::Quote**](Quote.md) | Response containing the deposit address for sending the `amount` of `originAsset` and the expected output amount. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


