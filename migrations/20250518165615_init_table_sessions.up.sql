CREATE TABLE table_sessions (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	table_id UUID NOT NULL,
	is_active BOOLEAN NOT NULL DEFAULT TRUE,

	created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
