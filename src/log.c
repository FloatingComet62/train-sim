#include "lib.h"
#include <stdarg.h>

void lib_log_info(const char *format, ...) {
  if (LOG_LEVEL > LOG_LEVEL_INFO) {
    return;
  }
  printf("[INFO] ");
  va_list args;
  va_start(args, format);
  vprintf(format, args);
  va_end(args);
  printf("\n");
}
void lib_log_warn(const char *format, ...) {
  if (LOG_LEVEL > LOG_LEVEL_WARN) {
    return;
  }
  printf("[WARN] ");
  va_list args;
  va_start(args, format);
  vprintf(format, args);
  va_end(args);
  printf("\n");
}
void lib_log_error(const char *format, ...) {
  if (LOG_LEVEL > LOG_LEVEL_ERROR) {
    return;
  }
  printf("[ERROR] ");
  va_list args;
  va_start(args, format);
  vprintf(format, args);
  va_end(args);
  printf("\n");
}
void lib_log_info_more(char* file, int line_num, const char *format, ...) {
  if (LOG_LEVEL > LOG_LEVEL_INFO) {
    return;
  }
  printf("[INFO] (%s:%d) ", file, line_num);
  va_list args;
  va_start(args, format);
  vprintf(format, args);
  va_end(args);
  printf("\n");
}
void lib_log_warn_more(char* file, int line_num, const char *format, ...) {
  if (LOG_LEVEL > LOG_LEVEL_WARN) {
    return;
  }
  printf("[WARN] (%s:%d) ", file, line_num);
  va_list args;
  va_start(args, format);
  vprintf(format, args);
  va_end(args);
  printf("\n");
}
void lib_log_error_more(char* file, int line_num, const char *format, ...) {
  if (LOG_LEVEL > LOG_LEVEL_ERROR) {
    return;
  }
  printf("[ERROR] (%s:%d) ", file, line_num);
  va_list args;
  va_start(args, format);
  vprintf(format, args);
  va_end(args);
  printf("\n");
}