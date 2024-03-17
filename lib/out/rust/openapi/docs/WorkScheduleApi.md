# \WorkScheduleApi

All URIs are relative to *https://community.openproject.org*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_non_working_day**](WorkScheduleApi.md#create_non_working_day) | **POST** /api/v3/days/non_working | Creates a non-working day (NOT IMPLEMENTED)
[**delete_non_working_day**](WorkScheduleApi.md#delete_non_working_day) | **DELETE** /api/v3/days/non_working/{date} | Removes a non-working day (NOT IMPLEMENTED)
[**list_days**](WorkScheduleApi.md#list_days) | **GET** /api/v3/days | Lists days
[**list_non_working_days**](WorkScheduleApi.md#list_non_working_days) | **GET** /api/v3/days/non_working | Lists all non working days
[**list_week_days**](WorkScheduleApi.md#list_week_days) | **GET** /api/v3/days/week | Lists week days
[**update_non_working_day**](WorkScheduleApi.md#update_non_working_day) | **PATCH** /api/v3/days/non_working/{date} | Update a non-working day attributes (NOT IMPLEMENTED)
[**update_week_day**](WorkScheduleApi.md#update_week_day) | **PATCH** /api/v3/days/week/{day} | Update a week day attributes (NOT IMPLEMENTED)
[**update_week_days**](WorkScheduleApi.md#update_week_days) | **PATCH** /api/v3/days/week | Update week days (NOT IMPLEMENTED)
[**view_day**](WorkScheduleApi.md#view_day) | **GET** /api/v3/days/{date} | View day
[**view_non_working_day**](WorkScheduleApi.md#view_non_working_day) | **GET** /api/v3/days/non_working/{date} | View a non-working day
[**view_week_day**](WorkScheduleApi.md#view_week_day) | **GET** /api/v3/days/week/{day} | View a week day



## create_non_working_day

> models::NonWorkingDayModel create_non_working_day(non_working_day_model)
Creates a non-working day (NOT IMPLEMENTED)

**(NOT IMPLEMENTED)** Marks a day as being a non-working day.  Note: creating a non-working day will not affect the start and finish dates of work packages but will affect their duration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**non_working_day_model** | Option<[**NonWorkingDayModel**](NonWorkingDayModel.md)> |  |  |

### Return type

[**models::NonWorkingDayModel**](NonWorkingDayModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_non_working_day

> delete_non_working_day(date)
Removes a non-working day (NOT IMPLEMENTED)

**(NOT IMPLEMENTED)** Removes the non-working day at the given date.  Note: deleting a non-working day will not affect the start and finish dates of work packages but will affect their duration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | **String** | The date of the non-working day to view in ISO 8601 format. | [required] |

### Return type

 (empty response body)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_days

> models::DayCollectionModel list_days(filters)
Lists days

Lists days information for a given date interval.  All days from the beginning of current month to the end of following month are returned by default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON specifying filter conditions.  Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported filters are:  + date: the inclusive date interval to scope days to look up. When   unspecified, default is from the beginning of current month to the end   of following month.    Example: `{ \"date\": { \"operator\": \"<>d\", \"values\": [\"2022-05-02\",\"2022-05-26\"] } }`   would return days between May 5 and May 26 2022, inclusive.  + working: when `true`, returns only the working days. When `false`,   returns only the non-working days (weekend days and non-working days).   When unspecified, returns both working and non-working days.    Example: `{ \"working\": { \"operator\": \"=\", \"values\": [\"t\"] } }`   would exclude non-working days from the response. |  |

### Return type

[**models::DayCollectionModel**](DayCollectionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_non_working_days

> models::NonWorkingDayCollectionModel list_non_working_days(filters)
Lists all non working days

Lists all one-time non working days, such as holidays. It does not lists the non working weekdays, such as each Saturday, Sunday. For listing the weekends, the `/api/v3/days` endpoint should be used.  All days from current year are returned by default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | JSON specifying filter conditions.  Accepts the same format as returned by the [queries](https://www.openproject.org/docs/api/endpoints/queries/) endpoint. Currently supported filters are:  + date: the inclusive date interval to scope days to look up. When   unspecified, default is from the beginning to the end of current year.    Example: `{ \"date\": { \"operator\": \"<>d\", \"values\": [\"2022-05-02\",\"2022-05-26\"] } }`   would return days between May 5 and May 26 2022, inclusive. |  |

### Return type

[**models::NonWorkingDayCollectionModel**](NonWorkingDayCollectionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_week_days

> models::WeekDayCollectionModel list_week_days()
Lists week days

Lists week days with work schedule information.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WeekDayCollectionModel**](WeekDayCollectionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_non_working_day

> models::NonWorkingDayModel update_non_working_day(date, non_working_day_model)
Update a non-working day attributes (NOT IMPLEMENTED)

**(NOT IMPLEMENTED)** Update the non-working day information for a given date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | **String** | The date of the non-working day to view in ISO 8601 format. | [required] |
**non_working_day_model** | Option<[**NonWorkingDayModel**](NonWorkingDayModel.md)> |  |  |

### Return type

[**models::NonWorkingDayModel**](NonWorkingDayModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_week_day

> models::WeekDayModel update_week_day(day, week_day_write_model)
Update a week day attributes (NOT IMPLEMENTED)

**(NOT IMPLEMENTED)** Makes a week day a working or non-working day.  Note: changing a week day working attribute will not affect the start and finish dates of work packages but will affect their duration attribute.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**day** | **i32** | The week day from 1 to 7. 1 is Monday. 7 is Sunday. | [required] |
**week_day_write_model** | Option<[**WeekDayWriteModel**](WeekDayWriteModel.md)> |  |  |

### Return type

[**models::WeekDayModel**](WeekDayModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_week_days

> models::WeekDayCollectionModel update_week_days(week_day_collection_write_model)
Update week days (NOT IMPLEMENTED)

**(NOT IMPLEMENTED)** Update multiple week days with work schedule information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**week_day_collection_write_model** | Option<[**WeekDayCollectionWriteModel**](WeekDayCollectionWriteModel.md)> |  |  |

### Return type

[**models::WeekDayCollectionModel**](WeekDayCollectionModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/hal+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_day

> models::DayModel view_day(date)
View day

View the day information for a given date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | **String** | The date of the non-working day to view in ISO 8601 format. | [required] |

### Return type

[**models::DayModel**](DayModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_non_working_day

> models::NonWorkingDayModel view_non_working_day(date)
View a non-working day

Returns the non-working day information for a given date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**date** | **String** | The date of the non-working day to view in ISO 8601 format. | [required] |

### Return type

[**models::NonWorkingDayModel**](NonWorkingDayModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_week_day

> models::WeekDayModel view_week_day(day)
View a week day

View a week day and its attributes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**day** | **i32** | The week day from 1 to 7. 1 is Monday. 7 is Sunday. | [required] |

### Return type

[**models::WeekDayModel**](WeekDayModel.md)

### Authorization

[BasicAuth](../README.md#BasicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/hal+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

