# \OperationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_dag_operations_api_v1_operation_dag_get**](OperationApi.md#get_dag_operations_api_v1_operation_dag_get) | **GET** /api/v1/operation/dag | Get Dag Operations
[**get_operations_api_v1_operation_get**](OperationApi.md#get_operations_api_v1_operation_get) | **GET** /api/v1/operation/ | Get Operations
[**get_other_operations_api_v1_operation_other_get**](OperationApi.md#get_other_operations_api_v1_operation_other_get) | **GET** /api/v1/operation/other | Get Other Operations



## get_dag_operations_api_v1_operation_dag_get

> Vec<crate::models::Operation> get_dag_operations_api_v1_operation_dag_get()
Get Dag Operations

Return list of every dag operation, topologically sorted

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Operation>**](Operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_operations_api_v1_operation_get

> Vec<crate::models::Operation> get_operations_api_v1_operation_get()
Get Operations

Returns every operations possible

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Operation>**](Operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_other_operations_api_v1_operation_other_get

> Vec<crate::models::Operation> get_other_operations_api_v1_operation_other_get()
Get Other Operations

Returns operations outside of the dag

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Operation>**](Operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

