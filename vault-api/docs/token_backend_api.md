# token_backend_api

All URIs are relative to *http://localhost/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
**create_orphan_token**](token_backend_api.md#create_orphan_token) | **POST** /auth/token/create-orphan | Create an orphan token
**create_token**](token_backend_api.md#create_token) | **POST** /auth/token/create | Create token
**Log_in_with_TLS_certificate**](token_backend_api.md#Log_in_with_TLS_certificate) | **POST** /auth/cert/login | Log in
**renew_own_token**](token_backend_api.md#renew_own_token) | **POST** /auth/token/renew-self | Renew own token


# **create_orphan_token**
> models::AuthResponse create_orphan_token(x_vault_token, body)
Create an orphan token

Create a new token without basing it off of a role.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_vault_token** | **String**| Vault token for authorization | 
  **body** | [**CreateTokenParameters**](CreateTokenParameters.md)| Parameters | 

### Return type

[**models::AuthResponse**](AuthResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_token**
> models::AuthResponse create_token(x_vault_token, body)
Create token

Create a new token without basing it off of a role.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_vault_token** | **String**| Vault token for authorization | 
  **body** | [**CreateTokenParameters**](CreateTokenParameters.md)| Parameters | 

### Return type

[**models::AuthResponse**](AuthResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **Log_in_with_TLS_certificate**
> models::AuthResponse Log_in_with_TLS_certificate(optional)
Log in

Log in with a TLS certificate

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**AuthCertLoginParameters**](AuthCertLoginParameters.md)| Parameters | 

### Return type

[**models::AuthResponse**](AuthResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: */*, 

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **renew_own_token**
> models::AuthResponse renew_own_token(x_vault_token, body)
Renew own token

Renews the token used to make this request.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_vault_token** | **String**| Vault token for authorization | 
  **body** | [**RenewSelfParameters**](RenewSelfParameters.md)| Parameters | 

### Return type

[**models::AuthResponse**](AuthResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

