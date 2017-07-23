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

sensors_db_t *sensors_db_new();

void sensors_db_destroy(sensors_db_t *db);

sensor_t *sensors_db_get(sensors_db_t *db, uint32_t ip);

void sensors_db_add(sensors_db_t *db, sensor_t *sensor);

////////////////////////////////////////////////////////////////////////////////
// Sensor
////////////////////////////////////////////////////////////////////////////////

sensor_t *sensor_new(uint8_t ip[16]);

const char *sensor_get_ip_string(const sensor_t *sensor);

uint32_t sensor_get_ip(const sensor_t *sensor);

observation_id_t *sensor_get_observation_id(const sensor_t *sensor,
                                            uint32_t id);

void *sensor_get_worker(const sensor_t *sensor);

void sensor_set_worker(sensor_t *sensor, void *worker);

void sensor_add_observation_id(sensor_t *sensor,
                               observation_id_t *observation_id);

void sensor_add_default_observation_id(sensor_t *sensor, uint32_t id,
                                       observation_id_t *observation_id);

////////////////////////////////////////////////////////////////////////////////
// Observation ID
////////////////////////////////////////////////////////////////////////////////

observation_id_t *observation_id_new(uint32_t id);

const char *
observation_id_get_network_ip(const observation_id_t *observation_id,
                              const uint8_t ip[16]);

uint32_t observation_id_get_id(const observation_id_t *observation_id);

const char *
observation_id_get_network_name(const observation_id_t *observation_id,
                                const uint8_t ip[16]);

const char *
observation_id_get_selector_name(const observation_id_t *observation_id,
                                 uint64_t selector_id);

const char *
observation_id_get_interface_name(const observation_id_t *observation_id,
                                  uint64_t interface_id);

const char *
observation_id_get_interface_description(const observation_id_t *observation_id,
                                         uint64_t interface_id);

int64_t observation_id_get_fallback_first_switch(
    const observation_id_t *observation_id);

void *observation_id_get_template(const observation_id_t *observation_id,
                                  const uint16_t template_id);

const char *
observation_id_get_application_name(const observation_id_t *observation_id,
                                    uint64_t application_id);

const char *
observation_id_get_enrichment(const observation_id_t *observation_id);

bool observation_id_want_client_dns(const observation_id_t *observation_id);

bool observation_id_want_target_dns(const observation_id_t *observation_id);

bool observation_id_is_exporter_in_wan_side(
    const observation_id_t *observation_id);

bool observation_id_is_span(const observation_id_t *observation_id);

void observation_id_add_template(observation_id_t *observation_id,
                                 const void *tmpl);

void observation_id_add_template_async(observation_id_t *observation_id,
                                       void *tmpl);

void observation_id_add_application_id(observation_id_t *observation_id,
                                       uint64_t application_id,
                                       const char *application_name,
                                       size_t application_name_len);

void observation_id_add_selector_id(observation_id_t *observation_id,
                                    uint64_t selector_id,
                                    const char *selector_name,
                                    size_t selector_name_len);

void observation_id_add_interface(observation_id_t *observation_id,
                                  uint64_t interface_id,
                                  const char *interface_name,
                                  size_t interface_name_len,
                                  const char *interface_description,
                                  size_t interface_description_len);

void observation_id_set_enrichment(const observation_id_t *observation_id,
                                   const char *enrichment);

void observation_id_set_fallback_first_switch(observation_id_t *observation_id,
                                              int64_t fallback_first_switch);

void observation_id_set_exporter_in_wan_side(observation_id_t *observation_id);

void observation_id_set_span_mode(observation_id_t *observation_id);

void observation_id_enable_ptr_dns_client(observation_id_t *observation_id);

void observation_id_enable_ptr_dns_target(observation_id_t *observation_id);

////////////////////////////////////////////////////////////////////////////////
// Network
////////////////////////////////////////////////////////////////////////////////

network_t *network_new(net_address_t address, const char *name);
