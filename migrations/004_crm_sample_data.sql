-- Sample data for CRM system
-- This file contains sample contacts, custom field values, tags, and activities

-- =============================================
-- SAMPLE CONTACTS
-- =============================================
INSERT INTO contacts (id, first_name, last_name, email, phone, company, job_title, city, state, country, lead_source, lead_status, notes) VALUES
('c0001-0000-0000-0000-000000000001', 'John', 'Smith', 'john.smith@techcorp.com', '+1-555-0101', 'TechCorp Solutions', 'CTO', 'San Francisco', 'CA', 'USA', 'website', 'qualified', 'Interested in enterprise survey solutions'),
('c0001-0000-0000-0000-000000000002', 'Sarah', 'Johnson', 'sarah.j@healthplus.com', '+1-555-0102', 'HealthPlus Medical', 'Director of Operations', 'Boston', 'MA', 'USA', 'referral', 'proposal', 'Looking for patient feedback system'),
('c0001-0000-0000-0000-000000000003', 'Michael', 'Chen', 'mchen@edutech.org', '+1-555-0103', 'EduTech Institute', 'Research Manager', 'Austin', 'TX', 'USA', 'conference', 'new', 'Met at EdTech conference 2024'),
('c0001-0000-0000-0000-000000000004', 'Emily', 'Davis', 'emily.davis@retailco.com', '+1-555-0104', 'RetailCo Inc', 'Marketing Manager', 'Chicago', 'IL', 'USA', 'linkedin', 'contacted', 'Interested in customer satisfaction surveys'),
('c0001-0000-0000-0000-000000000005', 'David', 'Wilson', 'dwilson@startup.io', '+1-555-0105', 'StartupIO', 'Founder', 'Seattle', 'WA', 'USA', 'website', 'negotiation', 'Early stage startup, price sensitive'),
('c0001-0000-0000-0000-000000000006', 'Lisa', 'Brown', 'lisa.brown@consulting.com', '+1-555-0106', 'Brown Consulting', 'Senior Consultant', 'New York', 'NY', 'USA', 'email_campaign', 'closed_won', 'Signed annual contract'),
('c0001-0000-0000-0000-000000000007', 'Robert', 'Taylor', 'rtaylor@manufacturing.com', '+1-555-0107', 'Taylor Manufacturing', 'Quality Manager', 'Detroit', 'MI', 'USA', 'trade_show', 'closed_lost', 'Went with competitor'),
('c0001-0000-0000-0000-000000000008', 'Jennifer', 'Anderson', 'j.anderson@nonprofit.org', '+1-555-0108', 'Community Nonprofit', 'Program Director', 'Portland', 'OR', 'USA', 'referral', 'qualified', 'Needs volunteer feedback system');

-- =============================================
-- SAMPLE CONTACT TAGS
-- =============================================
INSERT INTO contact_tags (id, name, color, description) VALUES
('ct001-0000-0000-0000-000000000001', 'Enterprise', '#007bff', 'Large enterprise clients'),
('ct001-0000-0000-0000-000000000002', 'SMB', '#28a745', 'Small to medium business'),
('ct001-0000-0000-0000-000000000003', 'Startup', '#ffc107', 'Early stage startups'),
('ct001-0000-0000-0000-000000000004', 'Healthcare', '#dc3545', 'Healthcare industry'),
('ct001-0000-0000-0000-000000000005', 'Education', '#6f42c1', 'Educational institutions'),
('ct001-0000-0000-0000-000000000006', 'Technology', '#17a2b8', 'Technology companies'),
('ct001-0000-0000-0000-000000000007', 'Hot Lead', '#fd7e14', 'High priority prospects'),
('ct001-0000-0000-0000-000000000008', 'VIP', '#e83e8c', 'VIP customers');

-- =============================================
-- CONTACT TAG ASSIGNMENTS
-- =============================================
INSERT INTO contact_tag_assignments (contact_id, tag_id) VALUES
-- John Smith - TechCorp (Enterprise, Technology, Hot Lead)
('c0001-0000-0000-0000-000000000001', 'ct001-0000-0000-0000-000000000001'),
('c0001-0000-0000-0000-000000000001', 'ct001-0000-0000-0000-000000000006'),
('c0001-0000-0000-0000-000000000001', 'ct001-0000-0000-0000-000000000007'),
-- Sarah Johnson - HealthPlus (Enterprise, Healthcare, Hot Lead)
('c0001-0000-0000-0000-000000000002', 'ct001-0000-0000-0000-000000000001'),
('c0001-0000-0000-0000-000000000002', 'ct001-0000-0000-0000-000000000004'),
('c0001-0000-0000-0000-000000000002', 'ct001-0000-0000-0000-000000000007'),
-- Michael Chen - EduTech (SMB, Education)
('c0001-0000-0000-0000-000000000003', 'ct001-0000-0000-0000-000000000002'),
('c0001-0000-0000-0000-000000000003', 'ct001-0000-0000-0000-000000000005'),
-- Emily Davis - RetailCo (Enterprise)
('c0001-0000-0000-0000-000000000004', 'ct001-0000-0000-0000-000000000001'),
-- David Wilson - StartupIO (Startup, Technology, Hot Lead)
('c0001-0000-0000-0000-000000000005', 'ct001-0000-0000-0000-000000000003'),
('c0001-0000-0000-0000-000000000005', 'ct001-0000-0000-0000-000000000006'),
('c0001-0000-0000-0000-000000000005', 'ct001-0000-0000-0000-000000000007'),
-- Lisa Brown - Brown Consulting (SMB, VIP)
('c0001-0000-0000-0000-000000000006', 'ct001-0000-0000-0000-000000000002'),
('c0001-0000-0000-0000-000000000006', 'ct001-0000-0000-0000-000000000008');

