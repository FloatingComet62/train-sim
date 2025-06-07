#include "lib.h"

int main() {
  info("Hello, World!");
  TrainNetwork* network = init_train_network(5);
  if (!network) {
    error("Failed to initialize train network");
    return 1;
  }
  TrainStation station1 = {{0, 0}};
  TrainStation station2 = {{1, 0}};
  TrainStation station3 = {{2, 3}};
  TrainStation station4 = {{3, 5}};
  TrainStation station5 = {{4, 7}};

  position pos = lerp(station1.pos, station4.pos, 0.5f);
  info("%f %f", pos.x, pos.y);

  return 0;
}
