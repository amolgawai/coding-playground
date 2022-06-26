cmake_minimum_required(VERSION 3.23 FATAL_ERROR)

# ---- Project ----

# Note: update this to your new project's name and version
project(
  const_string_lib
  VERSION 1.0
  DESCRIPTION "Constant String Library"
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

  # Enable Testing
  include(CTest)


endif()

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# ---- Add dependencies via CPM ----
# see https://github.com/TheLartians/CPM.cmake for more info

include(cmake/CPM.cmake)
include(cmake/tools.cmake)


# ---- Create library ----

add_library(const_str src/const_string.cpp)

# All users of this library will need at least C++17
target_compile_features(const_str PUBLIC cxx_std_17)

target_include_directories(
  const_str PUBLIC $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
                 $<INSTALL_INTERFACE:include/${PROJECT_NAME}-${PROJECT_VERSION}>
                 )



if(CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME AND BUILD_TESTING)
  # ------ Testing Libraries --------

  set(Boost_USE_STATIC_LIBS ON)
  set(Boost_USE_MULTITHREADED ON)
  set(Boost_USE_STATIC_RUNTIME OFF)
  CPMAddPackage(
    NAME Boost
    VERSION 1.79.0
    GITHUB_REPOSITORY "boostorg/boost"
    GIT_TAG "boost-1.79.0"
  )
  if(Boost_ADDED)
      message("boost asio downloaded at " ${Boost_SOURCE_DIR}/libs/asio/include)

      file(GLOB files "${Boost_SOURCE_DIR}/libs/*/include")
      foreach(filename ${files})
          include_directories(${filename})
      endforeach()

      file(GLOB files "${Boost_SOURCE_DIR}/libs/*/*/include")
      foreach(filename ${files})
          include_directories(${filename})
      endforeach()
  endif()

  CPMAddPackage("gh:onqtam/doctest@2.4.8")
  CPMAddPackage("gh:catchorg/Catch2@3.0.1")
  list(APPEND CMAKE_MODULE_PATH ${Catch2_SOURCE_DIR}/extras)
  CPMAddPackage(
    NAME googletest
    GITHUB_REPOSITORY google/googletest
    GIT_TAG release-1.11.0
    VERSION 1.11.0
    OPTIONS "INSTALL_GTEST OFF" "gtest_force_shared_crt"
)

  add_subdirectory(tests)
endif()
