--! users
SELECT *
FROM users
ORDER BY :order_by DESC
OFFSET :offset
LIMIT :limit;

--! user_count
SELECT COUNT(*)
FROM users;

--! users_by_permission
SELECT *
FROM users
WHERE permission = :permission::permission
ORDER BY :order_by DESC
OFFSET :offset
LIMIT :limit;

--! users_by_email
SELECT *
FROM users
WHERE email = :email
LIMIT 1;

--! users_by_username
SELECT *
FROM users
WHERE username = :username
LIMIT 1;

--! update_user_permission
UPDATE users
SET 
    permission = :permission::permission,
    updated_at = NOW()
WHERE id = :id
RETURNING *;

--! update_user
UPDATE users
SET 
    email = COALESCE(:email, email),
    username = COALESCE(:username, username),
    first_name = COALESCE(:first_name, first_name),
    last_name = COALESCE(:last_name, last_name),
    updated_at = NOW()
WHERE id = :id
RETURNING *;

--! create_user
INSERT INTO users (
    email,
    username,
    first_name,
    last_name,
    permission
)
VALUES (
    :email,
    :username,
    :first_name,
    :last_name,
    :permission::permission
)
RETURNING *;

--! bulk_insert_users
INSERT INTO users (
    email,
    username,
    first_name,
    last_name,
    permission
)
SELECT 
    (value->>'email')::text,
    (value->>'username')::text,  
    (value->>'first_name')::text,
    (value->>'last_name')::text,
    (value->>'permission')::permission
FROM json_array_elements(:users::json)
RETURNING *;

--! delete_user
DELETE FROM users
WHERE id = :id
RETURNING *;

--! search_users
SELECT *
FROM users
WHERE 
    (LOWER(username) LIKE LOWER(:search) || '%' OR
    LOWER(email) LIKE LOWER(:search) || '%' OR
    LOWER(first_name) LIKE LOWER(:search) || '%' OR
    LOWER(last_name) LIKE LOWER(:search) || '%')
ORDER BY :order_by DESC
OFFSET :offset
LIMIT :limit;