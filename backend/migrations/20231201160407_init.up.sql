create table "user" (
    id uuid not null primary key default gen_random_uuid(),
    name varchar(128) not null unique,
    password varchar(256) not null,
    created_at timestamp with time zone default now(),
    updated_at timestamp with time zone default now()
);


create table survey (
    id uuid not null primary key default gen_random_uuid(),
    title varchar(128) not null unique,
    user_id uuid references "user" (id) on delete cascade not null,
    survey_data jsonb not null,
    created_at timestamp with time zone default now(),
    updated_at timestamp with time zone default now()
);

create table result (
    id uuid not null primary key default gen_random_uuid(),
    survey_id uuid references "survey" (id) on delete cascade not null,
    result_data jsonb not null,
    created_at timestamp with time zone default now(),
    updated_at timestamp with time zone default now()
);

insert into "user" (id, name, password)
values (
        '8b073e53-b0c1-49b8-84de-75c217bb2ba6',
        'User 1',
        '$argon2id$v=19$m=19456,t=2,p=1$yLan33bjG806SrNA+8sJrA$dctf03yoVst+6PqeHIflSOV8vHDk+bASvRCByOeZFoA'
    ),
    (
        '3bbd8257-92e2-4941-97a4-193b78f6debe',
        'User 2',
        '$argon2id$v=19$m=19456,t=2,p=1$AlNEX6OxhaA39SBkNJC4IA$oWftJ9rkepJ/bd66VALigcNY6Qu2YsBN2OZ+utLiX7A'
    ),
    (
        '6e18a45c-3bd7-4292-a106-65f7ed3ec0db',
        'User 3',
        '$argon2id$v=19$m=19456,t=2,p=1$Lpk0qQ/FcUzYcY5GaKlFqw$rQFP4pLT6HLadql259qZsSLm7u24Rl/CD6cj7vxehs4'
    ),
    (
        '3c74ca92-e532-4688-b11b-78647db2d4ff',
        'User 4',
        '$argon2id$v=19$m=19456,t=2,p=1$VlnaPWeF33fALhRWbqgxpg$ak8tBXMVP27Y3+epKKUwUGllT8v7NeNxD5tJeM/O4IE'
    ),
    (
        '93eeb282-b0c5-4a93-ab26-fa78cfca8cb1',
        'User 5',
        '$argon2id$v=19$m=19456,t=2,p=1$zr4gcoc/zVYfone4UfsZiQ$2S93nJJqA/UQeKY7iUGoEt9wbqe+Vd73HPZSFNLl5gA'
    ),
    (
        '3c2ee863-f2d1-433d-b8c1-5228f0a3fe9d',
        'User 6',
        '$argon2id$v=19$m=19456,t=2,p=1$oytC6Lw2kygxqOU5qV7Z7A$RyNHoQYjpaUQLiYrDpQoSPO4OjQHkojs2vUULnci4Jc'
    ),
    (
        '9f93786f-8dab-495d-be02-ddb620f5d2a3',
        'User 7',
        '$argon2id$v=19$m=19456,t=2,p=1$ywdEqGE45WfLr5zDVvas4Q$tQJIHq/XWbHj2MJnKkYUaGDwO+S3+sF06hGPtGkqTX4'
    );

