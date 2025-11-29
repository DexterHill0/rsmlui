include(cmake/CPM.cmake)

set(SKIP_INSTALL_ALL ON CACHE BOOL "" FORCE)
set(BUILD_SHARED_LIBS OFF CACHE BOOL "" FORCE)
set(BUILD_TESTING OFF CACHE BOOL "" FORCE)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})

CPMAddPackage(
    NAME zlib
    GITHUB_REPOSITORY madler/zlib
    GIT_TAG v1.3.1
    OPTIONS "ZLIB_USE_STATIC_LIBS ON"
)

add_library(ZLIB::ZLIB ALIAS zlibstatic)
target_include_directories(zlibstatic PUBLIC ${zlib_SOURCE_DIR} ${zlib_BINARY_DIR})

CPMAddPackage(
    NAME libpng
    GITHUB_REPOSITORY pnggroup/libpng
    VERSION 1.6.50
    OPTIONS "PNG_SHARED OFF"
)

target_link_libraries(png_static PRIVATE ZLIB::ZLIB)

add_library(PNG::PNG ALIAS png_static)
target_include_directories(png_static PUBLIC ${libpng_SOURCE_DIR} ${libpng_BINARY_DIR})

CPMAddPackage("gh:google/brotli#v1.1.0")

add_library(BROTLI::common ALIAS brotlicommon)
add_library(BROTLI::dec    ALIAS brotlidec)
add_library(BROTLI::enc    ALIAS brotlienc)

target_include_directories(brotlicommon PUBLIC ${brotli_SOURCE_DIR}/c/include)
target_include_directories(brotlidec    PUBLIC ${brotli_SOURCE_DIR}/c/include)
target_include_directories(brotlienc    PUBLIC ${brotli_SOURCE_DIR}/c/include)

CPMAddPackage(
    NAME bzip2
    URL https://gitlab.com/bzip2/bzip2/-/archive/master/bzip2-master.zip
    OPTIONS "ENABLE_STATIC_LIB 1"
)

add_library(BZIP2::BZIP2 ALIAS bz2_static)
target_include_directories(bz2_static PUBLIC ${bzip2_SOURCE_DIR})

CPMAddPackage(
    NAME harfbuzz
    GITHUB_REPOSITORY harfbuzz/harfbuzz
    GIT_TAG 11.2.1
)

add_library(Harfbuzz::Harfbuzz ALIAS harfbuzz)
target_include_directories(harfbuzz PUBLIC ${harfbuzz_SOURCE_DIR}/src)


CPMAddPackage(
    NAME freetype
    GITHUB_REPOSITORY freetype/freetype
    GIT_TAG VER-2-13-3
)

target_link_libraries(freetype PRIVATE
    ZLIB::ZLIB
    PNG::PNG
    BROTLI::dec
    BROTLI::common
    BROTLI::enc
    BZIP2::BZIP2
    Harfbuzz::Harfbuzz
)

add_library(Freetype::Freetype ALIAS freetype)

target_include_directories(freetype PUBLIC ${freetype_SOURCE_DIR}/include)

target_link_libraries(${RSMLUI_SYS_LIB_NAME} PUBLIC freetype)