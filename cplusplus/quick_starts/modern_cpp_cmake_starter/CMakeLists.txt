cmake_minimum_required(VERSION 3.23 FATAL_ERROR)

# ---- Project ----

# Note: update this to your new project's name and version
project(
  ModernCPPPractice
  VERSION 1.0
  DESCRIPTION "Practicing Modern C++"
  LANGUAGES CXX
)

# ---- Include guards ----

if(PROJECT_SOURCE_DIR STREQUAL PROJECT_BINARY_DIR)
  message(FATAL_ERROR "In-source builds not allowed. Use a build directory.")
endif()

if(CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME)

  # Optionally set things like CMAKE_CXX_STANDARD, CMAKE_POSITION_INDEPENDENT_CODE here

  # Let's ensure -std=c++xx instead of -std=g++xx
  set(CMAKE_CXX_EXTENSIONS OFF)

  # Let's nicely support folders in IDEs
  set_property(GLOBAL PROPERTY USE_FOLDERS ON)

  # Testing only available if this is the main app Note this needs to be done in the main CMakeLists
  # since it calls enable_testing, which must be in the main CMakeLists.
  include(CTest)
  # include(CMakePrintHelpers) Docs only available if this is the main app
  find_package(Doxygen)
endif()

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_LIST_DIR}/cmake")
# cmake_print_variables(CMAKE_MODULE_PATH)

# ---- Add dependencies via CPM ----
# see https://github.com/TheLartians/CPM.cmake for more info

include(cmake/CPM.cmake)
include(cmake/tools.cmake)

CPMAddPackage(
  NAME fmt
  GIT_TAG 7.1.3
  GITHUB_REPOSITORY fmtlib/fmt
  OPTIONS "FMT_INSTALL YES" # create an installable target
)
CPMAddPackage("gh:onqtam/doctest@2.4.8")
CPMAddPackage("gh:TheLartians/Format.cmake@1.7.3")
add_subdirectory(libs)

CPMAddPackage(
  GITHUB_REPOSITORY jarro2783/cxxopts
  VERSION 2.2.1
  OPTIONS "CXXOPTS_BUILD_EXAMPLES NO" "CXXOPTS_BUILD_TESTS NO" "CXXOPTS_ENABLE_INSTALL YES"
)
add_subdirectory(apps)

# PackageProject.cmake will be used to make our target installable
CPMAddPackage("gh:TheLartians/PackageProject.cmake@1.8.0")

# ---- Create an installable target ----
# this allows users to install and find the library via `find_package()`. the location where the
# project's version header will be placed should match the project's regular header paths
string(TOLOWER greeter/version.h VERSION_HEADER_LOCATION)
packageProject(
  NAME greeter
  VERSION ${PROJECT_VERSION}
  NAMESPACE greeter
  BINARY_DIR ${PROJECT_BINARY_DIR}
  INCLUDE_DIR ${PROJECT_SOURCE_DIR}/libs/greeter/include
  INCLUDE_DESTINATION include/${PROJECT_NAME}-${PROJECT_VERSION}
  VERSION_HEADER "${VERSION_HEADER_LOCATION}"
  COMPATIBILITY SameMajorVersion
  DEPENDENCIES "fmt 8.1.1"
)
