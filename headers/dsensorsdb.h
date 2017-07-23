#pragma once

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>

typedef struct net_address_s {
  uint8_t network[16];
  uint8_t network_mask[16];
  uint8_t broadcast[16];
} net_address_t;

typedef struct sensor_s sensor_t;
typedef struct sensors_db_s sensors_db_t;
typedef struct observation_id_s observation_id_t;
typedef struct network_s network_t;

////////////////////////////////////////////////////////////////////////////////
// Sensors DB
////////////////////////////////////////////////////////////////////////////////

/**
 * [sensors_database_new description]
 * @return [description]
 */
sensors_db_t *sensors_db_new();

/**
 * [sensors_database_destroy description]
 * @param database [description]
 */
void sensors_db_destroy(sensors_db_t *db);

/**
* [sensors_database_get description]
* @param  database_ptr [description]
* @param  ip           [description]
* @return              [description]
*/
sensor_t *sensors_db_get(sensors_db_t *db, uint32_t ip);

/**
 * [sensors_database_add description]
 * @param database_ptr [description]
 * @param ip           [description]
 * @param sensor       [description]
 */
void sensors_db_add(sensors_db_t *db, uint32_t ip, sensor_t *sensor);

/**
 * [sensors_db_add_bad_sensor description]
 * @param  db        [description]
 * @param  sensor_ip [description]
 * @return           [description]
 */
int sensors_db_add_bad_sensor(sensors_db_t *db, uint64_t sensor_ip);

////////////////////////////////////////////////////////////////////////////////
// Sensor
////////////////////////////////////////////////////////////////////////////////

/**
 * [sensor_new description]
 * @param  ip [description]
 * @return    [description]
 */
sensor_t *sensor_new(net_address_t ip);

/**
 * [sensor_get_address description]
 * @param  sensor [description]
 * @return        [description]
 */
net_address_t sensor_get_address(const sensor_t *sensor);

/**
 * [sensor_get_ip_string description]
 * @param  sensor [description]
 * @return        [description]
 */
const char *sensor_get_ip_string(const sensor_t *sensor);

/**
 * [sensor_get_ip description]
 * @param  sensor [description]
 * @return        [description]
 */
uint32_t sensor_get_ip(const sensor_t *sensor);

/**
 * [sensor_get_observation_id description]
 * @param  sensor [description]
 * @param  id     [description]
 * @return        [description]
 */
observation_id_t *sensor_get_observation_id(const sensor_t *sensor,
                                            uint32_t id);

/**
 * [sensor_get_worker description]
 * @param  sensor [description]
 * @return        [description]
 */
void *sensor_get_worker(const sensor_t *sensor);

/**
 * [sensor_set_worker description]
 * @param sensor [description]
 * @param worker [description]
 */
void sensor_set_worker(sensor_t *sensor, void *worker);

/**
 * [sensor_add_observation_id description]
 * @param sensor         [description]
 * @param id             [description]
 * @param observation_id [description]
 */
void sensor_add_observation_id(sensor_t *sensor, uint32_t id,
                               observation_id_t *observation_id);

/**
 * [sensor_add_default_observation_id description]
 * @param sensor         [description]
 * @param id             [description]
 * @param observation_id [description]
 */
void sensor_add_default_observation_id(sensor_t *sensor, uint32_t id,
                                       observation_id_t *observation_id);

////////////////////////////////////////////////////////////////////////////////
// Observation ID
////////////////////////////////////////////////////////////////////////////////

/**
 * [observation_id_new description]
 * @param  id [description]
 * @return    [description]
 */
observation_id_t *observation_id_new(uint32_t id);

/**
 * [observation_id_get_network_ip description]
 * @param  observation_id [description]
 * @param  [name]         [description]
 * @return                [description]
 */
const char *
observation_id_get_network_ip(const observation_id_t *observation_id,
                              const uint8_t ip[16]);

/**
 * [observation_id_num description]
 * @param  observation_id [description]
 * @return                [description]
 */
uint32_t observation_id_get_id(const observation_id_t *observation_id);

/**
 * [observation_id_get_network_name description]
 * @param  observation_id [description]
 * @param  [name]         [description]
 * @return                [description]
 */
const char *
observation_id_get_network_name(const observation_id_t *observation_id,
                                const uint8_t ip[16]);

/**
 * [observation_id_get_selector_name description]
 * @param  observation_id [description]
 * @param  selector_id    [description]
 * @return                [description]
 */
const char *
observation_id_get_selector_name(const observation_id_t *observation_id,
                                 uint64_t selector_id);

/**
 * [observation_id_get_interface_name description]
 * @param  observation_id [description]
 * @param  interface_id   [description]
 * @return                [description]
 */
const char *
observation_id_get_interface_name(const observation_id_t *observation_id,
                                  uint64_t interface_id);