insert into survey (title, user_id, survey_data)
values (
        'Survey 1',
        '8b073e53-b0c1-49b8-84de-75c217bb2ba6',
        '[
            {
                "question": "How do you feel today",
                "type": "single",
                "answers": ["Happy", "Sad", "Neutral"]
            },
            {
                "question": "Whats your favorite color",
                "type": "multiple",
                "answers": ["Red", "Blue", "Green"]
            },
            {
                "question": "Whats your favorite food",
                "type": "input"
            },
            {
                "question": "How do you feel today",
                "type": "single",
                "answers": ["Happy", "Sad", "Neutral"]
            },
            {
                "question": "Whats your favorite color",
                "type": "multiple",
                "answers": ["Red", "Blue", "Green"]
            },
            {
                "question": "Whats your favorite food",
                "type": "input"
            },
             {
                "question": "How do you feel today",
                "type": "single",
                "answers": ["Happy", "Sad", "Neutral"]
            },
            {
                "question": "Whats your favorite color",
                "type": "multiple",
                "answers": ["Red", "Blue", "Green"]
            },
            {
                "question": "Whats your favorite food",
                "type": "input"
            },
            {
                "question": "How do you feel today",
                "type": "single",
                "answers": ["Happy", "Sad", "Neutral"]
            },
            {
                "question": "Whats your favorite color",
                "type": "multiple",
                "answers": ["Red", "Blue", "Green"]
            },
            {
                "question": "Whats your favorite food",
                "type": "input"
            },
             {
                "question": "How do you feel today",
                "type": "single",
                "answers": ["Happy", "Sad", "Neutral"]
            },
            {
                "question": "Whats your favorite color",
                "type": "multiple",
                "answers": ["Red", "Blue", "Green"]
            },
            {
                "question": "Whats your favorite food",
                "type": "input"
            },
            {
                "question": "How do you feel today",
                "type": "single",
                "answers": ["Happy", "Sad", "Neutral"]
            },
            {
                "question": "Whats your favorite color",
                "type": "multiple",
                "answers": ["Red", "Blue", "Green"]
            },
            {
                "question": "Whats your favorite food",
                "type": "input"
            }
        ]'
    ),
    (
        'Survey 2',
        '8b073e53-b0c1-49b8-84de-75c217bb2ba6',
        '[
            {
                "question": "Whats your favorite season",
                "type": "multiple",
                "answers": ["Spring", "Summer", "Autumn", "Winter"]
            },
            {
                "question": "Whats your favorite animal",
                "type": "single",
                "answers": ["Dog", "Cat", "Bird"]
            },
            {
                "question": "Whats your favorite book",
                "type": "input"
            }
        ]'
    ),
    (
        'Survey 3',
        '8b073e53-b0c1-49b8-84de-75c217bb2ba6',
        '[
            {
                "question": "Whats your favorite movie genre",
                "type": "single",
                "answers": ["Action", "Comedy", "Drama"]
            },
            {
                "question": "Whats your favorite song",
                "type": "input"
            },
            {
                "question": "Whats your favorite sport",
                "type": "single",
                "answers": ["Football", "Basketball", "Tennis"]
            }
        ]'
    ),
    (
        'Survey 4',
        '93eeb282-b0c5-4a93-ab26-fa78cfca8cb1',
        '[
            {
                "question": "Whats your favorite country",
                "type": "single",
                "answers": ["USA", "UK", "Australia"]
            },
            {
                "question": "Whats your favorite city",
                "type": "single",
                "answers": ["New York", "London", "Sydney"]
            },
            {
                "question": "Whats your favorite cuisine",
                "type": "single",
                "answers": ["Italian", "Chinese", "Mexican"]
            }
        ]'
    ),
    (
        'Survey 5',
        '93eeb282-b0c5-4a93-ab26-fa78cfca8cb1',
        '[
            {
                "question": "Whats your favorite hobby",
                "type": "single",
                "answers": ["Reading", "Cooking", "Gardening"]
            },
            {
                "question": "Whats your favorite TV show",
                "type": "single",
                "answers": ["Friends", "The Office", "Game of Thrones"]
            },
            {
                "question": "Whats your favorite music genre",
                "type": "single",
                "answers": ["Pop", "Rock", "Jazz"]
            }
        ]'
    ),
    (
        'Survey 6',
        '9f93786f-8dab-495d-be02-ddb620f5d2a3',
        '[
            {
                "question": "Whats your favorite fruit",
                "type": "single",
                "answers": ["Apple", "Banana", "Orange"]
            },
            {
                "question": "Whats your favorite vegetable",
                "type": "single",
                "answers": ["Carrot", "Broccoli", "Pea"]
            },
            {
                "question": "Whats your favorite dessert",
                "type": "single",
                "answers": ["Ice Cream", "Cake", "Pie"]
            }
        ]'
    ),
    (
        'Survey 7',
        '9f93786f-8dab-495d-be02-ddb620f5d2a3',
        '[
            {
                "question": "Whats your favorite drink",
                "type": "single",
                "answers": ["Coffee", "Tea", "Juice"]
            },
            {
                "question": "Whats your favorite car brand",
                "type": "single",
                "answers": ["Toyota", "Ford", "BMW"]
            },
            {
                "question": "Whats your favorite smartphone brand",
                "type": "single",
                "answers": ["Apple", "Samsung", "Google"]
            }
        ]'
    )