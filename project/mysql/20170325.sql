Use pusoy_dos;
DROP TABLE IF EXISTS `notifications`;

CREATE TABLE IF NOT EXISTS `notifications` (
    `user` INT UNSIGNED,
    `subscription` VARCHAR(500),
    PRIMARY KEY (`user`))
ENGINE = InnoDB
DEFAULT CHARACTER SET = utf8;
