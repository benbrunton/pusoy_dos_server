-- pusoy dos

DROP SCHEMA IF EXISTS `pusoy_dos` ;

CREATE SCHEMA IF NOT EXISTS `pusoy_dos` DEFAULT CHARACTER SET latin1 ;
USE `pusoy_dos`;


-- user table
DROP TABLE IF EXISTS `user` ;

CREATE TABLE IF NOT EXISTS `user` (
  `id` INT(10) UNSIGNED NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(64) NOT NULL,
  `creation_date` DATETIME NOT NULL,
  `provider_id` VARCHAR(45) NOT NULL DEFAULT '0',
  `provider_type` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`id`),
  INDEX `idx_name` (`name` ASC),
  INDEX `idx_prov_id` (`provider_id` ASC))
ENGINE = InnoDB
AUTO_INCREMENT = 4
DEFAULT CHARACTER SET = utf8;