-- =============================================
-- SAMPLE CUSTOM FIELD VALUES
-- =============================================
INSERT INTO contact_custom_values (contact_id, custom_field_id, value, value_json, value_number, value_date, value_boolean) VALUES
-- John Smith custom values
('c0001-0000-0000-0000-000000000001', 'cf001-0000-0000-0000-000000000001', 'https://linkedin.com/in/johnsmith', NULL, NULL, NULL, NULL),
('c0001-0000-0000-0000-000000000001', 'cf001-0000-0000-0000-000000000002', '50000000', NULL, 50000000, NULL, NULL),
('c0001-0000-0000-0000-000000000001', 'cf001-0000-0000-0000-000000000003', 'Technology', NULL, NULL, NULL, NULL),
('c0001-0000-0000-0000-000000000001', 'cf001-0000-0000-0000-000000000004', 'Email', NULL, NULL, NULL, NULL),
('c0001-0000-0000-0000-000000000001', 'cf001-0000-0000-0000-000000000005', 'true', NULL, NULL, NULL, true),
('c0001-0000-0000-0000-000000000001', 'cf001-0000-0000-0000-000000000006', '2024-01-15', NULL, NULL, '2024-01-15', NULL),
('c0001-0000-0000-0000-000000000001', 'cf001-0000-0000-0000-000000000007', '["JavaScript", "Python", "Go"]', '["JavaScript", "Python", "Go"]', NULL, NULL, NULL),

-- Sarah Johnson custom values
('c0001-0000-0000-0000-000000000002', 'cf001-0000-0000-0000-000000000001', 'https://linkedin.com/in/sarahjohnson', NULL, NULL, NULL, NULL),
('c0001-0000-0000-0000-000000000002', 'cf001-0000-0000-0000-000000000002', '25000000', NULL, 25000000, NULL, NULL),
('c0001-0000-0000-0000-000000000002', 'cf001-0000-0000-0000-000000000003', 'Healthcare', NULL, NULL, NULL, NULL),
('c0001-0000-0000-0000-000000000002', 'cf001-0000-0000-0000-000000000004', 'Phone', NULL, NULL, NULL, NULL),
('c0001-0000-0000-0000-000000000002', 'cf001-0000-0000-0000-000000000005', 'false', NULL, NULL, NULL, false),
('c0001-0000-0000-0000-000000000002', 'cf001-0000-0000-0000-000000000006', '2024-02-20', NULL, NULL, '2024-02-20', NULL),

-- Michael Chen custom values
('c0001-0000-0000-0000-000000000003', 'cf001-0000-0000-0000-000000000003', 'Education', NULL, NULL, NULL, NULL),
('c0001-0000-0000-0000-000000000003', 'cf001-0000-0000-0000-000000000004', 'Email', NULL, NULL, NULL, NULL),
('c0001-0000-0000-0000-000000000003', 'cf001-0000-0000-0000-000000000005', 'true', NULL, NULL, NULL, true),
('c0001-0000-0000-0000-000000000003', 'cf001-0000-0000-0000-000000000007', '["Python", "Java"]', '["Python", "Java"]', NULL, NULL, NULL),

-- David Wilson custom values
('c0001-0000-0000-0000-000000000005', 'cf001-0000-0000-0000-000000000002', '1000000', NULL, 1000000, NULL, NULL),
('c0001-0000-0000-0000-000000000005', 'cf001-0000-0000-0000-000000000003', 'Technology', NULL, NULL, NULL, NULL),
('c0001-0000-0000-0000-000000000005', 'cf001-0000-0000-0000-000000000004', 'LinkedIn', NULL, NULL, NULL, NULL),
('c0001-0000-0000-0000-000000000005', 'cf001-0000-0000-0000-000000000007', '["JavaScript", "Rust"]', '["JavaScript", "Rust"]', NULL, NULL, NULL);

