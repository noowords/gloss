-- ============================================================
-- NOTIFICATIONS
-- ============================================================

DROP TABLE IF EXISTS notification_deliveries;
DROP TABLE IF EXISTS notifications;


-- ============================================================
-- OTP / AUTH
-- ============================================================

DROP TABLE IF EXISTS otp_challenges;
DROP TABLE IF EXISTS auth_sessions;


-- ============================================================
-- FAVORITES / REVIEWS
-- ============================================================

DROP TABLE IF EXISTS favorite_specialists;
DROP TABLE IF EXISTS reviews;


-- ============================================================
-- APPOINTMENTS
-- ============================================================

DROP TABLE IF EXISTS appointment_services;
DROP TABLE IF EXISTS appointments;


-- ============================================================
-- SPECIALIST LEAVE / TIME OFF
-- ============================================================

DROP TABLE IF EXISTS specialist_time_off;
DROP TABLE IF EXISTS specialist_leave_allowances;


-- ============================================================
-- SPECIALIST SCHEDULE OVERRIDES
-- ============================================================

DROP TABLE IF EXISTS specialist_schedule_override_intervals;
DROP TABLE IF EXISTS specialist_schedule_overrides;


-- ============================================================
-- SPECIALIST SCHEDULES
-- ============================================================

DROP TABLE IF EXISTS specialist_schedule_intervals;
DROP TABLE IF EXISTS specialist_schedules;


-- ============================================================
-- SPECIALIST SERVICES
-- ============================================================

DROP TABLE IF EXISTS specialist_services;


-- ============================================================
-- SERVICES
-- ============================================================

DROP TABLE IF EXISTS salon_services;
DROP TABLE IF EXISTS service_addon_rules;
DROP TABLE IF EXISTS services;


-- ============================================================
-- SPECIALISTS
-- ============================================================

DROP TABLE IF EXISTS specialists;


-- ============================================================
-- SALON SCHEDULE
-- ============================================================

DROP TABLE IF EXISTS salon_schedule_exception_intervals;
DROP TABLE IF EXISTS salon_schedule_exceptions;
DROP TABLE IF EXISTS salon_schedule_intervals;


-- ============================================================
-- SALON WORK POLICY
-- ============================================================

DROP TABLE IF EXISTS salon_work_policies;


-- ============================================================
-- SALONS
-- ============================================================

DROP TABLE IF EXISTS salons;


-- ============================================================
-- USERS
-- ============================================================

DROP TABLE IF EXISTS profiles;
DROP TABLE IF EXISTS user_roles;
DROP TABLE IF EXISTS user_providers;
DROP TABLE IF EXISTS users;
