use crate::dto::contact_filter_dto::*;
use crate::errors::AppError;
use sqlx::PgPool;
use std::collections::HashMap;
use validator::Validate;

pub struct ContactFilterService;

#[derive(Debug)]
pub struct QueryBuilder {
    sql_parts: Vec<String>,
    parameters: Vec<serde_json::Value>,
    param_counter: usize,
    custom_field_joins: HashMap<String, String>,
    join_counter: usize,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            sql_parts: Vec::new(),
            parameters: Vec::new(),
            param_counter: 1,
            custom_field_joins: HashMap::new(),
            join_counter: 1,
        }
    }
    
    pub fn build_filter_query(
        &mut self,
        filter: &ContactFilterRequest,
    ) -> Result<(String, Vec<serde_json::Value>), AppError> {
        // Start with base query
        let mut base_query = String::from(
            r#"
            SELECT DISTINCT 
                c.id,
                c.first_name,
                c.last_name,
                CONCAT(c.first_name, ' ', c.last_name) as full_name,
                c.email,
                c.phone,
                c.company,
                c.job_title,
                c.lead_status,
                c.created_at,
                c.updated_at,
                COALESCE(
                    json_object_agg(
                        cf.field_name,
                        CASE
                            WHEN cf.field_type = 'number' THEN to_jsonb(ccv.value_number)
                            WHEN cf.field_type = 'date' THEN to_jsonb(ccv.value_date)
                            WHEN cf.field_type = 'boolean' THEN to_jsonb(ccv.value_boolean)
                            ELSE to_jsonb(ccv.value)
                        END
                    ) FILTER (WHERE cf.field_name IS NOT NULL),
                    '{}'::json
                ) as custom_fields
            FROM contacts c
            LEFT JOIN contact_custom_values ccv ON c.id = ccv.contact_id
            LEFT JOIN custom_fields cf ON ccv.custom_field_id = cf.id
            "#
        );
        
        // Build WHERE clause
        if !filter.conditions.is_empty() {
            let root_filter = FilterNode::Group {
                logic: filter.logic.clone(),
                conditions: filter.conditions.clone(),
            };
            
            let where_clause = self.build_where_clause(&root_filter)?;
            base_query.push_str(&format!(" WHERE c.is_active = true AND ({})", where_clause));
        } else {
            base_query.push_str(" WHERE c.is_active = true");
        }
        
        // Add GROUP BY for custom fields aggregation
        base_query.push_str(
            r#"
            GROUP BY c.id, c.first_name, c.last_name, c.email, c.phone, 
                     c.company, c.job_title, c.lead_status, c.created_at, c.updated_at
            "#
        );
        
        // Add sorting
        if let Some(sort_field) = &filter.sort_by {
            let sort_direction = match filter.sort_order {
                SortOrder::Asc => "ASC",
                SortOrder::Desc => "DESC",
            };
            
            if STANDARD_FIELDS.contains(&sort_field.as_str()) {
                base_query.push_str(&format!(" ORDER BY c.{} {}", sort_field, sort_direction));
            } else {
                // Custom field sorting - more complex
                base_query.push_str(&format!(
                    " ORDER BY (custom_fields->>'{}') {}",
                    sort_field, sort_direction
                ));
            }
        } else {
            base_query.push_str(" ORDER BY c.updated_at DESC");
        }
        
        // Add pagination
        let offset = (filter.page - 1) * filter.limit;
        base_query.push_str(&format!(" LIMIT {} OFFSET {}", filter.limit, offset));
        
        Ok((base_query, self.parameters.clone()))
    }
    
    fn build_where_clause(&mut self, node: &FilterNode) -> Result<String, AppError> {
        match node {
            FilterNode::Condition { field, operator, value, field_type } => {
                self.build_condition_clause(field, operator, value, field_type.as_ref())
            }
            FilterNode::Group { logic, conditions } => {
                if conditions.is_empty() {
                    return Ok("1=1".to_string());
                }
                
                let mut clauses = Vec::new();
                for condition in conditions {
                    clauses.push(self.build_where_clause(condition)?);
                }
                
                let logic_op = match logic {
                    LogicOperator::And => " AND ",
                    LogicOperator::Or => " OR ",
                };
                
                Ok(format!("({})", clauses.join(logic_op)))
            }
        }
    }
    
    fn build_condition_clause(
        &mut self,
        field: &str,
        operator: &FilterOperator,
        value: &serde_json::Value,
        field_type: Option<&FieldType>,
    ) -> Result<String, AppError> {
        if STANDARD_FIELDS.contains(&field) {
            self.build_standard_field_condition(field, operator, value)
        } else {
            self.build_custom_field_condition(field, operator, value, field_type)
        }
    }
    
    fn build_standard_field_condition(
        &mut self,
        field: &str,
        operator: &FilterOperator,
        value: &serde_json::Value,
    ) -> Result<String, AppError> {
        let column = format!("c.{}", field);
        self.build_operator_condition(&column, operator, value, None)
    }
    
    fn build_custom_field_condition(
        &mut self,
        field: &str,
        operator: &FilterOperator,
        value: &serde_json::Value,
        field_type: Option<&FieldType>,
    ) -> Result<String, AppError> {
        // For custom fields, we need to check in the aggregated custom_fields JSON
        // But since we're in WHERE clause, we need to use EXISTS subquery
        let param_placeholder = format!("${}", self.param_counter);
        self.param_counter += 1;
        self.parameters.push(value.clone());

        let field_param = format!("${}", self.param_counter);
        self.param_counter += 1;
        self.parameters.push(serde_json::Value::String(field.to_string()));
        
        let condition = match operator {
            FilterOperator::Equals => {
                format!(
                    r#"EXISTS (
                        SELECT 1 FROM contact_custom_values ccv2 
                        JOIN custom_fields cf2 ON ccv2.custom_field_id = cf2.id 
                        WHERE ccv2.contact_id = c.id 
                        AND cf2.field_name = {}
                        AND (
                            (cf2.field_type = 'text' AND ccv2.value = {}) OR
                            (cf2.field_type = 'number' AND ccv2.value_number = {}) OR
                            (cf2.field_type = 'date' AND ccv2.value_date = {}) OR
                            (cf2.field_type = 'boolean' AND ccv2.value_boolean = {})
                        )
                    )"#,
                    field_param, param_placeholder, param_placeholder, param_placeholder, param_placeholder
                )
            }
            FilterOperator::NotEquals => {
                format!(
                    r#"NOT EXISTS (
                        SELECT 1 FROM contact_custom_values ccv2 
                        JOIN custom_fields cf2 ON ccv2.custom_field_id = cf2.id 
                        WHERE ccv2.contact_id = c.id 
                        AND cf2.field_name = {}
                        AND (
                            (cf2.field_type = 'text' AND ccv2.value = {}) OR
                            (cf2.field_type = 'number' AND ccv2.value_number = {}) OR
                            (cf2.field_type = 'date' AND ccv2.value_date = {}) OR
                            (cf2.field_type = 'boolean' AND ccv2.value_boolean = {})
                        )
                    )"#,
                    field_param, param_placeholder, param_placeholder, param_placeholder, param_placeholder
                )
            }
            FilterOperator::Contains => {
                format!(
                    r#"EXISTS (
                        SELECT 1 FROM contact_custom_values ccv2 
                        JOIN custom_fields cf2 ON ccv2.custom_field_id = cf2.id 
                        WHERE ccv2.contact_id = c.id 
                        AND cf2.field_name = {}
                        AND cf2.field_type = 'text'
                        AND ccv2.value ILIKE '%' || {} || '%'
                    )"#,
                    field_param, param_placeholder
                )
            }
            FilterOperator::GreaterThan => {
                format!(
                    r#"EXISTS (
                        SELECT 1 FROM contact_custom_values ccv2 
                        JOIN custom_fields cf2 ON ccv2.custom_field_id = cf2.id 
                        WHERE ccv2.contact_id = c.id 
                        AND cf2.field_name = {}
                        AND cf2.field_type = 'number'
                        AND ccv2.value_number > {}
                    )"#,
                    field_param, param_placeholder
                )
            }
            FilterOperator::LessThan => {
                format!(
                    r#"EXISTS (
                        SELECT 1 FROM contact_custom_values ccv2 
                        JOIN custom_fields cf2 ON ccv2.custom_field_id = cf2.id 
                        WHERE ccv2.contact_id = c.id 
                        AND cf2.field_name = {}
                        AND cf2.field_type = 'number'
                        AND ccv2.value_number < {}
                    )"#,
                    field_param, param_placeholder
                )
            }
            FilterOperator::IsEmpty => {
                format!(
                    r#"NOT EXISTS (
                        SELECT 1 FROM contact_custom_values ccv2 
                        JOIN custom_fields cf2 ON ccv2.custom_field_id = cf2.id 
                        WHERE ccv2.contact_id = c.id 
                        AND cf2.field_name = {}
                    )"#,
                    field_param
                )
            }
            FilterOperator::IsNotEmpty => {
                format!(
                    r#"EXISTS (
                        SELECT 1 FROM contact_custom_values ccv2 
                        JOIN custom_fields cf2 ON ccv2.custom_field_id = cf2.id 
                        WHERE ccv2.contact_id = c.id 
                        AND cf2.field_name = {}
                    )"#,
                    field_param
                )
            }
            _ => {
                return Err(AppError::ValidationError(
                    format!("Operator {:?} not supported for custom fields yet", operator)
                ));
            }
        };
        
        Ok(condition)
    }
    
    fn build_operator_condition(
        &mut self,
        column: &str,
        operator: &FilterOperator,
        value: &serde_json::Value,
        _field_type: Option<&FieldType>,
    ) -> Result<String, AppError> {
        let param_placeholder = format!("${}", self.param_counter);
        self.param_counter += 1;
        self.parameters.push(value.clone());
        
        let condition = match operator {
            FilterOperator::Equals => format!("{} = {}", column, param_placeholder),
            FilterOperator::NotEquals => format!("{} != {}", column, param_placeholder),
            FilterOperator::Contains => format!("{} ILIKE '%' || {} || '%'", column, param_placeholder),
            FilterOperator::StartsWith => format!("{} ILIKE {} || '%'", column, param_placeholder),
            FilterOperator::EndsWith => format!("{} ILIKE '%' || {}", column, param_placeholder),
            FilterOperator::GreaterThan => format!("{} > {}", column, param_placeholder),
            FilterOperator::LessThan => format!("{} < {}", column, param_placeholder),
            FilterOperator::GreaterEqual => format!("{} >= {}", column, param_placeholder),
            FilterOperator::LessEqual => format!("{} <= {}", column, param_placeholder),
            FilterOperator::IsEmpty => format!("{} IS NULL OR {} = ''", column, column),
            FilterOperator::IsNotEmpty => format!("{} IS NOT NULL AND {} != ''", column, column),
            FilterOperator::In => {
                if let serde_json::Value::Array(arr) = value {
                    let mut placeholders = Vec::new();
                    for item in arr {
                        let placeholder = format!("${}", self.param_counter);
                        self.param_counter += 1;
                        self.parameters.push(item.clone());
                        placeholders.push(placeholder);
                    }
                    format!("{} IN ({})", column, placeholders.join(", "))
                } else {
                    return Err(AppError::ValidationError("IN operator requires array value".to_string()));
                }
            }
            FilterOperator::NotIn => {
                if let serde_json::Value::Array(arr) = value {
                    let mut placeholders = Vec::new();
                    for item in arr {
                        let placeholder = format!("${}", self.param_counter);
                        self.param_counter += 1;
                        self.parameters.push(item.clone());
                        placeholders.push(placeholder);
                    }
                    format!("{} NOT IN ({})", column, placeholders.join(", "))
                } else {
                    return Err(AppError::ValidationError("NOT IN operator requires array value".to_string()));
                }
            }
            FilterOperator::Between => {
                if let serde_json::Value::Array(arr) = value {
                    if arr.len() != 2 {
                        return Err(AppError::ValidationError("BETWEEN operator requires array with 2 values".to_string()));
                    }
                    let placeholder1 = format!("${}", self.param_counter);
                    self.param_counter += 1;
                    self.parameters.push(arr[0].clone());

                    let placeholder2 = format!("${}", self.param_counter);
                    self.param_counter += 1;
                    self.parameters.push(arr[1].clone());
                    
                    format!("{} BETWEEN {} AND {}", column, placeholder1, placeholder2)
                } else {
                    return Err(AppError::ValidationError("BETWEEN operator requires array value".to_string()));
                }
            }
            FilterOperator::After => format!("{} > {}", column, param_placeholder),
            FilterOperator::Before => format!("{} < {}", column, param_placeholder),
        };
        
        Ok(condition)
    }
}

