# up.sql
--
CREATE TABLE `rocket_app`.`pageviews` (
   `id` BIGINT NOT NULL AUTO_INCREMENT,
   `view_time` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
   `url` VARCHAR(2083) NOT NULL,
   `user_agent` VARCHAR(2083) NOT NULL,
   `referrer` VARCHAR(2083) NOT NULL,
   `device_type` TINYINT NOT NULL DEFAULT '0',
   PRIMARY KEY (`id`)
);