USE pusoy_dos;
ALTER TABLE `user_game` ADD UNIQUE `unique_index`(`game`, `user`);
