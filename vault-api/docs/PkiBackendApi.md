# \PkiBackendApi

All URIs are relative to *http://localhost/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**generate_cert**](PkiBackendApi.md#generate_cert) | **Post** /{mount}/issue/{name} | Generate certificate
[**read_cert**](PkiBackendApi.md#read_cert) | **Get** /{mount}/cert/{serial} | Read certificate


# **generate_cert**
> ::models::GenerateCertificateResponse generate_cert(x_vault_token, mount, name, body)
Generate certificate

Generate a new certificate based on a role. The private key is not stored and must be retained by the client.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **x_vault_token** | **String**| Vault token for authorization | 
  **mount** | **String**| Name of the mount - commonly \&quot;pki\&quot;. | 
  **name** | **String**| Name of the role to create this certificate against | 
  **body** | [**GenerateCertificateParameters**](GenerateCertificateParameters.md)| Parameters | 

### Return type

[**::models::GenerateCertificateResponse**](GenerateCertificateResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_cert**
> ::models::CertificateResponse read_cert(mount, serial)
Read certificate

Read a certificate in PEM format (within JSON). This is an unauthenticated endpoint.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **mount** | **String**| Name of the mount - commonly \&quot;pki\&quot;. | 
  **serial** | **String**| Name of the certificate | 

### Return type

[**::models::CertificateResponse**](CertificateResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

