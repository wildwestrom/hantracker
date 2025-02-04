CREATE TABLE user_profile (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL DEFAULT "default",
    test_exists BOOLEAN NOT NULL DEFAULT false,
    test_progress INTEGER NOT NULL DEFAULT 0
);

INSERT INTO user_profile (id) VALUES (0);

CREATE TABLE user_profile_characters (
    profile INTEGER NOT NULL REFERENCES user_profile(id)
        ON DELETE CASCADE,
    char INTEGER NOT NULL,
    known BOOLEAN NOT NULL
);

CREATE TABLE user_profile_raw_test_text (
    profile INTEGER NOT NULL REFERENCES user_profile(id)
        ON DELETE CASCADE ON UPDATE CASCADE,
    text TEXT NOT NULL DEFAULT ""
);

INSERT INTO user_profile_raw_test_text (profile, text) VALUES (0, "");