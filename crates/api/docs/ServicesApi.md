# \ServicesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_service_api_v1_service_service_id_get**](ServicesApi.md#get_service_api_v1_service_service_id_get) | **GET** /api/v1/service/{service_id} | Get Service
[**get_services_api_v1_service_get**](ServicesApi.md#get_services_api_v1_service_get) | **GET** /api/v1/service/ | Get Services
[**patch_service_api_v1_service_service_id_patch**](ServicesApi.md#patch_service_api_v1_service_service_id_patch) | **PATCH** /api/v1/service/{service_id} | Patch Service
[**put_service_api_v1_service_service_id_put**](ServicesApi.md#put_service_api_v1_service_service_id_put) | **PUT** /api/v1/service/{service_id} | Put Service



## get_service_api_v1_service_service_id_get

> crate::models::Service get_service_api_v1_service_service_id_get(service_id)
Get Service

Gets service identified by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** |  | [required] |

### Return type

[**crate::models::Service**](Service.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_services_api_v1_service_get

> Vec<crate::models::Service> get_services_api_v1_service_get()
Get Services

Returns the list of services

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Service>**](Service.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_service_api_v1_service_service_id_patch

> crate::models::ServiceUpdateResponse patch_service_api_v1_service_service_id_patch(service_id, service_update)
Patch Service

Modifies a service definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** |  | [required] |
**service_update** | [**ServiceUpdate**](ServiceUpdate.md) |  | [required] |

### Return type

[**crate::models::ServiceUpdateResponse**](ServiceUpdateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_service_api_v1_service_service_id_put

> crate::models::ServiceUpdateResponse put_service_api_v1_service_service_id_put(service_id, service_update)
Put Service

Sets a service definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** |  | [required] |
**service_update** | [**ServiceUpdate**](ServiceUpdate.md) |  | [required] |

### Return type

[**crate::models::ServiceUpdateResponse**](ServiceUpdateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