impl ContactFilterService {
    /// Filter contacts with complex nested conditions
    pub async fn filter_contacts(
        pool: &PgPool,
        filter_request: ContactFilterRequest,
    ) -> Result<ContactFilterResponse, AppError> {
        let start_time = std::time::Instant::now();

        // Validate the filter request
        filter_request.validate().map_err(|e| {
            tracing::warn!("Contact filter validation failed: {:?}", e);
            AppError::ValidationError(e.to_string())
        })?;

        // Build the query
        let mut query_builder = QueryBuilder::new();
        let (sql_query, parameters) = query_builder.build_filter_query(&filter_request)?;

        tracing::info!("Generated SQL query: {}", sql_query);
        tracing::debug!("Query parameters: {:?}", parameters);

        // Execute the main query
        let contacts = Self::execute_filter_query(pool, &sql_query, &parameters).await?;

        // Get total count for pagination
        let total_count = Self::get_total_count(pool, &filter_request).await?;

        // Create pagination info
        let pagination = PaginationInfo::new(filter_request.page, filter_request.limit, total_count);

        // Create filter summary
        let execution_time = start_time.elapsed().as_millis() as u64;
        let filter_summary = Self::create_filter_summary(&filter_request, execution_time);

        tracing::info!(
            "Filter executed successfully: {} contacts found in {}ms",
            contacts.len(),
            execution_time
        );

        Ok(ContactFilterResponse {
            success: true,
            data: contacts,
            pagination,
            total_count,
            filter_summary,
        })
    }

