# TokenResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asset_id** | **String** | Unique asset identifier | 
**decimals** | **f64** | Number of decimals for the token | 
**blockchain** | **Blockchain** | Blockchain associated with the token (enum: near, eth, base, arb, btc, sol, ton, doge, xrp, zec, gnosis, bera, bsc, pol, tron, sui, op, avax, cardano, ltc, xlayer, monad, bch, adi, plasma, starknet, aleo) | 
**symbol** | **String** | Token symbol (e.g. BTC, ETH) | 
**price** | **f64** | Current price of the token in USD | 
**price_updated_at** | **String** | Date when the token price was last updated | 
**contract_address** | Option<**String**> | Contract address of the token | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


