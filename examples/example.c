#include <assert.h>
#include <dsensorsdb.h>
#include <stdlib.h>
#include <string.h>

const uint8_t sample_data[128] = {0};

int main() {
  uint8_t address[] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 1, 2, 3, 4};
  uint8_t *template = calloc(128, sizeof(uint8_t));

  memcpy(template, sample_data, 128);

  observation_id_t *observation_id = observation_id_new(123);
  assert(observation_id);

  sensor_t *sensor = sensor_new(address);
  assert(sensor);

  sensors_db_t *database = sensors_db_new();
  assert(database);

  observation_id_add_template(observation_id, 42, template);
  sensor_add_observation_id(sensor, observation_id);
  sensors_db_add(database, sensor);

  sensor_t *same_sensor = sensors_db_get(database, 16909060);
  assert(same_sensor);

  observation_id_t *same_observation_id =
      sensor_get_observation_id(same_sensor, 123);
  assert(same_observation_id);

  void *same_template = observation_id_get_template(same_observation_id, 42);
  assert(same_template);

  free(same_template);
  sensors_db_destroy(database);

  return 0;
}
