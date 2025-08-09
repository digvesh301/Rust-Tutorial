-- Sample data for Survey Application
-- This file contains sample data to demonstrate the table relationships

-- Insert default roles
INSERT INTO roles (id, name, description, permissions, is_system_role) VALUES
('550e8400-e29b-41d4-a716-446655440001', 'super_admin', 'Full system administrator', 
 '["system:*", "org:*", "survey:*", "user:*"]', true),
('550e8400-e29b-41d4-a716-446655440002', 'org_admin', 'Organization administrator', 
 '["org:manage", "survey:*", "user:manage"]', true),
('550e8400-e29b-41d4-a716-446655440003', 'survey_creator', 'Can create and manage surveys', 
 '["survey:create", "survey:manage", "survey:view"]', true),
('550e8400-e29b-41d4-a716-446655440004', 'survey_manager', 'Can manage assigned surveys', 
 '["survey:manage", "survey:view"]', true),
('550e8400-e29b-41d4-a716-446655440005', 'respondent', 'Can respond to surveys', 
 '["survey:respond", "survey:view"]', true),
('550e8400-e29b-41d4-a716-446655440006', 'viewer', 'Read-only access', 
 '["survey:view"]', true);

-- Insert sample organizations
INSERT INTO organizations (id, name, description, website, email, is_active) VALUES
('660e8400-e29b-41d4-a716-446655440001', 'TechCorp Inc.', 'Technology consulting company', 
 'https://techcorp.com', 'contact@techcorp.com', true),
('660e8400-e29b-41d4-a716-446655440002', 'EduInstitute', 'Educational research institute', 
 'https://eduinstitute.org', 'info@eduinstitute.org', true),
('660e8400-e29b-41d4-a716-446655440003', 'HealthCare Plus', 'Healthcare services provider', 
 'https://healthcareplus.com', 'support@healthcareplus.com', true);

-- Insert sample users
INSERT INTO users (id, organization_id, email, username, password_hash, first_name, last_name, is_active, is_verified) VALUES
('770e8400-e29b-41d4-a716-446655440001', '660e8400-e29b-41d4-a716-446655440001', 
 'admin@techcorp.com', 'admin_tech', '$2b$12$hash1', 'John', 'Admin', true, true),
('770e8400-e29b-41d4-a716-446655440002', '660e8400-e29b-41d4-a716-446655440001', 
 'creator@techcorp.com', 'creator_tech', '$2b$12$hash2', 'Jane', 'Creator', true, true),
('770e8400-e29b-41d4-a716-446655440003', '660e8400-e29b-41d4-a716-446655440002', 
 'researcher@eduinstitute.org', 'researcher_edu', '$2b$12$hash3', 'Bob', 'Researcher', true, true),
('770e8400-e29b-41d4-a716-446655440004', '660e8400-e29b-41d4-a716-446655440003', 
 'manager@healthcareplus.com', 'manager_health', '$2b$12$hash4', 'Alice', 'Manager', true, true),
('770e8400-e29b-41d4-a716-446655440005', NULL, 
 'respondent@example.com', 'respondent1', '$2b$12$hash5', 'Charlie', 'User', true, true);

-- Insert organization memberships
INSERT INTO organization_members (organization_id, user_id, member_type, invited_by) VALUES
('660e8400-e29b-41d4-a716-446655440001', '770e8400-e29b-41d4-a716-446655440001', 'owner', NULL),
('660e8400-e29b-41d4-a716-446655440001', '770e8400-e29b-41d4-a716-446655440002', 'admin', '770e8400-e29b-41d4-a716-446655440001'),
('660e8400-e29b-41d4-a716-446655440002', '770e8400-e29b-41d4-a716-446655440003', 'owner', NULL),
('660e8400-e29b-41d4-a716-446655440003', '770e8400-e29b-41d4-a716-446655440004', 'owner', NULL);

-- Assign roles to users
INSERT INTO user_roles (user_id, role_id, organization_id, assigned_by) VALUES
-- John Admin gets org_admin role for TechCorp
('770e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440002', 
 '660e8400-e29b-41d4-a716-446655440001', NULL),
-- Jane Creator gets survey_creator role for TechCorp
('770e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440003', 
 '660e8400-e29b-41d4-a716-446655440001', '770e8400-e29b-41d4-a716-446655440001'),
-- Bob Researcher gets survey_creator role for EduInstitute
('770e8400-e29b-41d4-a716-446655440003', '550e8400-e29b-41d4-a716-446655440003', 
 '660e8400-e29b-41d4-a716-446655440002', NULL),
-- Alice Manager gets survey_manager role for HealthCare Plus
('770e8400-e29b-41d4-a716-446655440004', '550e8400-e29b-41d4-a716-446655440004', 
 '660e8400-e29b-41d4-a716-446655440003', NULL),
-- Charlie gets respondent role (global)
('770e8400-e29b-41d4-a716-446655440005', '550e8400-e29b-41d4-a716-446655440005', 
 NULL, NULL);

