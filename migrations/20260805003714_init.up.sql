CREATE TABLE users (
  id BINARY(16) NOT NULL,
  role VARCHAR(50) NOT NULL,
  phone VARCHAR(20) DEFAULT NULL,
  deleted_at TIMESTAMP NULL DEFAULT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  
  PRIMARY KEY (id),
  KEY idx_phone (phone),
  KEY idx_role (role),
  KEY idx_deleted_at (deleted_at)
);

CREATE TABLE masters (
  user_id BINARY(16) NOT NULL,
  schedule JSON NOT NULL,
  
  PRIMARY KEY (user_id),
  CONSTRAINT fk_master_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE profiles (
  user_id BINARY(16) NOT NULL,
  first_name VARCHAR(100) NOT NULL,
  last_name VARCHAR(100) DEFAULT NULL,
  avatar_url VARCHAR(500) DEFAULT NULL,
  bio TEXT,
  
  PRIMARY KEY (user_id),
  CONSTRAINT fk_profile_user FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE services (
  id BINARY(16) NOT NULL,
  master_id BINARY(16) NOT NULL,
  name VARCHAR(100) NOT NULL,
  price DECIMAL(10,2) NOT NULL,
  duration INT NOT NULL,
  is_active TINYINT(1) DEFAULT '1',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  
  PRIMARY KEY (id),
  KEY idx_master_active (master_id, is_active),
  CONSTRAINT fk_service_master FOREIGN KEY (master_id) REFERENCES masters (user_id) ON DELETE CASCADE
);

CREATE TABLE appointments (
  id BINARY(16) NOT NULL,
  master_id BINARY(16) NOT NULL,
  client_id BINARY(16) NOT NULL,
  date DATE NOT NULL,
  time TIME NOT NULL,
  status ENUM('pending','confirmed','cancelled','completed') NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  
  PRIMARY KEY (id),
  UNIQUE KEY uniq_master_datetime (master_id, date, time),
  KEY idx_master_date (master_id, date),
  KEY idx_client_date (client_id, date),
  KEY idx_date_time (date, time),
  KEY idx_status (status),
  CONSTRAINT fk_appointment_client FOREIGN KEY (client_id) REFERENCES users (id) ON DELETE CASCADE,
  CONSTRAINT fk_appointment_master FOREIGN KEY (master_id) REFERENCES masters (user_id) ON DELETE CASCADE
);

CREATE TABLE appointment_services (
  appointment_id BINARY(16) NOT NULL,
  service_id BINARY(16) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  
  PRIMARY KEY (appointment_id, service_id),
  KEY idx_service (service_id),
  CONSTRAINT fk_appointment_services_appointment FOREIGN KEY (appointment_id) REFERENCES appointments (id) ON DELETE CASCADE,
  CONSTRAINT fk_appointment_services_service FOREIGN KEY (service_id) REFERENCES services (id)
);
