-- ============================================================
-- GLOSS API V1 - UP MIGRATION
-- ============================================================


-- ============================================================
-- USERS
-- ============================================================

CREATE TABLE users (
    id BINARY(16) PRIMARY KEY,

    status VARCHAR(32) NOT NULL,

    created_at DATETIME(6) NOT NULL,
    updated_at DATETIME(6) NOT NULL,

    KEY idx_users_status (status)
);


CREATE TABLE user_providers (
    id BINARY(16) PRIMARY KEY,
    user_id BINARY(16) NOT NULL,

    type VARCHAR(32) NOT NULL,
    subject VARCHAR(255) NOT NULL,

    verified_at DATETIME(6) NULL,
    created_at DATETIME(6) NOT NULL,
    updated_at DATETIME(6) NOT NULL,

    CONSTRAINT fk_user_providers_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE,

    UNIQUE KEY uq_user_providers_type_subject (
        type,
        subject
    ),

    KEY idx_user_providers_user (user_id)
);


CREATE TABLE user_roles (
    user_id BINARY(16) NOT NULL,

    name VARCHAR(32) NOT NULL,

    created_at DATETIME(6) NOT NULL,

    PRIMARY KEY (
        user_id,
        name
    ),

    CONSTRAINT fk_user_roles_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);


CREATE TABLE user_profiles (
    user_id BINARY(16) PRIMARY KEY,

    first_name VARCHAR(128) NOT NULL,
    last_name VARCHAR(128) NULL,
    avatar_url VARCHAR(2048) NULL,

    created_at DATETIME(6) NOT NULL,
    updated_at DATETIME(6) NOT NULL,

    CONSTRAINT fk_user_profiles_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);


-- ============================================================
-- SALONS
-- ============================================================

CREATE TABLE salons (
    id BINARY(16) PRIMARY KEY,

    code VARCHAR(32) NOT NULL,
    name VARCHAR(128) NOT NULL,
    city VARCHAR(128) NOT NULL,
    address VARCHAR(255) NULL,
    timezone VARCHAR(64) NOT NULL,
    status VARCHAR(32) NOT NULL,

    created_at DATETIME(6) NOT NULL,
    updated_at DATETIME(6) NOT NULL,

    UNIQUE KEY uq_salons_code (code),

    KEY idx_salons_city (city),
    KEY idx_salons_status (status)
);


CREATE TABLE salon_work_policies (
    salon_id BINARY(16) PRIMARY KEY,

    weekly_work_minutes SMALLINT UNSIGNED NOT NULL,
    booking_step_minutes SMALLINT UNSIGNED NOT NULL,
    booking_horizon_days SMALLINT UNSIGNED NOT NULL,
    default_annual_leave_minutes INT UNSIGNED NOT NULL,

    created_at DATETIME(6) NOT NULL,
    updated_at DATETIME(6) NOT NULL,

    CONSTRAINT fk_salon_work_policies_salon
        FOREIGN KEY (salon_id)
        REFERENCES salons(id)
        ON DELETE CASCADE
);


-- ============================================================
-- SALON SCHEDULE
-- ============================================================

CREATE TABLE salon_schedule_intervals (
    id BINARY(16) PRIMARY KEY,
    salon_id BINARY(16) NOT NULL,

    weekday TINYINT UNSIGNED NOT NULL,
    starts_at TIME NOT NULL,
    ends_at TIME NOT NULL,

    CONSTRAINT fk_salon_schedule_intervals_salon
        FOREIGN KEY (salon_id)
        REFERENCES salons(id)
        ON DELETE CASCADE,

    UNIQUE KEY uq_salon_schedule_interval (
        salon_id,
        weekday,
        starts_at,
        ends_at
    ),

    KEY idx_salon_schedule_weekday (
        salon_id,
        weekday
    )
);


CREATE TABLE salon_schedule_exceptions (
    id BINARY(16) PRIMARY KEY,
    salon_id BINARY(16) NOT NULL,

    date DATE NOT NULL,
    type VARCHAR(32) NOT NULL,
    reason VARCHAR(255) NULL,

    created_at DATETIME(6) NOT NULL,
    updated_at DATETIME(6) NOT NULL,

    CONSTRAINT fk_salon_schedule_exceptions_salon
        FOREIGN KEY (salon_id)
        REFERENCES salons(id)
        ON DELETE CASCADE,

    UNIQUE KEY uq_salon_schedule_exception (
        salon_id,
        date
    )
);


