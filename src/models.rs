use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Report {
    pub uuid: Option<String>,
    pub decision_visibility: Option<String>,
    pub decision_visibility_other: Option<String>,
    pub end_date_visibility_restriction: Option<String>,
    pub decision_monetary: Option<String>,
    pub decision_monetary_other: Option<String>,
    pub end_date_monetary_restriction: Option<String>,
    pub decision_provision: Option<String>,
    pub end_date_service_restriction: Option<String>,
    pub decision_account: Option<String>,
    pub end_date_account_restriction: Option<String>,
    pub account_type: Option<String>,
    pub decision_ground: Option<String>,
    pub decision_ground_reference_url: Option<String>,
    pub illegal_content_legal_ground: Option<String>,
    pub incompatible_content_ground: Option<String>,
    pub incompatible_content_illegal: Option<String>,
    pub category: Option<String>,
    pub category_addition: Option<String>,
    pub category_specification: Option<String>,
    pub category_specification_other: Option<String>,
    pub content_type: Option<String>,
    pub content_type_other: Option<String>,
    pub content_language: Option<String>,
    pub content_date: Option<String>,
    pub application_date: Option<String>,
    pub source_type: Option<String>,
    pub source_identity: Option<String>,
    pub automated_detection: Option<String>,
    pub automated_decision: Option<String>,
    pub platform_name: Option<String>,
    pub platform_uid: Option<String>,
    pub created_at: Option<String>,
    pub report_id: Option<String>,
    pub target_id: Option<String>,
    pub report_type: Option<String>,
}
