CREATE TABLE `account`
(
    `public_key`        varchar(42) NOT NULL,
    `ssv_balance_human` float       NOT NULL,
    PRIMARY KEY (`public_key`),
    UNIQUE KEY `account_public_key_uindex` (`public_key`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;


CREATE TABLE `operator`
(
    `id`                 int unsigned NOT NULL,
    `name`               varchar(50) NOT NULL,
    `account_public_key` varchar(42) NOT NULL,
    `status`             varchar(10) NOT NULL,
    `validator_count`    int unsigned NOT NULL,
    `fee_human`          float unsigned DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `operator_name_uindex` (`name`),
    KEY                  `operator_account` (`account_public_key`),
    CONSTRAINT `operator_account` FOREIGN KEY (`account_public_key`) REFERENCES `account` (`public_key`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
