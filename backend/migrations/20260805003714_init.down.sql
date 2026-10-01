-- ============================================================
-- GLOSS API V1 - DOWN MIGRATION
-- ============================================================

DROP TABLE IF EXISTS notification_deliveries;
DROP TABLE IF EXISTS notifications;

DROP TABLE IF EXISTS otp_challenges;
DROP TABLE IF EXISTS auth_sessions;

DROP TABLE IF EXISTS user_favorite_specialists;

DROP TABLE IF EXISTS appointment_reviews;
DROP TABLE IF EXISTS appointment_services;
DROP TABLE IF EXISTS appointments;

DROP TABLE IF EXISTS specialist_time_off;
DROP TABLE IF EXISTS specialist_leave_allowances;

DROP TABLE IF EXISTS specialist_schedule_override_intervals;
DROP TABLE IF EXISTS specialist_schedule_overrides;

DROP TABLE IF EXISTS specialist_schedule_intervals;
DROP TABLE IF EXISTS specialist_schedules;

DROP TABLE IF EXISTS specialist_services;

DROP TABLE IF EXISTS salon_services;
DROP TABLE IF EXISTS service_addon_rules;
DROP TABLE IF EXISTS services;

DROP TABLE IF EXISTS specialists;

DROP TABLE IF EXISTS salon_schedule_exception_intervals;
DROP TABLE IF EXISTS salon_schedule_exceptions;
DROP TABLE IF EXISTS salon_schedule_intervals;

DROP TABLE IF EXISTS salon_work_policies;
DROP TABLE IF EXISTS salons;

DROP TABLE IF EXISTS user_profiles;
DROP TABLE IF EXISTS user_roles;
DROP TABLE IF EXISTS user_providers;
DROP TABLE IF EXISTS users;