CREATE TABLE salon_schedule_exception_intervals (
    id BINARY(16) PRIMARY KEY,
    exception_id BINARY(16) NOT NULL,

    starts_at TIME NOT NULL,
    ends_at TIME NOT NULL,

    CONSTRAINT fk_salon_schedule_exception_intervals_exception
        FOREIGN KEY (exception_id)
        REFERENCES salon_schedule_exceptions(id)
        ON DELETE CASCADE,

    KEY idx_salon_schedule_exception_intervals_exception (
        exception_id
    )
);


-- ============================================================
-- SPECIALISTS
-- ============================================================

CREATE TABLE specialists (
    id BINARY(16) PRIMARY KEY,
    user_id BINARY(16) NOT NULL,
    salon_id BINARY(16) NOT NULL,

    bio VARCHAR(1024) NULL,
    experience_started_at DATE NULL,
    status VARCHAR(32) NOT NULL,

    created_at DATETIME(6) NOT NULL,
    updated_at DATETIME(6) NOT NULL,

    CONSTRAINT fk_specialists_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE RESTRICT,

    CONSTRAINT fk_specialists_salon
        FOREIGN KEY (salon_id)
        REFERENCES salons(id)
        ON DELETE RESTRICT,

    UNIQUE KEY uq_specialists_user (
        user_id
    ),

    UNIQUE KEY uq_specialists_salon_id (
        salon_id,
        id
    ),

    KEY idx_specialists_salon (salon_id),
    KEY idx_specialists_status (status)
);


-- ============================================================
-- SERVICES
-- ============================================================

CREATE TABLE services (
    id BINARY(16) PRIMARY KEY,

    name VARCHAR(128) NOT NULL,
    description VARCHAR(1024) NULL,
    preview_url VARCHAR(2048) NULL,
    category VARCHAR(64) NOT NULL,
    kind VARCHAR(32) NOT NULL,
    duration_minutes SMALLINT UNSIGNED NOT NULL,
    status VARCHAR(32) NOT NULL,

    created_at DATETIME(6) NOT NULL,
    updated_at DATETIME(6) NOT NULL,

    KEY idx_services_category (category),
    KEY idx_services_kind (kind),
    KEY idx_services_status (status)
);


