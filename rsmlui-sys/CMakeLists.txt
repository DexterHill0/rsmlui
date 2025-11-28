cmake_minimum_required(VERSION 3.22)
project(rsmlui-sys-lib)

add_subdirectory(RmlUi RmlUi-build EXCLUDE_FROM_ALL)

if(NOT DEFINED RMLUI_OUTPUT_NAME)
    message(FATAL_ERROR "RMLUI_OUTPUT_NAME must be set")
endif()

if(NOT DEFINED RSMLUI_SYS_LIB_NAME)
    message(FATAL_ERROR "RSMLUI_SYS_LIB_NAME must be set")
endif()

if(NOT DEFINED RSMLUI_CXX_INCLUDE_DIR)
    message(FATAL_ERROR "RSMLUI_CXX_INCLUDE_DIR must be set")
endif()

# build the cpp files so theyre included in intellisense
file(GLOB_RECURSE RSMLUI_CXX_SRCS
    "${CMAKE_CURRENT_SOURCE_DIR}/src/cxx/*.cpp"
)

add_library(${RSMLUI_SYS_LIB_NAME} STATIC ${RSMLUI_CXX_SRCS})

target_link_libraries(${RSMLUI_SYS_LIB_NAME} PUBLIC RmlUi::RmlUi)

target_include_directories(${RSMLUI_SYS_LIB_NAME} 
    PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/RmlUi/Include
        ${RSMLUI_CXX_INCLUDE_DIR}
        ${CMAKE_CURRENT_SOURCE_DIR}/include
        ${CMAKE_CURRENT_BINARY_DIR}
)


if(WIN32)
    target_link_libraries(${RSMLUI_SYS_LIB_NAME} PUBLIC user32)
    set_target_properties(
        ${RSMLUI_SYS_LIB_NAME}
        PROPERTIES
        MSVC_RUNTIME_LIBRARY "MultiThreadedDLL"
    )
endif()
