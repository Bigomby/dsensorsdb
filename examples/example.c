#include <dsensorsdb.h>
#include <assert.h>

net_address_t address = {
    .network_address = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4},
    .network_mask = {4, 3, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
    .broadcast = {0, 0, 0, 0, 0, 0, 1, 2, 2, 1, 0, 0, 0, 0, 0, 0},
};

void assert_eq_address(net_address_t addr1, net_address_t addr2) {
  for (int i = 0; i < 16; i++) {
    assert(addr1.network_address[i] == addr2.network_address[i]);
    assert(addr1.network_mask[i] == addr2.network_mask[i]);
    assert(addr1.broadcast[i] == addr2.broadcast[i]);
  }
}

int main() {
  sensors_db_t *database = sensors_db_new();
  assert(database);

  sensor_t *sensor = sensor_new(address);
  assert(sensor);

  sensors_db_add(database, 1234, sensor);

  sensor_t *missing_sensor = sensors_db_get(database, 5678);
  assert(!missing_sensor);

  sensor_t *same_sensor = sensors_db_get(database, 1234);
  assert(same_sensor);

  net_address_t same_address = sensor_get_address(same_sensor);
  assert_eq_address(address, same_address);

  sensors_db_destroy(database);

  return 0;
}
