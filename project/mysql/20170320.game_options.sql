USE pusoy_dos;
ALTER table game
  ADD `creation_date` DATETIME NOT NULL,
  ADD `max_move_duration` INT(10) NULL,
  ADD `max_players` INT(10) NULL,
  ADD `decks` INT(10) NOT NULL DEFAULT 1;
