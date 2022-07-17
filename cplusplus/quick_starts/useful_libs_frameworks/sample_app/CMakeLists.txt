cmake_minimum_required(VERSION 3.23 FATAL_ERROR)

# ---- Project ----

# Note: update this to your new project's name and version
project(
    UsefulLibsAndFrameworks
    VERSION 1.0
    DESCRIPTION "Samples for useful C++ libraries and Frameworks"
    LANGUAGES CXX
)

include(CMakePrintHelpers)
# Options: Things you can set via commandline options to cmake (e.g. -DENABLE_LTO=[ON|OFF])
option(ENABLE_WARNINGS_SETTINGS "Allow target_set_warnings to add flags and defines.
                                 Set this to OFF if you want to provide your own warning parameters." ON
)
option(ENABLE_LTO "Enable link time optimization" ON)
option(ENABLE_CODE_COVERAGE "Enable test coverage" OFF)
# option(ENABLE_DOCTESTS "Include tests in the library. Setting this to OFF will remove all doctest related code. Tests
# in tests/*.cpp will still be enabled." ON)

# Include stuff. No change needed.
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_SOURCE_DIR}/cmake/")
include(ConfigSafeGuards)
include(Colors)
include(LTO)
include(Misc)
include(Warnings)
include(CPM)
include(tools)

if(CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME)

    # Let's ensure -std=c++xx instead of -std=g++xx
    set(CMAKE_CXX_EXTENSIONS OFF)

    # Let's nicely support folders in IDEs
    set_property(GLOBAL PROPERTY USE_FOLDERS ON)

    # Testing only available if this is the main app Note this needs to be done in the main CMakeLists since it calls
    # enable_testing, which must be in the main CMakeLists.
    include(CTest)
    if(ENABLE_CODE_COVERAGE)
        # Adds a 'coverage' target.
        include(CodeCoverage)
    endif()
    # Docs only available if this is the main app
    include(Documentation)
endif()

# Check for LTO support.
find_lto(CXX)

# ---- Add dependencies via CPM ----
# see https://github.com/TheLartians/CPM.cmake for more info
CPMAddPackage(
    NAME Format.cmake
    VERSION 1.7.3
    GITHUB_REPOSITORY TheLartians/Format.cmake
    OPTIONS # set to yes skip cmake formatting
            "FORMAT_SKIP_CMAKE NO"
            # path to exclude (optional, supports regular expressions)
            "CMAKE_FORMAT_EXCLUDE (cmake/.*|_deps/.*|cpm-package-lock\.cmake)"
)

CPMAddPackage(
    NAME fmt
    GIT_TAG 9.0.0
    GITHUB_REPOSITORY fmtlib/fmt
    OPTIONS "FMT_INSTALL YES" # create an installable target
)

CPMAddPackage(
    GITHUB_REPOSITORY jarro2783/cxxopts
    VERSION 3.0.0
    OPTIONS "CXXOPTS_BUILD_EXAMPLES NO" "CXXOPTS_BUILD_TESTS NO" "CXXOPTS_ENABLE_INSTALL YES"
)

CPMAddPackage("gh:gabime/spdlog@1.10.0")
CPMAddPackage(
  NAME cereal
  VERSION 1.3.2
  GITHUB_REPOSITORY USCiLab/cereal
  OPTIONS "SKIP_PORTABILITY_TEST ON" "JUST_INSTALL_CEREAL ON"
)
CPMAddPackage("gh:nlohmann/json@3.10.5")
CPMAddPackage("gh:ericniebler/range-v3#0.12.0")
CPMAddPackage("gh:eliaskosunen/scnlib@1.1.2")

file(GLOB sources CONFIGURE_DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)

add_executable(SampleApp ${sources})

target_compile_features(SampleApp PRIVATE cxx_std_17)
set_target_properties(SampleApp PROPERTIES OUTPUT_NAME "SampleApp")
target_set_warnings(
    SampleApp
    ENABLE
    ALL
    AS_ERROR
    ALL
    DISABLE
    Annoying
) # Set warnings (if needed).

target_link_libraries(SampleApp cxxopts::cxxopts  scn::scn)
