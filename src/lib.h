#ifndef LIB_H
#define LIB_H

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct {
    int id;
} People;

typedef struct { float x; float y; } position;
position lerp(position a, position b, float ratio);

typedef struct {
    position pos;
} TrainStation;

typedef struct {
    int id;
    int from_station_idx;
    int to_station_idx;
    float journey_ratio; // 0 means at (from_station), 1 means at (to_station)
} Train;
position train_position(Train* train, TrainStation* stations);

typedef struct {
    int number_of_stations;
    int** station_connections;
} TrainNetwork;

TrainNetwork* init_train_network(int number_of_stations);
void deinit_train_network(TrainNetwork* network);
void add_connection(TrainNetwork* network, int from_station_id, int to_station_id, float weight);

enum LogLevel {
    LOG_LEVEL_INFO,
    LOG_LEVEL_WARN,
    LOG_LEVEL_ERROR,
    LOG_LEVEL_NO_LOGS,
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