# \OneClickApi

All URIs are relative to *https://1click.chaindefuser.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_execution_status**](OneClickApi.md#get_execution_status) | **GET** /v0/status | Check swap execution status
[**get_quote**](OneClickApi.md#get_quote) | **POST** /v0/quote | Request a swap quote
[**get_tokens**](OneClickApi.md#get_tokens) | **GET** /v0/tokens | Get supported tokens
[**submit_deposit_tx**](OneClickApi.md#submit_deposit_tx) | **POST** /v0/deposit/submit | Submit deposit transaction hash



## get_execution_status

> models::GetExecutionStatusResponse get_execution_status(deposit_address)
Check swap execution status

Retrieves the current status of a swap using the unique deposit address from the quote.  The response includes the state of the swap (e.g., pending, processing, success, refunded) and any associated swap and transaction details.

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
Request a swap quote

Generates a swap quote based on input parameters such as the assets, amount, slippage tolerance, and recipient/refund information.  Returns pricing details, estimated time, and a unique **deposit address** to which tokens must be transferred to initiate the swap.  You can set the `dry` parameter to `true` to simulate the quote request **without generating a deposit address** or initiating the swap process. This is useful for previewing swap parameters or validating input data without committing to an actual swap.  This endpoint is the first required step in the swap process.

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
Get supported tokens

Retrieves a list of tokens currently supported by the 1Click API for asset swaps.  Each token entry includes its blockchain, contract address (if available), price in USD, and other metadata such as symbol and decimals.

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
Submit deposit transaction hash

Optionally notifies the 1Click service that a deposit has been sent to the specified address, using the blockchain transaction hash.  This step can speed up swap processing by allowing the system to preemptively verify the deposit.

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

