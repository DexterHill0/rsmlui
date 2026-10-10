# Includes the external packages that the optional interfaces need, such as GLFW for the GLFW system interface.

if(DEFINED RSMLUI_SYSTEM_GLFW)
    if(NOT DEFINED RSMLUI_GLFW_INCLUDE_DIRS)
        message(FATAL_ERROR "RSMLUI_GLFW_INCLUDE_DIRS must be set when RSMLUI_SYSTEM_GLFW is on")
    endif()

    target_include_directories(${RSMLUI_SYS_LIB_NAME} PRIVATE ${RSMLUI_GLFW_INCLUDE_DIRS})
endif()
