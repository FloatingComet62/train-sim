#include "lib.h"

position train_position(Train* train, TrainStation* stations) {
  position from_pos = stations[train->from_station_idx].pos;
  position to_pos = stations[train->to_station_idx].pos;
  return lerp(from_pos, to_pos, train->journey_ratio);
}