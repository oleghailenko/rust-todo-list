CREATE TABLE items (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT REFERENCES users(id),
    list_id BIGINT REFERENCES lists(id),
    description VARCHAR(255) NOT NULL,
    done BOOL
);