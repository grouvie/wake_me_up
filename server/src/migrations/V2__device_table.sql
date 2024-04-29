CREATE TABLE IF NOT EXISTS device
(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    name VARCHAR(255),
    mac_address VARCHAR(255),
    CONSTRAINT fk_device_user FOREIGN KEY (user_id)
        REFERENCES "user" (id) ON UPDATE NO ACTION ON DELETE NO ACTION
);
