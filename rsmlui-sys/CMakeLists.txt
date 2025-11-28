cmake_minimum_required(VERSION 3.22)
project(rsmlui_sys_lib)

include(cmake/CPM.cmake)

set(CARGO_TARGET_DIR ${CMAKE_SOURCE_DIR}/target)
set(CXX_BRIDGE_DIR ${CARGO_TARGET_DIR}/cxxbridge)

add_library(${PROJECT_NAME} STATIC dummy.cpp)

configure_file(
    ${CMAKE_CURRENT_SOURCE_DIR}/rsmlui_config.h.in
    ${CMAKE_CURRENT_BINARY_DIR}/rsmlui_config.h
)

CPMAddPackage(
    NAME RmlUi
    GIT_TAG 6.1
    GITHUB_REPOSITORY mikke89/RmlUi
    OPTIONS "BUILD_SHARED_LIBS OFF" "RMLUI_STATIC_LIB" "RMLUI_FONT_ENGINE none" "RMLUI_DEBUG" "RMLUI_NO_THIRDPARTY_CONTAINERS" "RMLUI_DISABLE_INCLUDE_WINDOWS" "UNICODE" "_UNICODE" "WIN32_LEAN_AND_MEAN"
)

target_link_libraries(${PROJECT_NAME} PUBLIC RmlUi::RmlUi)

if(WIN32)
    target_link_libraries(${PROJECT_NAME} PUBLIC user32)
    set_target_properties(
        ${PROJECT_NAME} 
        PROPERTIES
        MSVC_RUNTIME_LIBRARY "MultiThreadedDLL"
    )
endif()

target_include_directories(${PROJECT_NAME} PUBLIC 
    ${CMAKE_CURRENT_SOURCE_DIR}/include
    ${CXX_BRIDGE_DIR}/rust
    ${CMAKE_SOURCE_DIR}
)