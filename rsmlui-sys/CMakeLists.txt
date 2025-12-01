cmake_minimum_required(VERSION 3.22)
project(rsmlui-sys-lib)

set(CMAKE_CXX_STANDARD 20)

if(NOT DEFINED RSMLUI_SYS_LIB_NAME)
    message(FATAL_ERROR "RSMLUI_SYS_LIB_NAME must be set")
endif()

if(NOT DEFINED RSMLUI_DUMMY_SOURCES_DIR)
    message(FATAL_ERROR "RSMLUI_DUMMY_SOURCES_DIR must be set")
endif()


set(SOURCES_NORMAL "")
cmake_path(NORMAL_PATH RSMLUI_DUMMY_SOURCES_DIR OUTPUT_VARIABLE SOURCES_NORMAL)
# create a dummy cpp file with all the headers for intellisense purposes
file(GLOB_RECURSE RSMLUI_DUMMY_SOURCES "${SOURCES_NORMAL}/*.cpp")

file(GLOB_RECURSE RSMLUI_CXX_SOURCES "${CMAKE_SOURCE_DIR}/src/cxx/*.cpp")

# make a dummy library for the intellisense
add_library(${RSMLUI_SYS_LIB_NAME} STATIC ${RSMLUI_DUMMY_SOURCES} ${RSMLUI_CXX_SOURCES})

# include freetype if requested. variable is set in build.rs
if (DEFINED RMLUI_FONT_ENGINE AND "${RMLUI_FONT_ENGINE}" STREQUAL "freetype")
    include(cmake/FetchFreetype.cmake)

    set(Freetype_ROOT "${freetype_BINARY_DIR}" CACHE PATH "")
    set(FREETYPE_INCLUDE_DIRS "${freetype_SOURCE_DIR}/include" CACHE PATH "")
endif() 

add_subdirectory(RmlUi RmlUi-build EXCLUDE_FROM_ALL)

if(NOT DEFINED RMLUI_OUTPUT_NAME)
    message(FATAL_ERROR "RMLUI_OUTPUT_NAME must be set")
endif()

if(NOT DEFINED RSMLUI_CXX_INCLUDE_DIR)
    message(FATAL_ERROR "RSMLUI_CXX_INCLUDE_DIR must be set")
endif()

target_link_libraries(${RSMLUI_SYS_LIB_NAME} PUBLIC RmlUi::RmlUi)

# define all our includes so `compile_commands.json` contains everything required for intellisense
target_include_directories(${RSMLUI_SYS_LIB_NAME} 
    PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/RmlUi/Include
        ${CMAKE_CURRENT_SOURCE_DIR}/RmlUi/Backends
        ${RSMLUI_CXX_INCLUDE_DIR}
        ${CMAKE_CURRENT_SOURCE_DIR}/src/include
        ${CMAKE_CURRENT_BINARY_DIR}
)