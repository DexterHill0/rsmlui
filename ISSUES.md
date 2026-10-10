## Issues/Feedback for RmlUI based on what I encountered in this project

- Enums should declare a type (log type enum should be `: int` otherwise the int-type of the enum changes per-platform)
- Functions could return error codes instead of booleans for better debugging
- void* user data pointer in callbacks
