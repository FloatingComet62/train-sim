#include "lib.h"

TrainNetwork* init_train_network(int number_of_stations) {
  TrainNetwork* network = malloc(sizeof(TrainNetwork));
  if (!network) {
    error("Failed to allocate memory for TrainNetwork");
    return NULL;
  }
  network->number_of_stations = number_of_stations;
  network->station_connections = malloc(number_of_stations * number_of_stations * sizeof(float));
  if (!network->station_connections) {
    error("Failed to allocate memory for station connections");
    free(network);
    return NULL;
  }
  return network;
}

void deinit_train_network(TrainNetwork* network) {
  free(network->station_connections);
  free(network);
}
void add_connection(TrainNetwork* network, int from_station_id, int to_station_id, float weight) {
  network->station_connections[from_station_id][to_station_id] = weight;
}