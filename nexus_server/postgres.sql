-- 启用 uuid-ossp 扩展，用于生成 UUID
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 用户表 (users)
CREATE TABLE users (
    user_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),  -- 使用 UUID 作为主键
    username VARCHAR(50) UNIQUE NOT NULL,                 -- 用户名唯一
    password_hash TEXT NOT NULL,                          -- 存储密码哈希
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP        -- 用户创建时间
);

-- 消息表 (messages)
CREATE TABLE messages (
    message_id SERIAL PRIMARY KEY,                        -- 自增消息 ID
    sender_id UUID NOT NULL,                              -- 发送者 ID（外键）
    receiver_id UUID NOT NULL,                                     -- 接收者 ID（用户或群组）
    content TEXT NOT NULL,                                -- 消息内容
    message_type VARCHAR(10) CHECK (message_type IN ('text', 'audio', 'video')),  -- 消息类型约束
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,        -- 消息发送时间
    CONSTRAINT fk_sender FOREIGN KEY (sender_id) REFERENCES users (user_id) ON DELETE CASCADE,
    CONSTRAINT fk_receiver FOREIGN KEY (receiver_id) REFERENCES users (user_id) ON DELETE SET NULL
);

-- 群组表 (groups)
CREATE TABLE groups (
    group_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),  -- 群组 ID 使用 UUID
    group_name VARCHAR(100) NOT NULL,                      -- 群组名称
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP         -- 群组创建时间
);

-- 群组成员表 (group_members)
CREATE TABLE group_members (
    group_id UUID NOT NULL,                                -- 群组 ID
    user_id UUID NOT NULL,                                 -- 用户 ID
    PRIMARY KEY (group_id, user_id),                       -- 复合主键
    CONSTRAINT fk_group FOREIGN KEY (group_id) REFERENCES groups (group_id) ON DELETE CASCADE,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users (user_id) ON DELETE CASCADE
);
