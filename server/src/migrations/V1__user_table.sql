CREATE TABLE IF NOT EXISTS "user"
(
    id SERIAL PRIMARY KEY,
    username VARCHAR(255),
    password_hash VARCHAR(255)
);

/*
    We create a demo user named "Grouvie" with the password "password".
    This user is used for demonstration and testing purposes.
*/
INSERT INTO "user" (username, password_hash) VALUES ('Grouvie', '$argon2d$v=19$m=16,t=2,p=1$NE0zdGdrRmVsU3NuSjNxWQ$spAkm1R/xrV1ZlvvMPYkFA');