    async fn execute_filter_query(
        pool: &PgPool,
        sql_query: &str,
        parameters: &[serde_json::Value],
    ) -> Result<Vec<ContactSummary>, AppError> {
        let mut query = sqlx::query_as::<_, ContactSummaryRow>(sql_query);

        // Bind parameters - convert to appropriate types
        for param in parameters {
            match param {
                serde_json::Value::String(s) => query = query.bind(s),
                serde_json::Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        query = query.bind(i);
                    } else if let Some(f) = n.as_f64() {
                        query = query.bind(f);
                    } else {
                        query = query.bind(n.to_string());
                    }
                }
                serde_json::Value::Bool(b) => query = query.bind(b),
                serde_json::Value::Null => query = query.bind(Option::<String>::None),
                _ => query = query.bind(param.to_string()),
            }
        }

        let rows = query.fetch_all(pool).await.map_err(|e| {
            tracing::error!("Error executing filter query: {}", e);
            AppError::DatabaseError(e)
        })?;

        // Convert rows to ContactSummary
        let contacts = rows.into_iter().map(|row| ContactSummary {
            id: row.id,
            first_name: row.first_name,
            last_name: row.last_name,
            full_name: row.full_name,
            email: row.email,
            phone: row.phone,
            company: row.company,
            job_title: row.job_title,
            lead_status: row.lead_status,
            created_at: row.created_at,
            updated_at: row.updated_at,
            custom_fields: row.custom_fields.map(|json| {
                serde_json::from_value(json).unwrap_or_default()
            }),
        }).collect();

        Ok(contacts)
    }

    async fn get_total_count(
        pool: &PgPool,
        filter_request: &ContactFilterRequest,
    ) -> Result<u64, AppError> {
        let mut query_builder = QueryBuilder::new();

        // Build count query (similar to main query but with COUNT)
        let mut count_query = String::from(
            r#"
            SELECT COUNT(DISTINCT c.id)
            FROM contacts c
            LEFT JOIN contact_custom_values ccv ON c.id = ccv.contact_id
            LEFT JOIN custom_fields cf ON ccv.custom_field_id = cf.id
            "#
        );

        // Build WHERE clause
        if !filter_request.conditions.is_empty() {
            let root_filter = FilterNode::Group {
                logic: filter_request.logic.clone(),
                conditions: filter_request.conditions.clone(),
            };

            let where_clause = query_builder.build_where_clause(&root_filter)?;
            count_query.push_str(&format!(" WHERE c.is_active = true AND ({})", where_clause));
        } else {
            count_query.push_str(" WHERE c.is_active = true");
        }

        let mut query = sqlx::query_scalar::<_, i64>(&count_query);

        // Bind parameters - convert to appropriate types
        for param in &query_builder.parameters {
            match param {
                serde_json::Value::String(s) => query = query.bind(s),
                serde_json::Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        query = query.bind(i);
                    } else if let Some(f) = n.as_f64() {
                        query = query.bind(f);
                    } else {
                        query = query.bind(n.to_string());
                    }
                }
                serde_json::Value::Bool(b) => query = query.bind(b),
                serde_json::Value::Null => query = query.bind(Option::<String>::None),
                _ => query = query.bind(param.to_string()),
            }
        }

        let count = query.fetch_one(pool).await.map_err(|e| {
            tracing::error!("Error executing count query: {}", e);
            AppError::DatabaseError(e)
        })?;

        Ok(count as u64)
    }

    fn create_filter_summary(
        filter_request: &ContactFilterRequest,
        execution_time_ms: u64,
    ) -> FilterSummary {
        let mut fields_used = Vec::new();
        let mut custom_fields_used = Vec::new();
        let total_conditions = Self::count_conditions(&filter_request.conditions);

        Self::extract_fields_from_conditions(&filter_request.conditions, &mut fields_used, &mut custom_fields_used);

        // Remove duplicates and sort
        fields_used.sort();
        fields_used.dedup();
        custom_fields_used.sort();
        custom_fields_used.dedup();

        FilterSummary {
            total_conditions,
            fields_used,
            custom_fields_used,
            execution_time_ms,
        }
    }

    fn count_conditions(conditions: &[FilterNode]) -> u32 {
        let mut count = 0;
        for condition in conditions {
            match condition {
                FilterNode::Condition { .. } => count += 1,
                FilterNode::Group { conditions, .. } => {
                    count += Self::count_conditions(conditions);
                }
            }
        }
        count
    }

    fn extract_fields_from_conditions(
        conditions: &[FilterNode],
        fields_used: &mut Vec<String>,
        custom_fields_used: &mut Vec<String>,
    ) {
        for condition in conditions {
            match condition {
                FilterNode::Condition { field, .. } => {
                    if STANDARD_FIELDS.contains(&field.as_str()) {
                        fields_used.push(field.clone());
                    } else {
                        custom_fields_used.push(field.clone());
                    }
                }
                FilterNode::Group { conditions, .. } => {
                    Self::extract_fields_from_conditions(conditions, fields_used, custom_fields_used);
                }
            }
        }
    }
}

// Helper struct for database row mapping
#[derive(sqlx::FromRow)]
struct ContactSummaryRow {
    id: uuid::Uuid,
    first_name: String,
    last_name: String,
    full_name: String,
    email: String,
    phone: Option<String>,
    company: Option<String>,
    job_title: Option<String>,
    lead_status: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    custom_fields: Option<serde_json::Value>,
}
