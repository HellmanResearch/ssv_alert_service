-- MySQL dump 10.13  Distrib 8.0.28, for macos11 (x86_64)
--
-- Host: 192.168.1.128    Database: ssv_alert_service_5
-- ------------------------------------------------------
-- Server version	8.0.27

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `__diesel_schema_migrations`
--
--
-- Table structure for table `account`
--

/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `account` (
                           `public_key` varchar(42) NOT NULL,
                           `ssv_balance_human` float NOT NULL,
                           PRIMARY KEY (`public_key`),
                           UNIQUE KEY `account_public_key_uindex` (`public_key`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `decided`
--

/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `decided` (
                           `id` bigint NOT NULL AUTO_INCREMENT,
                           `role` varchar(10) NOT NULL,
                           `validator_public_key` varchar(100) NOT NULL,
                           `signature` text NOT NULL,
                           `height` bigint NOT NULL,
                           `round` int NOT NULL,
                           `identifier` varchar(100) NOT NULL,
                           `message_type` int NOT NULL,
                           `timestamp` bigint unsigned NOT NULL,
                           PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=3037 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `operator`
--

/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `operator` (
                            `id` int unsigned NOT NULL,
                            `name` varchar(50) NOT NULL,
                            `account_public_key` varchar(42) NOT NULL,
                            `status` varchar(10) NOT NULL,
                            `validator_count` int unsigned NOT NULL,
                            `fee_human` float unsigned DEFAULT NULL,
                            PRIMARY KEY (`id`),
                            UNIQUE KEY `operator_name_uindex` (`name`),
                            KEY `operator_account` (`account_public_key`),
                            CONSTRAINT `operator_account` FOREIGN KEY (`account_public_key`) REFERENCES `account` (`public_key`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `operator_decided_record`
--

/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `operator_decided_record` (
                                           `id` bigint NOT NULL AUTO_INCREMENT,
                                           `operator_id` int NOT NULL,
                                           `validator_public_key` varchar(100) NOT NULL,
                                           `height` int NOT NULL,
                                           `round` int NOT NULL,
                                           `timestamp` bigint unsigned NOT NULL,
                                           PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=10382 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `posts`
--

/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `posts` (
                         `id` int NOT NULL AUTO_INCREMENT,
                         `title` varchar(30) NOT NULL,
                         `body` text NOT NULL,
                         `published` tinyint(1) NOT NULL DEFAULT '0',
                         PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `validator`
--

CREATE TABLE `validator` (
                             `account_public_key` varchar(42) NOT NULL,
                             `public_key` varchar(100) NOT NULL,
                             PRIMARY KEY (`public_key`),
                             UNIQUE KEY `validator_public_key_uindex` (`public_key`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;


CREATE TABLE `validator_operator` (
                                      `id` int NOT NULL AUTO_INCREMENT,
                                      `validator_public_key` varchar(100) NOT NULL,
                                      `operator_id` int NOT NULL,
                                      PRIMARY KEY (`id`),
                                      UNIQUE KEY `validator_operator_id_uindex` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE TABLE `performance_record` (
                                      `id` int unsigned NOT NULL AUTO_INCREMENT,
                                      `operator_id` int unsigned NOT NULL,
                                      `performance` float NOT NULL,
                                      `timestamp` bigint unsigned NOT NULL,
                                      PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;