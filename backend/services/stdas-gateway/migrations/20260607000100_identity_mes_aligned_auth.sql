CREATE TABLE IF NOT EXISTS c_users (
    id varchar(40) PRIMARY KEY,
    username varchar(255) NOT NULL,
    passwd varchar(255) NOT NULL,
    fname varchar(255),
    lname varchar(255),
    notes varchar(255),
    schedule varchar(255),
    language varchar(255),
    badge varchar(255),
    site_id varchar(40),
    department varchar(255),
    is_system_reserved char(1) NOT NULL DEFAULT 'N',
    is_system_manager char(1) NOT NULL DEFAULT 'N',
    person_code varchar(20) NOT NULL DEFAULT '0',
    creat_user varchar(40) NOT NULL DEFAULT 'system',
    creat_date timestamp with time zone NOT NULL DEFAULT now(),
    lm_user varchar(40) NOT NULL DEFAULT 'system',
    lm_date timestamp with time zone NOT NULL DEFAULT now(),
    leader_name varchar(255),
    duty_name varchar(255),
    new_account_source varchar(40) DEFAULT 'STDAS',
    is_on_job char(1) DEFAULT 'Y',
    depart_date timestamp with time zone,
    leader_emp_no varchar(40),
    sync_failure_type varchar(1) DEFAULT '0',
    CONSTRAINT uk_c_users_username UNIQUE (username),
    CONSTRAINT ck_c_users_system_reserved CHECK (is_system_reserved IN ('Y', 'N')),
    CONSTRAINT ck_c_users_system_manager CHECK (is_system_manager IN ('Y', 'N')),
    CONSTRAINT ck_c_users_on_job CHECK (is_on_job IN ('Y', 'N'))
);

CREATE INDEX IF NOT EXISTS c_users_id01 ON c_users (username, site_id);
CREATE INDEX IF NOT EXISTS c_users_id02 ON c_users (person_code);

COMMENT ON TABLE c_users IS 'STDAS test-department user master. Table and field names follow MES c_users semantics where useful; scope is intentionally smaller than factory MES.';
COMMENT ON COLUMN c_users.id IS 'MES c_users.id naming convention: internal UUID string.';
COMMENT ON COLUMN c_users.username IS 'MES c_users.username naming convention: login/account.';
COMMENT ON COLUMN c_users.passwd IS 'MES c_users.passwd naming convention: Argon2id PHC hash only.';
COMMENT ON COLUMN c_users.person_code IS 'MES c_users.person_code naming convention: employee number.';
COMMENT ON COLUMN c_users.site_id IS 'MES c_users.site_id naming convention: default site.';
COMMENT ON COLUMN c_users.is_on_job IS 'MES c_users.is_on_job naming convention: Y active, N departed.';

CREATE TABLE IF NOT EXISTS c_roles (
    id integer NOT NULL,
    site_id varchar(40),
    role_name varchar(255),
    is_system_reserved char(1) NOT NULL DEFAULT 'N',
    authorized_level varchar(10) NOT NULL DEFAULT '1',
    roles_uuid varchar(40) NOT NULL,
    decentralization char(1) NOT NULL DEFAULT 'N',
    create_user varchar(40) NOT NULL DEFAULT 'system',
    create_time timestamp with time zone NOT NULL DEFAULT now(),
    lm_user varchar(40) NOT NULL DEFAULT 'system',
    lm_time timestamp with time zone NOT NULL DEFAULT now(),
    CONSTRAINT pk_c_roles PRIMARY KEY (id, roles_uuid),
    CONSTRAINT uk_c_roles_id UNIQUE (id),
    CONSTRAINT uk_c_roles UNIQUE (site_id, role_name),
    CONSTRAINT ck_c_roles_system_reserved CHECK (is_system_reserved IN ('Y', 'N')),
    CONSTRAINT ck_c_roles_decentralization CHECK (decentralization IN ('Y', 'N'))
);

CREATE TABLE IF NOT EXISTS c_user_rl (
    role_id integer NOT NULL,
    is_system_reserved char(1) NOT NULL DEFAULT 'N',
    user_id varchar(40) NOT NULL,
    creat_user varchar(40),
    creat_date timestamp with time zone DEFAULT now(),
    lm_user varchar(40),
    lm_date timestamp with time zone,
    CONSTRAINT pk_c_user_rl PRIMARY KEY (user_id, role_id),
    CONSTRAINT fk_c_user_rl_user FOREIGN KEY (user_id) REFERENCES c_users (id),
    CONSTRAINT fk_c_user_rl_role FOREIGN KEY (role_id) REFERENCES c_roles (id),
    CONSTRAINT ck_c_user_rl_system_reserved CHECK (is_system_reserved IN ('Y', 'N'))
);

CREATE TABLE IF NOT EXISTS r_user_session (
    session_uuid varchar(40) PRIMARY KEY,
    user_id varchar(40) NOT NULL,
    access_token_hash varchar(64) NOT NULL,
    token_type varchar(20) NOT NULL DEFAULT 'Bearer',
    expires_at timestamp with time zone NOT NULL,
    is_revoked char(1) NOT NULL DEFAULT 'N',
    create_time timestamp with time zone NOT NULL DEFAULT now(),
    revoked_time timestamp with time zone,
    CONSTRAINT uk_r_user_session_token_hash UNIQUE (access_token_hash),
    CONSTRAINT fk_r_user_session_user FOREIGN KEY (user_id) REFERENCES c_users (id),
    CONSTRAINT ck_r_user_session_revoked CHECK (is_revoked IN ('Y', 'N'))
);

CREATE INDEX IF NOT EXISTS r_user_session_id01 ON r_user_session (user_id, expires_at);

COMMENT ON TABLE r_user_session IS 'Current STDAS access sessions; stores token hash only.';
