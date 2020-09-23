CREATE TABLE auths(
    user_id VARCHAR NOT NULL,
    token VARCHAR NOT NULL UNIQUE,
    PRIMARY KEY (user_id, token),
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
)
