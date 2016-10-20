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

-- session table
DROP TABLE IF EXISTS `session`;

CREATE TABLE IF NOT EXISTS `session` (
    `id` VARCHAR(45) NOT NULL,
    `user_id` INT(10) UNSIGNED NULL DEFAULT NULL,
    PRIMARY KEY (`id`))
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8;

-- game table
DROP TABLE IF EXISTS `game` ;

CREATE TABLE IF NOT EXISTS `game` (
  `id` INT(10) UNSIGNED NOT NULL AUTO_INCREMENT,
  `creator` INT(10) UNSIGNED NOT NULL,
  PRIMARY KEY (`id`))
ENGINE = InnoDB
AUTO_INCREMENT = 4
DEFAULT CHARACTER SET = utf8;


