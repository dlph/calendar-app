local_resource(
    'cargo build',
    dir='src-tauri',
    cmd='cargo build',
    deps=['src', 'src-tauri'],
    auto_init=False,
    trigger_mode=TRIGGER_MODE_MANUAL,
    allow_parallel=False,
)

local_resource(
    'cargo tauri dev',
    dir='src-tauri',
    serve_cmd='cargo tauri dev',
    auto_init=False,
    trigger_mode=TRIGGER_MODE_MANUAL,
    allow_parallel=False,
)