/**
 * [observation_id_get_interface_description description]
 * @param  observation_id [description]
 * @param  interface_id   [description]
 * @return                [description]
 */
const char *
observation_id_get_interface_description(const observation_id_t *observation_id,
                                         uint64_t interface_id);

/**
 * [observation_id_fallback_first_switch description]
 * @param  obs_id [description]
 * @return        [description]
 */
int64_t observation_id_get_fallback_first_switch(
    const observation_id_t *observation_id);

/**
 * [observation_id_get_template description]
 * @param  observation_id [description]
 * @param  template_id    [description]
 * @return                [description]
 */
void *observation_id_get_template(const observation_id_t *observation_id,
                                  const uint16_t template_id);

/**
 * [observation_id_get_application_name description]
 * @param  observation_id [description]
 * @param  application_id [description]
 * @return                [description]
 */
const char *
observation_id_get_application_name(const observation_id_t *observation_id,
                                    uint64_t application_id);

/**
 * [observation_id_enrichment description]
 * @param  obs_id [description]
 * @return        [description]
 */
const char *
observation_id_get_enrichment(const observation_id_t *observation_id);

/**
 * [observation_id_want_client_dns description]
 * @param  observation_id [description]
 * @return                [description]
 */
bool observation_id_want_client_dns(const observation_id_t *observation_id);

/**
 * [observation_id_want_target_dns description]
 * @param  observation_id [description]
 * @return                [description]
 */
bool observation_id_want_target_dns(const observation_id_t *observation_id);

/**
 * [observation_id_is_exporter_in_wan_side description]
 * @param  observation_id [description]
 * @return                [description]
 */
bool observation_id_is_exporter_in_wan_side(
    const observation_id_t *observation_id);

/**
 * [observation_id_is_span description]
 * @param  observation_id [description]
 * @return                [description]
 */
bool observation_id_is_span(const observation_id_t *observation_id);

/**
* [observation_id_add_template description]
* @param observation_id [description]
* @param tmpl           [description]
*/
void observation_id_add_template(observation_id_t *observation_id,
                                 const void *tmpl);

/**
 * [observation_id_add_template_async description]
 * @param observation_id [description]
 * @param tmpl           [description]
 */
void observation_id_add_template_async(observation_id_t *observation_id,
                                       void *tmpl);

/**
 * [observation_id_add_application_id description]
 * @param observation_id       [description]
 * @param application_id       [description]
 * @param application_name     [description]
 * @param application_name_len [description]
 */
void observation_id_add_application_id(observation_id_t *observation_id,
                                       uint64_t application_id,
                                       const char *application_name,
                                       size_t application_name_len);

/**
 * [observation_id_add_selector_id description]
 * @param observation_id    [description]
 * @param selector_id       [description]
 * @param selector_name     [description]
 * @param selector_name_len [description]
 */
void observation_id_add_selector_id(observation_id_t *observation_id,
                                    uint64_t selector_id,
                                    const char *selector_name,
                                    size_t selector_name_len);

/**
 * [observation_id_add_interface description]
 * @param observation_id            [description]
 * @param interface_id              [description]
 * @param interface_name            [description]
 * @param interface_name_len        [description]
 * @param interface_description     [description]
 * @param interface_description_len [description]
 */
void observation_id_add_interface(observation_id_t *observation_id,
                                  uint64_t interface_id,
                                  const char *interface_name,
                                  size_t interface_name_len,
                                  const char *interface_description,
                                  size_t interface_description_len);

/**
 * [observation_id_set_enrichment description]
 * @param observation_id [description]
 * @param enrichment     [description]
 */
void observation_id_set_enrichment(const observation_id_t *observation_id,
                                   const char *enrichment);

/**
 * [observation_id_set_fallback_first_switch description]
 * @param observation_id [description]
 * @param first_switch   [description]
 */
void observation_id_set_fallback_first_switch(observation_id_t *observation_id,
                                              int64_t fallback_first_switch);

/**
 * [observation_id_set_exporter_in_wan_side description]
 * @param observation_id [description]
 */
void observation_id_set_exporter_in_wan_side(observation_id_t *observation_id);

/**
 * [observation_id_set_span_mode description]
 * @param observation_id [description]
 */
void observation_id_set_span_mode(observation_id_t *observation_id);

/**
 * [observation_id_enable_ptr_dns_client description]
 * @param observation_id [description]
 */
void observation_id_enable_ptr_dns_client(observation_id_t *observation_id);

/**
 * [observation_id_enable_ptr_dns_client description]
 * @param observation_id [description]
 */
void observation_id_enable_ptr_dns_target(observation_id_t *observation_id);

////////////////////////////////////////////////////////////////////////////////
// Network
////////////////////////////////////////////////////////////////////////////////

/**
 * [network_new description]
 * @return [description]
 */
network_t *network_new(net_address_t address, const char *name);