-- Insert sample surveys
INSERT INTO surveys (id, organization_id, created_by, title, description, is_public, allow_anonymous) VALUES
('880e8400-e29b-41d4-a716-446655440001', '660e8400-e29b-41d4-a716-446655440001', 
 '770e8400-e29b-41d4-a716-446655440002', 'Employee Satisfaction Survey', 
 'Annual survey to measure employee satisfaction and engagement', false, false),
('880e8400-e29b-41d4-a716-446655440002', '660e8400-e29b-41d4-a716-446655440002', 
 '770e8400-e29b-41d4-a716-446655440003', 'Student Learning Experience', 
 'Survey to understand student learning preferences and outcomes', true, true),
('880e8400-e29b-41d4-a716-446655440003', '660e8400-e29b-41d4-a716-446655440003', 
 '770e8400-e29b-41d4-a716-446655440004', 'Patient Feedback Survey', 
 'Collect feedback on healthcare services and patient experience', false, true);

-- Insert sample questions
INSERT INTO questions (id, survey_id, question_text, question_type, options, is_required, order_index) VALUES
-- Questions for Employee Satisfaction Survey
('990e8400-e29b-41d4-a716-446655440001', '880e8400-e29b-41d4-a716-446655440001', 
 'How satisfied are you with your current role?', 'single_choice', 
 '["Very Satisfied", "Satisfied", "Neutral", "Dissatisfied", "Very Dissatisfied"]', true, 1),
('990e8400-e29b-41d4-a716-446655440002', '880e8400-e29b-41d4-a716-446655440001', 
 'What aspects of your job do you enjoy most?', 'multiple_choice', 
 '["Team collaboration", "Learning opportunities", "Work-life balance", "Compensation", "Recognition"]', false, 2),
('990e8400-e29b-41d4-a716-446655440003', '880e8400-e29b-41d4-a716-446655440001', 
 'Any additional comments or suggestions?', 'text', NULL, false, 3),

-- Questions for Student Learning Experience
('990e8400-e29b-41d4-a716-446655440004', '880e8400-e29b-41d4-a716-446655440002', 
 'Rate the overall quality of instruction', 'rating', 
 '{"min": 1, "max": 5, "labels": {"1": "Poor", "5": "Excellent"}}', true, 1),
('990e8400-e29b-41d4-a716-446655440005', '880e8400-e29b-41d4-a716-446655440002', 
 'Would you recommend this course to other students?', 'boolean', NULL, true, 2),

-- Questions for Patient Feedback Survey
('990e8400-e29b-41d4-a716-446655440006', '880e8400-e29b-41d4-a716-446655440003', 
 'How would you rate your overall experience?', 'rating', 
 '{"min": 1, "max": 10, "labels": {"1": "Very Poor", "10": "Excellent"}}', true, 1),
('990e8400-e29b-41d4-a716-446655440007', '880e8400-e29b-41d4-a716-446655440003', 
 'What can we improve?', 'text', NULL, false, 2);

-- Insert survey permissions
INSERT INTO survey_permissions (survey_id, user_id, permission_type, granted_by) VALUES
-- John Admin can manage Employee Satisfaction Survey
('880e8400-e29b-41d4-a716-446655440001', '770e8400-e29b-41d4-a716-446655440001', 
 'manage', '770e8400-e29b-41d4-a716-446655440002'),
-- Charlie can respond to Student Learning Experience (public survey)
('880e8400-e29b-41d4-a716-446655440002', '770e8400-e29b-41d4-a716-446655440005', 
 'respond', '770e8400-e29b-41d4-a716-446655440003');

-- Insert sample survey responses
INSERT INTO survey_responses (id, survey_id, respondent_id, is_complete, submitted_at) VALUES
('aa0e8400-e29b-41d4-a716-446655440001', '880e8400-e29b-41d4-a716-446655440002', 
 '770e8400-e29b-41d4-a716-446655440005', true, NOW()),
('aa0e8400-e29b-41d4-a716-446655440002', '880e8400-e29b-41d4-a716-446655440002', 
 NULL, true, NOW()); -- Anonymous response

-- Insert sample question responses
INSERT INTO question_responses (survey_response_id, question_id, answer_number, answer_boolean) VALUES
-- Charlie's responses to Student Learning Experience
('aa0e8400-e29b-41d4-a716-446655440001', '990e8400-e29b-41d4-a716-446655440004', 4, NULL),
('aa0e8400-e29b-41d4-a716-446655440001', '990e8400-e29b-41d4-a716-446655440005', NULL, true),
-- Anonymous responses
('aa0e8400-e29b-41d4-a716-446655440002', '990e8400-e29b-41d4-a716-446655440004', 5, NULL),
('aa0e8400-e29b-41d4-a716-446655440002', '990e8400-e29b-41d4-a716-446655440005', NULL, true);