-- =============================================
-- SAMPLE CONTACT ACTIVITIES
-- =============================================
INSERT INTO contact_activities (contact_id, activity_type, subject, description, activity_date, duration_minutes, status) VALUES
-- John Smith activities
('c0001-0000-0000-0000-000000000001', 'call', 'Initial Discovery Call', 'Discussed survey requirements and current pain points', '2024-01-10 14:00:00', 45, 'completed'),
('c0001-0000-0000-0000-000000000001', 'email', 'Follow-up Email', 'Sent product demo and pricing information', '2024-01-12 09:30:00', NULL, 'completed'),
('c0001-0000-0000-0000-000000000001', 'meeting', 'Product Demo', 'Demonstrated survey platform features', '2024-01-18 15:00:00', 60, 'completed'),
('c0001-0000-0000-0000-000000000001', 'task', 'Prepare Proposal', 'Create custom proposal for TechCorp', '2024-01-25 10:00:00', NULL, 'scheduled'),

-- Sarah Johnson activities
('c0001-0000-0000-0000-000000000002', 'call', 'Qualification Call', 'Qualified healthcare survey needs', '2024-02-05 11:00:00', 30, 'completed'),
('c0001-0000-0000-0000-000000000002', 'email', 'Healthcare Case Studies', 'Sent relevant healthcare customer success stories', '2024-02-07 16:20:00', NULL, 'completed'),
('c0001-0000-0000-0000-000000000002', 'meeting', 'Technical Review', 'Reviewed HIPAA compliance and security features', '2024-02-15 13:00:00', 90, 'completed'),
('c0001-0000-0000-0000-000000000002', 'note', 'Proposal Sent', 'Sent formal proposal with healthcare-specific features', '2024-02-22 14:30:00', NULL, 'completed'),

-- Michael Chen activities
('c0001-0000-0000-0000-000000000003', 'note', 'Conference Contact', 'Met at EdTech 2024 conference, booth #245', '2024-03-01 12:00:00', NULL, 'completed'),
('c0001-0000-0000-0000-000000000003', 'email', 'Conference Follow-up', 'Thank you email with product information', '2024-03-03 08:15:00', NULL, 'completed'),
('c0001-0000-0000-0000-000000000003', 'task', 'Schedule Demo', 'Need to schedule product demonstration', '2024-03-10 09:00:00', NULL, 'scheduled'),

-- Emily Davis activities
('c0001-0000-0000-0000-000000000004', 'email', 'LinkedIn Connection', 'Connected on LinkedIn, sent introduction', '2024-02-28 10:45:00', NULL, 'completed'),
('c0001-0000-0000-0000-000000000004', 'call', 'Initial Contact', 'Brief introduction call about customer surveys', '2024-03-05 14:30:00', 20, 'completed'),

-- David Wilson activities
('c0001-0000-0000-0000-000000000005', 'call', 'Startup Consultation', 'Discussed startup-friendly pricing options', '2024-01-20 16:00:00', 40, 'completed'),
('c0001-0000-0000-0000-000000000005', 'email', 'Startup Package Info', 'Sent information about startup discount program', '2024-01-22 11:20:00', NULL, 'completed'),
('c0001-0000-0000-0000-000000000005', 'meeting', 'Negotiation Meeting', 'Discussed pricing and contract terms', '2024-02-10 15:30:00', 75, 'completed'),

-- Lisa Brown activities
('c0001-0000-0000-0000-000000000006', 'call', 'Consulting Needs Assessment', 'Assessed survey needs for consulting clients', '2024-01-05 13:00:00', 35, 'completed'),
('c0001-0000-0000-0000-000000000006', 'email', 'Contract Sent', 'Sent annual service contract', '2024-01-15 09:00:00', NULL, 'completed'),
('c0001-0000-0000-0000-000000000006', 'note', 'Contract Signed', 'Annual contract signed and executed', '2024-01-20 14:00:00', NULL, 'completed'),
('c0001-0000-0000-0000-000000000006', 'meeting', 'Onboarding Session', 'Initial platform setup and training', '2024-01-25 10:00:00', 120, 'completed'),

-- Robert Taylor activities
('c0001-0000-0000-0000-000000000007', 'call', 'Manufacturing Survey Discussion', 'Discussed quality control survey needs', '2024-02-01 15:00:00', 25, 'completed'),
('c0001-0000-0000-0000-000000000007', 'email', 'Proposal Sent', 'Sent manufacturing-specific proposal', '2024-02-05 10:30:00', NULL, 'completed'),
('c0001-0000-0000-0000-000000000007', 'note', 'Lost to Competitor', 'Chose competitor due to lower pricing', '2024-02-20 16:45:00', NULL, 'completed'),

-- Jennifer Anderson activities
('c0001-0000-0000-0000-000000000008', 'call', 'Nonprofit Consultation', 'Discussed volunteer feedback survey needs', '2024-03-01 11:00:00', 30, 'completed'),
('c0001-0000-0000-0000-000000000008', 'email', 'Nonprofit Pricing', 'Sent nonprofit discount information', '2024-03-03 14:15:00', NULL, 'completed'),
('c0001-0000-0000-0000-000000000008', 'task', 'Prepare Nonprofit Demo', 'Customize demo for nonprofit use cases', '2024-03-12 09:00:00', NULL, 'scheduled');
