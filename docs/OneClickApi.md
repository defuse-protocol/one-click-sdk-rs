# \OneClickApi

All URIs are relative to *https://1click.chaindefuser.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_execution_status**](OneClickApi.md#get_execution_status) | **Get** /v0/status | Returns execution status for a given deposit address
[**get_quote**](OneClickApi.md#get_quote) | **Post** /v0/quote | Returns an executable parameters for quote
[**get_tokens**](OneClickApi.md#get_tokens) | **Get** /v0/tokens | Returns tokens that can be swapped
[**submit_deposit_tx**](OneClickApi.md#submit_deposit_tx) | **Post** /v0/deposit/submit | Submit a deposit transaction



## get_execution_status

> models::GetExecutionStatusResponse get_execution_status(deposit_address)
Returns execution status for a given deposit address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deposit_address** | **String** |  | [required] |

### Return type

[**models::GetExecutionStatusResponse**](GetExecutionStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quote

> models::QuoteResponse get_quote(quote_request)
Returns an executable parameters for quote

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quote_request** | [**QuoteRequest**](QuoteRequest.md) |  | [required] |

### Return type

[**models::QuoteResponse**](QuoteResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tokens

> Vec<models::TokenResponse> get_tokens()
Returns tokens that can be swapped

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::TokenResponse>**](TokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_deposit_tx

> models::SubmitDepositTxResponse submit_deposit_tx(submit_deposit_tx_request)
Submit a deposit transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**submit_deposit_tx_request** | [**SubmitDepositTxRequest**](SubmitDepositTxRequest.md) |  | [required] |

### Return type

[**models::SubmitDepositTxResponse**](SubmitDepositTxResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

