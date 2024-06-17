# Dnsync

Dnsync is a simple tool to synchronize DNS records 

## Environment Variables

See the example in the `.env.example` file

| Key                   | Description                                                   |
|-----------------------|---------------------------------------------------------------|
| `SYNC_INTERVAL`       | Interval in seconds to sync DNS records. Example `60` seconds |
| `HOSTNAME_SUFFIX`     | Suffix to append to the hostname. Example `.vm`               |
| `RUST_LOG`            | Log level. Default is not set. Can take `info`, `debug` ...   |
| `RABBITMQ_HOST`       | RabbitMQ host                                                 |
| `RABBITMQ_PORT`       | RabbitMQ port                                                 |
| `RABBITMQ_USER`       | RabbitMQ user                                                 |
| `RABBITMQ_PASSWORD`   | RabbitMQ password                                             |
| `RABBITMQ_VH`         | RabbitMQ vhost                                                |
| `SELECTEL_PROJECT`    | Selectel project                                              |
| `SELECTEL_USERNAME`   | Selectel username                                             |
| `SELECTEL_PASSWORD`   | Selectel password                                             |
| `SELECTEL_ACCOUNT_ID` | Selectel account ID                                           |
| `SELECTEL_ZONE_NAME`  | Selectel zone name                                            |
| `SELECTEL_RECORD_TTL` | Selectel record TTL                                           |
| `SELECTEL_AUTH_URL`   | Selectel auth URL                                             |
| `SELECTEL_ZONE_URL`   | Selectel zone URL. Example `.domain.zone.`                    |

