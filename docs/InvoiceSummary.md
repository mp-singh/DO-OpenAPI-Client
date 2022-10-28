# InvoiceSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**invoice_uuid** | Option<**String**> | UUID of the invoice | [optional]
**billing_period** | Option<**String**> | Billing period of usage for which the invoice is issued, in `YYYY-MM`  format. | [optional]
**amount** | Option<**String**> | Total amount of the invoice, in USD.  This will reflect month-to-date usage in the invoice preview. | [optional]
**user_name** | Option<**String**> | Name of the DigitalOcean customer being invoiced. | [optional]
**user_billing_address** | Option<[**crate::models::InvoiceSummaryUserBillingAddress**](invoice_summary_user_billing_address.md)> |  | [optional]
**user_company** | Option<**String**> | Company of the DigitalOcean customer being invoiced, if set. | [optional]
**user_email** | Option<**String**> | Email of the DigitalOcean customer being invoiced. | [optional]
**product_charges** | Option<[**crate::models::InvoiceSummaryProductCharges**](invoice_summary_product_charges.md)> |  | [optional]
**overages** | Option<[**crate::models::InvoiceSummaryOverages**](invoice_summary_overages.md)> |  | [optional]
**taxes** | Option<[**crate::models::InvoiceSummaryTaxes**](invoice_summary_taxes.md)> |  | [optional]
**credits_and_adjustments** | Option<[**crate::models::InvoiceSummaryCreditsAndAdjustments**](invoice_summary_credits_and_adjustments.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


