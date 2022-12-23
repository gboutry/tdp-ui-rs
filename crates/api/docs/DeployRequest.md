# DeployRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**targets** | Option<**Vec<String>**> | List of operations used as targets on the dag generation, mutually exclusive with `sources`. | [optional]
**sources** | Option<**Vec<String>**> | List of operations used as sources on the dag generation, mutually exclusive with `targets`. | [optional]
**filter_type** | Option<[**crate::models::FilterTypeEnum**](FilterTypeEnum.md)> | Controls how the filter expression must be interpreted. | [optional]
**filter_expression** | Option<**String**> | Expression which will match on the operation list generated from the dag. Only operations matching will be kept for the deployment. | [optional]
**restart** | Option<**bool**> | Controls whether or not start operations will be replaced by a restart operation. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


