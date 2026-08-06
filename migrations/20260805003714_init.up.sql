CREATE TABLE users (
    id BINARY(16) NOT NULL,
    role VARCHAR(64) NOT NULL,
    deleted_at TIMESTAMP NULL DEFAULT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  
    PRIMARY KEY (id),
    KEY idx_role (role),
    KEY idx_deleted_at (deleted_at)
);

CREATE TABLE user_identities (
    id BINARY(16) NOT NULL,
    user_id BINARY(16) NOT NULL,
    provider_type VARCHAR(64) NOT NULL,
    provider_key VARCHAR(256) NOT NULL,
    provider_data JSON DEFAULT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    PRIMARY KEY (id),
    UNIQUE KEY uniq_provider_identity (provider_type, provider_key),
    
    CONSTRAINT fk_identity_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE user_verification_codes (
    id INT AUTO_INCREMENT PRIMARY KEY,
    phone VARCHAR(20) NOT NULL,
    code VARCHAR(6) NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    KEY idx_phone_code (phone, code)
);

CREATE TABLE specialists (
    user_id BINARY(16) NOT NULL,
    schedule JSON NOT NULL,

    PRIMARY KEY (user_id),
    
    CONSTRAINT fk_specialist_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE profiles (
    user_id BINARY(16) NOT NULL,
    first_name VARCHAR(128) NOT NULL,
    last_name VARCHAR(128) DEFAULT NULL,
    avatar_url VARCHAR(512) DEFAULT NULL,
    bio VARCHAR(512) DEFAULT NULL,

    PRIMARY KEY (user_id),
    
    CONSTRAINT fk_profile_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE services (
    id BINARY(16) NOT NULL,
    specialist_id BINARY(16) NOT NULL,
    name VARCHAR(100) NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    duration INT NOT NULL,
    is_active TINYINT(1) DEFAULT '1',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  
    PRIMARY KEY (id),
    KEY idx_specialist_active (specialist_id, is_active),
    
    CONSTRAINT fk_service_specialist FOREIGN KEY (specialist_id) REFERENCES specialists (user_id) ON DELETE CASCADE
);

CREATE TABLE appointments (
    id BINARY(16) NOT NULL,
    specialist_id BINARY(16) NOT NULL,
    client_id BINARY(16) NOT NULL,
    date DATE NOT NULL,
    time TIME NOT NULL,
    duration INT NOT NULL,
    status ENUM('pending', 'confirmed', 'completed', 'cancelled', 'no_show') NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  
    PRIMARY KEY (id),
    KEY idx_specialist_date_time (specialist_id, date, time),
    KEY idx_client_date (client_id, date),
    KEY idx_status (status),
    
    CONSTRAINT fk_appointment_client FOREIGN KEY (client_id) REFERENCES users (id) ON DELETE CASCADE,
    CONSTRAINT fk_appointment_specialist FOREIGN KEY (specialist_id) REFERENCES specialists (user_id) ON DELETE CASCADE
);

CREATE TABLE appointment_services (
    appointment_id BINARY(16) NOT NULL,
    service_id BINARY(16) NOT NULL,
    price_at_booking DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  
    PRIMARY KEY (appointment_id, service_id),
    
    CONSTRAINT fk_appointment_services_appointment FOREIGN KEY (appointment_id) REFERENCES appointments (id) ON DELETE CASCADE,
    CONSTRAINT fk_appointment_services_service FOREIGN KEY (service_id) REFERENCES services (id)
);
