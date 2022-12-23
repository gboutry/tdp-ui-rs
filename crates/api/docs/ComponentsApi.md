# \ComponentsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_component_api_v1_service_service_id_component_component_id_get**](ComponentsApi.md#get_component_api_v1_service_service_id_component_component_id_get) | **GET** /api/v1/service/{service_id}/component/{component_id} | Get Component
[**patch_component_api_v1_service_service_id_component_component_id_patch**](ComponentsApi.md#patch_component_api_v1_service_service_id_component_component_id_patch) | **PATCH** /api/v1/service/{service_id}/component/{component_id} | Patch Component
[**put_component_api_v1_service_service_id_component_component_id_put**](ComponentsApi.md#put_component_api_v1_service_service_id_component_component_id_put) | **PUT** /api/v1/service/{service_id}/component/{component_id} | Put Component



## get_component_api_v1_service_service_id_component_component_id_get

> crate::models::Component get_component_api_v1_service_service_id_component_component_id_get(service_id, component_id)
Get Component

Gets component identified by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** |  | [required] |
**component_id** | **String** |  | [required] |

### Return type

[**crate::models::Component**](Component.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_component_api_v1_service_service_id_component_component_id_patch

> crate::models::ComponentUpdateResponse patch_component_api_v1_service_service_id_component_component_id_patch(service_id, component_id, component_update)
Patch Component

Modifies a component definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** |  | [required] |
**component_id** | **String** |  | [required] |
**component_update** | [**ComponentUpdate**](ComponentUpdate.md) |  | [required] |

### Return type

[**crate::models::ComponentUpdateResponse**](ComponentUpdateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_component_api_v1_service_service_id_component_component_id_put

> crate::models::ComponentUpdateResponse put_component_api_v1_service_service_id_component_component_id_put(service_id, component_id, component_update)
Put Component

Sets a component definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_id** | **String** |  | [required] |
**component_id** | **String** |  | [required] |
**component_update** | [**ComponentUpdate**](ComponentUpdate.md) |  | [required] |

### Return type

[**crate::models::ComponentUpdateResponse**](ComponentUpdateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

