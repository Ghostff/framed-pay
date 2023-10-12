CREATE TABLE jobs
(
    id                  UUID                         NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()), -- Automatic incremental ID
    job_type            VARCHAR(255)                 NOT NULL,                                          -- Type or type of the task
    name                VARCHAR(255),                                                                   -- Name or type of the task
    payload             JSONB                        NOT NULL,                                          -- Payload or data for the task, using JSON format
    status              VARCHAR(50) DEFAULT 'queued' NOT NULL,                                          -- Status (queued, processing, completed, failed, etc.)
    priority            SMALLINT    DEFAULT 100      NOT NULL,                                          -- Priority level (lower numbers might mean higher priority)
    retries             SMALLINT    DEFAULT 0        NOT NULL,                                          -- Number of retries performed for the task
    max_retries         SMALLINT    DEFAULT 0        NOT NULL,                                          -- Maximum retries allowed before marked as failed
    error_message       TEXT,                                                                           -- Error message if the task failed
    created_at          TIMESTAMPTZ DEFAULT NOW(),                                                      -- Timestamp when the job was created
    updated_at          TIMESTAMPTZ DEFAULT NOW(),                                                      -- Timestamp when the job was last updated
    started_at          TIMESTAMPTZ,                                                                    -- Timestamp when the job started processing
    completed_at        TIMESTAMPTZ,                                                                    -- Timestamp when the job was completed (either success or failure)
    retry_after_minutes SMALLINT    DEFAULT 0        NOT NULL,                                          -- The delay (in minutes before next) retry
    ready_after         TIMESTAMPTZ DEFAULT NOW()                                                       -- Delay time before retry
);
