cmake_minimum_required(VERSION 3.20)
project(chut_os C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

add_library(chut_shell_parser STATIC
	userland/shell/parser.c
)
target_include_directories(chut_shell_parser PUBLIC
	${CMAKE_CURRENT_SOURCE_DIR}/userland/shell
)

enable_testing()
add_executable(chut_shell_parser_test
	tests/shell_parser_test.c
)
target_link_libraries(chut_shell_parser_test PRIVATE chut_shell_parser)
target_include_directories(chut_shell_parser_test PRIVATE
	${CMAKE_CURRENT_SOURCE_DIR}
)
add_test(NAME shell_parser COMMAND chut_shell_parser_test)
