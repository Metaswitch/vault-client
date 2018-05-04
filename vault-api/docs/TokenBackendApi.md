# \TokenBackendApi

All URIs are relative to *http://localhost/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_orphan_token**](TokenBackendApi.md#create_orphan_token) | **Post** /auth/token/create-orphan | Create an orphan token
[**create_token**](TokenBackendApi.md#create_token) | **Post** /auth/token/create | Create token
[**log_in_with_tls_certificate**](TokenBackendApi.md#log_in_with_tls_certificate) | **Post** /auth/cert/login | Log in
[**renew_own_token**](TokenBackendApi.md#renew_own_token) | **Post** /auth/token/renew-self | Renew own token


# **create_orphan_token**
> ::models::AuthResponse create_orphan_token(x_vault_token, body)
Create an orphan token

Create a new token without basing it off of a role.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_vault_token** | **String**| Vault token for authorization | 
  **body** | [**CreateTokenParameters**](CreateTokenParameters.md)| Parameters | 

### Return type

[**::models::AuthResponse**](AuthResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_token**
> ::models::AuthResponse create_token(x_vault_token, body)
Create token

Create a new token without basing it off of a role.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_vault_token** | **String**| Vault token for authorization | 
  **body** | [**CreateTokenParameters**](CreateTokenParameters.md)| Parameters | 

### Return type

[**::models::AuthResponse**](AuthResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **log_in_with_tls_certificate**
> ::models::AuthResponse log_in_with_tls_certificate(optional)
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

[**::models::AuthResponse**](AuthResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **renew_own_token**
> ::models::AuthResponse renew_own_token(x_vault_token, body)
Renew own token

Renews the token used to make this request.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_vault_token** | **String**| Vault token for authorization | 
  **body** | [**RenewSelfParameters**](RenewSelfParameters.md)| Parameters | 

### Return type

[**::models::AuthResponse**](AuthResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