CREATE TABLE service_addon_rules (
    primary_service_id BINARY(16) NOT NULL,
    addon_service_id BINARY(16) NOT NULL,

    created_at DATETIME(6) NOT NULL,

    PRIMARY KEY (
        primary_service_id,
        addon_service_id
    ),

    CONSTRAINT fk_service_addon_primary
        FOREIGN KEY (primary_service_id)
        REFERENCES services(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_service_addon_addon
        FOREIGN KEY (addon_service_id)
        REFERENCES services(id)
        ON DELETE CASCADE,

    KEY idx_service_addon_addon (
        addon_service_id
    )
);


CREATE TABLE salon_services (
    salon_id BINARY(16) NOT NULL,
    service_id BINARY(16) NOT NULL,

    price DECIMAL(10, 2) NOT NULL,
    status VARCHAR(32) NOT NULL,

    created_at DATETIME(6) NOT NULL,
    updated_at DATETIME(6) NOT NULL,

    PRIMARY KEY (
        salon_id,
        service_id
    ),

    CONSTRAINT fk_salon_services_salon
        FOREIGN KEY (salon_id)
        REFERENCES salons(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_salon_services_service
        FOREIGN KEY (service_id)
        REFERENCES services(id)
        ON DELETE CASCADE,

    KEY idx_salon_services_service (
        service_id
    ),

    KEY idx_salon_services_status (
        salon_id,
        status
    )
);


CREATE TABLE specialist_services (
    specialist_id BINARY(16) NOT NULL,
    salon_id BINARY(16) NOT NULL,
    service_id BINARY(16) NOT NULL,

    created_at DATETIME(6) NOT NULL,

    PRIMARY KEY (
        specialist_id,
        service_id
    ),

    CONSTRAINT fk_specialist_services_specialist
        FOREIGN KEY (
            salon_id,
            specialist_id
        )
        REFERENCES specialists(
            salon_id,
            id
        )
        ON DELETE CASCADE,

    CONSTRAINT fk_specialist_services_salon_service
        FOREIGN KEY (
            salon_id,
            service_id
        )
        REFERENCES salon_services(
            salon_id,
            service_id
        )
        ON DELETE CASCADE,

    KEY idx_specialist_services_salon_service (
        salon_id,
        service_id
    )
);


-- ============================================================
-- SPECIALIST SCHEDULES
-- ============================================================

CREATE TABLE specialist_schedules (
    id BINARY(16) PRIMARY KEY,
    specialist_id BINARY(16) NOT NULL,

    effective_from DATE NOT NULL,
    effective_until DATE NULL,

    created_at DATETIME(6) NOT NULL,
    updated_at DATETIME(6) NOT NULL,

    CONSTRAINT fk_specialist_schedules_specialist
        FOREIGN KEY (specialist_id)
        REFERENCES specialists(id)
        ON DELETE CASCADE,

    UNIQUE KEY uq_specialist_schedule_from (
        specialist_id,
        effective_from
    ),

    KEY idx_specialist_schedule_period (
        specialist_id,
        effective_from,
        effective_until
    )
);


CREATE TABLE specialist_schedule_intervals (
    id BINARY(16) PRIMARY KEY,
    schedule_id BINARY(16) NOT NULL,

    weekday TINYINT UNSIGNED NOT NULL,
    starts_at TIME NOT NULL,
    ends_at TIME NOT NULL,

    CONSTRAINT fk_specialist_schedule_intervals_schedule
        FOREIGN KEY (schedule_id)
        REFERENCES specialist_schedules(id)
        ON DELETE CASCADE,

    UNIQUE KEY uq_specialist_schedule_interval (
        schedule_id,
        weekday,
        starts_at,
        ends_at
    ),

    KEY idx_specialist_schedule_intervals_weekday (
        schedule_id,
        weekday
    )
);


-- ============================================================
-- SPECIALIST SCHEDULE OVERRIDES
-- ============================================================

CREATE TABLE specialist_schedule_overrides (
    id BINARY(16) PRIMARY KEY,
    specialist_id BINARY(16) NOT NULL,

    date DATE NOT NULL,
    reason VARCHAR(255) NULL,

    created_at DATETIME(6) NOT NULL,
    updated_at DATETIME(6) NOT NULL,

    CONSTRAINT fk_specialist_schedule_overrides_specialist
        FOREIGN KEY (specialist_id)
        REFERENCES specialists(id)
        ON DELETE CASCADE,

    UNIQUE KEY uq_specialist_schedule_override (
        specialist_id,
        date
    ),

    KEY idx_specialist_schedule_overrides_date (
        date
    )
);


CREATE TABLE specialist_schedule_override_intervals (
    id BINARY(16) PRIMARY KEY,
    override_id BINARY(16) NOT NULL,

    starts_at TIME NOT NULL,
    ends_at TIME NOT NULL,

    CONSTRAINT fk_specialist_schedule_override_intervals_override
        FOREIGN KEY (override_id)
        REFERENCES specialist_schedule_overrides(id)
        ON DELETE CASCADE,

    KEY idx_specialist_schedule_override_intervals_override (
        override_id
    )
);


-- ============================================================
-- SPECIALIST LEAVE / TIME OFF
-- ============================================================

CREATE TABLE specialist_leave_allowances (
    specialist_id BINARY(16) NOT NULL,

    year SMALLINT UNSIGNED NOT NULL,
    allocated_minutes INT UNSIGNED NOT NULL,

    created_at DATETIME(6) NOT NULL,
    updated_at DATETIME(6) NOT NULL,

    PRIMARY KEY (
        specialist_id,
        year
    ),

    CONSTRAINT fk_specialist_leave_allowances_specialist
        FOREIGN KEY (specialist_id)
        REFERENCES specialists(id)
        ON DELETE CASCADE
);


CREATE TABLE specialist_time_off (
    id BINARY(16) PRIMARY KEY,
    specialist_id BINARY(16) NOT NULL,

    date DATE NOT NULL,
    starts_at TIME NULL,
    ends_at TIME NULL,
    type VARCHAR(32) NOT NULL,
    charged_leave_minutes SMALLINT UNSIGNED NOT NULL,
    reason VARCHAR(255) NULL,

    created_at DATETIME(6) NOT NULL,
    updated_at DATETIME(6) NOT NULL,

    CONSTRAINT fk_specialist_time_off_specialist
        FOREIGN KEY (specialist_id)
        REFERENCES specialists(id)
        ON DELETE CASCADE,

    KEY idx_specialist_time_off_specialist_date (
        specialist_id,
        date
    ),

    KEY idx_specialist_time_off_date (
        date
    )
);


-- ============================================================
-- APPOINTMENTS
--
-- Domain invariants:
--   1 client
--   1 salon
--   1 specialist
--   exactly 1 primary service
--   0..N addons
-- ============================================================

CREATE TABLE appointments (
    id BINARY(16) PRIMARY KEY,
    client_id BINARY(16) NOT NULL,
    salon_id BINARY(16) NOT NULL,
    specialist_id BINARY(16) NOT NULL,

    status VARCHAR(32) NOT NULL,
    starts_at DATETIME(6) NOT NULL,
    ends_at DATETIME(6) NOT NULL,
    total_price_snapshot DECIMAL(10, 2) NOT NULL,
    total_duration_minutes_snapshot SMALLINT UNSIGNED NOT NULL,
    cancellation_reason VARCHAR(255) NULL,

    created_at DATETIME(6) NOT NULL,
    cancelled_at DATETIME(6) NULL,
    updated_at DATETIME(6) NOT NULL,

    CONSTRAINT fk_appointments_client
        FOREIGN KEY (client_id)
        REFERENCES users(id)
        ON DELETE RESTRICT,

    CONSTRAINT fk_appointments_specialist
        FOREIGN KEY (
            salon_id,
            specialist_id
        )
        REFERENCES specialists(
            salon_id,
            id
        )
        ON DELETE RESTRICT,

    KEY idx_appointments_client_starts (
        client_id,
        starts_at
    ),

    KEY idx_appointments_salon_starts (
        salon_id,
        starts_at
    ),

    KEY idx_appointments_specialist_starts (
        specialist_id,
        starts_at
    ),

    KEY idx_appointments_specialist_status_starts (
        specialist_id,
        status,
        starts_at
    ),

    KEY idx_appointments_salon_status_starts (
        salon_id,
        status,
        starts_at
    ),

    KEY idx_appointments_status_starts (
        status,
        starts_at
    )
);


CREATE TABLE appointment_services (
    appointment_id BINARY(16) NOT NULL,
    service_id BINARY(16) NOT NULL,

    role VARCHAR(32) NOT NULL,

    service_name_snapshot VARCHAR(128) NOT NULL,
    price_snapshot DECIMAL(10, 2) NOT NULL,
    duration_minutes_snapshot SMALLINT UNSIGNED NOT NULL,

    PRIMARY KEY (
        appointment_id,
        service_id
    ),

    CONSTRAINT fk_appointment_services_appointment
        FOREIGN KEY (appointment_id)
        REFERENCES appointments(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_appointment_services_service
        FOREIGN KEY (service_id)
        REFERENCES services(id)
        ON DELETE RESTRICT,

    KEY idx_appointment_services_service (
        service_id
    ),

    KEY idx_appointment_services_role (
        appointment_id,
        role
    )
);


-- ============================================================
-- APPOINTMENT REVIEWS
-- ============================================================

CREATE TABLE appointment_reviews (
    id BINARY(16) PRIMARY KEY,
    appointment_id BINARY(16) NOT NULL,

    rating TINYINT UNSIGNED NOT NULL,
    comment VARCHAR(1024) NULL,
    status VARCHAR(32) NOT NULL,

    created_at DATETIME(6) NOT NULL,
    updated_at DATETIME(6) NOT NULL,

    CONSTRAINT fk_appointment_reviews_appointment
        FOREIGN KEY (appointment_id)
        REFERENCES appointments(id)
        ON DELETE RESTRICT,

    UNIQUE KEY uq_appointment_reviews_appointment (
        appointment_id
    ),

    KEY idx_appointment_reviews_status_created (
        status,
        created_at
    )
);


-- ============================================================
-- USER FAVORITES
-- ============================================================

CREATE TABLE user_favorite_specialists (
    user_id BINARY(16) NOT NULL,
    specialist_id BINARY(16) NOT NULL,

    created_at DATETIME(6) NOT NULL,

    PRIMARY KEY (
        user_id,
        specialist_id
    ),

    CONSTRAINT fk_user_favorite_specialists_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_user_favorite_specialists_specialist
        FOREIGN KEY (specialist_id)
        REFERENCES specialists(id)
        ON DELETE CASCADE,

    KEY idx_user_favorite_specialists_specialist (
        specialist_id
    )
);


-- ============================================================
-- AUTH SESSIONS
-- ============================================================

CREATE TABLE auth_sessions (
    id BINARY(16) PRIMARY KEY,
    user_id BINARY(16) NOT NULL,

    refresh_token_hash VARBINARY(64) NOT NULL,
    user_agent VARCHAR(512) NULL,
    ip_address VARBINARY(16) NULL,

    expires_at DATETIME(6) NOT NULL,
    last_used_at DATETIME(6) NULL,
    revoked_at DATETIME(6) NULL,
    created_at DATETIME(6) NOT NULL,

    CONSTRAINT fk_auth_sessions_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE,

    UNIQUE KEY uq_auth_sessions_refresh_token_hash (
        refresh_token_hash
    ),

    KEY idx_auth_sessions_user (
        user_id
    ),

    KEY idx_auth_sessions_expires (
        expires_at
    )
);


-- ============================================================
-- OTP
-- ============================================================

CREATE TABLE otp_challenges (
    id BINARY(16) PRIMARY KEY,

    subject VARCHAR(255) NOT NULL,
    purpose VARCHAR(32) NOT NULL,
    code_hash VARBINARY(64) NOT NULL,
    attempts SMALLINT UNSIGNED NOT NULL,

    expires_at DATETIME(6) NOT NULL,
    verified_at DATETIME(6) NULL,
    consumed_at DATETIME(6) NULL,
    created_at DATETIME(6) NOT NULL,

    KEY idx_otp_challenges_subject (
        subject,
        purpose,
        created_at
    ),

    KEY idx_otp_challenges_expires (
        expires_at
    )
);


-- ============================================================
-- NOTIFICATIONS
--
-- Every notification exists in the in-app notification center.
-- Additional delivery channels are represented separately.
-- ============================================================

CREATE TABLE notifications (
    id BINARY(16) PRIMARY KEY,
    user_id BINARY(16) NOT NULL,

    type VARCHAR(64) NOT NULL,
    title VARCHAR(128) NOT NULL,
    message VARCHAR(1024) NOT NULL,

    read_at DATETIME(6) NULL,
    created_at DATETIME(6) NOT NULL,

    CONSTRAINT fk_notifications_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE,

    KEY idx_notifications_user_read_created (
        user_id,
        read_at,
        created_at
    ),

    KEY idx_notifications_type (
        type
    )
);


CREATE TABLE notification_deliveries (
    id BINARY(16) PRIMARY KEY,
    notification_id BINARY(16) NOT NULL,

    channel VARCHAR(32) NOT NULL,
    status VARCHAR(32) NOT NULL,

    attempted_at DATETIME(6) NULL,
    delivered_at DATETIME(6) NULL,

    created_at DATETIME(6) NOT NULL,
    updated_at DATETIME(6) NOT NULL,

    CONSTRAINT fk_notification_deliveries_notification
        FOREIGN KEY (notification_id)
        REFERENCES notifications(id)
        ON DELETE CASCADE,

    UNIQUE KEY uq_notification_delivery_channel (
        notification_id,
        channel
    ),

    KEY idx_notification_deliveries_status (
        status
    ),

    KEY idx_notification_deliveries_notification (
        notification_id
    )
);
