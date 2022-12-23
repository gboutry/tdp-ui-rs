# \DeployApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**deploy_node_api_v1_deploy_post**](DeployApi.md#deploy_node_api_v1_deploy_post) | **POST** /api/v1/deploy/ | Deploy Node
[**deployment_status_api_v1_deploy_status_get**](DeployApi.md#deployment_status_api_v1_deploy_status_get) | **GET** /api/v1/deploy/status | Deployment Status
[**get_deployment_api_v1_deploy_deployment_id_get**](DeployApi.md#get_deployment_api_v1_deploy_deployment_id_get) | **GET** /api/v1/deploy/{deployment_id} | Get Deployment
[**get_deployment_operation_api_v1_deploy_deployment_id_operation_operation_get**](DeployApi.md#get_deployment_operation_api_v1_deploy_deployment_id_operation_operation_get) | **GET** /api/v1/deploy/{deployment_id}/operation/{operation} | Get Deployment Operation
[**get_deployments_api_v1_deploy_get**](DeployApi.md#get_deployments_api_v1_deploy_get) | **GET** /api/v1/deploy/ | Get Deployments
[**operations_api_v1_deploy_operations_post**](DeployApi.md#operations_api_v1_deploy_operations_post) | **POST** /api/v1/deploy/operations | Operations
[**reconfigure_api_v1_deploy_reconfigure_post**](DeployApi.md#reconfigure_api_v1_deploy_reconfigure_post) | **POST** /api/v1/deploy/reconfigure | Reconfigure
[**resume_api_v1_deploy_resume_post**](DeployApi.md#resume_api_v1_deploy_resume_post) | **POST** /api/v1/deploy/resume | Resume



## deploy_node_api_v1_deploy_post

> serde_json::Value deploy_node_api_v1_deploy_post(deploy_request)
Deploy Node

Launches a deployment from the dag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deploy_request** | Option<[**DeployRequest**](DeployRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deployment_status_api_v1_deploy_status_get

> crate::models::DeployStatus deployment_status_api_v1_deploy_status_get()
Deployment Status

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::DeployStatus**](DeployStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deployment_api_v1_deploy_deployment_id_get

> crate::models::DeploymentLogWithOperations get_deployment_api_v1_deploy_deployment_id_get(deployment_id)
Get Deployment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_id** | **i32** |  | [required] |

### Return type

[**crate::models::DeploymentLogWithOperations**](DeploymentLogWithOperations.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deployment_operation_api_v1_deploy_deployment_id_operation_operation_get

> crate::models::OperationLog get_deployment_operation_api_v1_deploy_deployment_id_operation_operation_get(deployment_id, operation)
Get Deployment Operation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deployment_id** | **i32** |  | [required] |
**operation** | **String** |  | [required] |

### Return type

[**crate::models::OperationLog**](OperationLog.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deployments_api_v1_deploy_get

> Vec<crate::models::DeploymentLog> get_deployments_api_v1_deploy_get(limit, offset)
Get Deployments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |[default to 15]
**offset** | Option<**i32**> |  |  |[default to 0]

### Return type

[**Vec<crate::models::DeploymentLog>**](DeploymentLog.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## operations_api_v1_deploy_operations_post

> serde_json::Value operations_api_v1_deploy_operations_post(operations_request)
Operations

Run a list of operations on the cluster, can use operations outside of the dag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**operations_request** | [**OperationsRequest**](OperationsRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reconfigure_api_v1_deploy_reconfigure_post

> serde_json::Value reconfigure_api_v1_deploy_reconfigure_post()
Reconfigure

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resume_api_v1_deploy_resume_post

> serde_json::Value resume_api_v1_deploy_resume_post(resume_request)
Resume

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resume_request** | Option<[**ResumeRequest**](ResumeRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

