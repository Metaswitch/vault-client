# leases_api

All URIs are relative to *http://localhost/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
**revoke_lease**](leases_api.md#revoke_lease) | **PUT** /sys/leases/revoke | Revoke lease


# **revoke_lease**
> revoke_lease(x_vault_token, body)
Revoke lease

Revoke a lease

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_vault_token** | **String**| Vault token for authorization | 
  **body** | [**RevokeLeaseParameters**](RevokeLeaseParameters.md)| Parameters | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

