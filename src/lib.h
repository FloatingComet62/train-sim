#ifndef LIB_H
#define LIB_H

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

enum LogLevel {
    LOG_LEVEL_INFO,
    LOG_LEVEL_WARN,
    LOG_LEVEL_ERROR
};
#ifdef DEBUG
#define LOG_LEVEL LOG_LEVEL_INFO
#define info(...) lib_log_info_more(__FILE__, __LINE__, __VA_ARGS__)
#define warn(...) lib_log_warn_more(__FILE__, __LINE__, __VA_ARGS__)
#define error(...) lib_log_error_more(__FILE__, __LINE__, __VA_ARGS__)
#else
#define LOG_LEVEL LOG_LEVEL_WARN
#define info(...) lib_log_info(__VA_ARGS__)
#define warn(...) lib_log_warn(__VA_ARGS__)
#define error(...) lib_log_error(__VA_ARGS__)
#endif

void lib_log_info(const char *format, ...);
void lib_log_warn(const char *format, ...);
void lib_log_error(const char *format, ...);
void lib_log_info_more(char* file, int line_num, const char *format, ...);
void lib_log_warn_more(char* file, int line_num, const char *format, ...);
void lib_log_error_more(char* file, int line_num, const char *format, ...);

#endif