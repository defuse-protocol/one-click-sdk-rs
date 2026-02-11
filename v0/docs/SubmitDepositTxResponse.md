# SubmitDepositTxResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**correlation_id** | **String** | Unique identifier for request tracing and debugging | 
**quote_response** | [**models::QuoteResponse**](QuoteResponse.md) | Quote response from the original request | 
**status** | **Status** |  (enum: KNOWN_DEPOSIT_TX, PENDING_DEPOSIT, INCOMPLETE_DEPOSIT, PROCESSING, SUCCESS, REFUNDED, FAILED) | 
**updated_at** | **String** | Last time the state was updated | 
**swap_details** | [**models::SwapDetails**](SwapDetails.md) | Details of actual swaps and withdrawals | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


