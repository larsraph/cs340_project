/*   Beaver Club (CS340) - Team 61
   Raphael Larsen and America Pacheco*/



/*M!999999\- enable the sandbox mode */ 
-- MariaDB dump 10.19  Distrib 10.5.29-MariaDB, for Linux (x86_64)
--
-- Host: classmysql.engr.oregonstate.edu    Database: cs340_larsraph
-- ------------------------------------------------------
-- Server version	10.11.15-MariaDB-log

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `Addresses`
--

DROP TABLE IF EXISTS `Addresses`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `Addresses` (
  `address_id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `country_code` char(3) NOT NULL,
  `zip_code` varchar(45) NOT NULL,
  `address_ln1` varchar(45) NOT NULL,
  `address_ln2` varchar(45) DEFAULT NULL,
  `city` varchar(45) DEFAULT NULL,
  `state` varchar(45) DEFAULT NULL,
  PRIMARY KEY (`address_id`),
  UNIQUE KEY `address_id_UNIQUE` (`address_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Addresses`
--

LOCK TABLES `Addresses` WRITE;
/*!40000 ALTER TABLE `Addresses` DISABLE KEYS */;
/*!40000 ALTER TABLE `Addresses` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Clubs`
--

DROP TABLE IF EXISTS `Clubs`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `Clubs` (
  `club_id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(45) NOT NULL,
  `date_created` date NOT NULL,
  `is_active` tinyint(4) NOT NULL,
  PRIMARY KEY (`club_id`),
  UNIQUE KEY `club_id_UNIQUE` (`club_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Clubs`
--

LOCK TABLES `Clubs` WRITE;
/*!40000 ALTER TABLE `Clubs` DISABLE KEYS */;
/*!40000 ALTER TABLE `Clubs` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Events`
--

DROP TABLE IF EXISTS `Events`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `Events` (
  `event_id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(45) NOT NULL,
  `description` longtext NOT NULL,
  `time_start` datetime NOT NULL,
  `time_end` datetime NOT NULL,
  `club_id` int(10) unsigned NOT NULL,
  `organizer_id` int(10) unsigned NOT NULL,
  PRIMARY KEY (`event_id`,`club_id`,`organizer_id`),
  UNIQUE KEY `event_id_UNIQUE` (`event_id`),
  KEY `fk_Events_Clubs_idx` (`club_id`),
  KEY `fk_Events_People1_idx` (`organizer_id`),
  CONSTRAINT `fk_Events_Clubs` FOREIGN KEY (`club_id`) REFERENCES `Clubs` (`club_id`) ON DELETE CASCADE,
  CONSTRAINT `fk_Events_People1` FOREIGN KEY (`organizer_id`) REFERENCES `People` (`person_id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Events`
--

LOCK TABLES `Events` WRITE;
/*!40000 ALTER TABLE `Events` DISABLE KEYS */;
/*!40000 ALTER TABLE `Events` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Membership`
--

DROP TABLE IF EXISTS `Membership`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `Membership` (
  `person_id` int(10) unsigned NOT NULL,
  `role_id` int(10) unsigned NOT NULL,
  `club_id` int(10) unsigned NOT NULL,
  PRIMARY KEY (`person_id`,`role_id`,`club_id`),
  KEY `fk_Membership_Roles1_idx` (`role_id`),
  KEY `fk_Membership_Clubs1_idx` (`club_id`),
  CONSTRAINT `fk_Membership_Clubs1` FOREIGN KEY (`club_id`) REFERENCES `Clubs` (`club_id`) ON DELETE CASCADE,
  CONSTRAINT `fk_Membership_People1` FOREIGN KEY (`person_id`) REFERENCES `People` (`person_id`) ON DELETE CASCADE,
  CONSTRAINT `fk_Membership_Roles1` FOREIGN KEY (`role_id`) REFERENCES `Roles` (`role_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Membership`
--

LOCK TABLES `Membership` WRITE;
/*!40000 ALTER TABLE `Membership` DISABLE KEYS */;
/*!40000 ALTER TABLE `Membership` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `People`
--

DROP TABLE IF EXISTS `People`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `People` (
  `person_id` int(10) unsigned NOT NULL,
  `email` varchar(45) NOT NULL,
  `onid` varchar(45) NOT NULL,
  `phone_numer` varchar(45) NOT NULL,
  `date_of_birth` date NOT NULL,
  `gender_identity` varchar(45) NOT NULL,
  `address_id` int(10) unsigned NOT NULL,
  PRIMARY KEY (`person_id`,`address_id`),
  UNIQUE KEY `person_id_UNIQUE` (`person_id`),
  UNIQUE KEY `email_UNIQUE` (`email`),
  UNIQUE KEY `onid_UNIQUE` (`onid`),
  UNIQUE KEY `phone_numer_UNIQUE` (`phone_numer`),
  KEY `fk_People_Addresses1_idx` (`address_id`),
  CONSTRAINT `fk_People_Addresses1` FOREIGN KEY (`address_id`) REFERENCES `Addresses` (`address_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `People`
--

LOCK TABLES `People` WRITE;
/*!40000 ALTER TABLE `People` DISABLE KEYS */;
/*!40000 ALTER TABLE `People` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `PhysicalEvents`
--

DROP TABLE IF EXISTS `PhysicalEvents`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `PhysicalEvents` (
  `event_id` int(10) unsigned NOT NULL,
  `address_id` int(10) unsigned NOT NULL,
  PRIMARY KEY (`event_id`,`address_id`),
  KEY `fk_PhysicalEvents_Addresses1_idx` (`address_id`),
  CONSTRAINT `fk_PhysicalEvents_Addresses1` FOREIGN KEY (`address_id`) REFERENCES `Addresses` (`address_id`),
  CONSTRAINT `fk_PhysicalEvents_Events1` FOREIGN KEY (`event_id`) REFERENCES `Events` (`event_id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `PhysicalEvents`
--

LOCK TABLES `PhysicalEvents` WRITE;
/*!40000 ALTER TABLE `PhysicalEvents` DISABLE KEYS */;
/*!40000 ALTER TABLE `PhysicalEvents` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Roles`
--

DROP TABLE IF EXISTS `Roles`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `Roles` (
  `role_id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(45) NOT NULL,
  PRIMARY KEY (`role_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Roles`
--

LOCK TABLES `Roles` WRITE;
/*!40000 ALTER TABLE `Roles` DISABLE KEYS */;
/*!40000 ALTER TABLE `Roles` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `VirtualEvents`
--

DROP TABLE IF EXISTS `VirtualEvents`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `VirtualEvents` (
  `event_id` int(10) unsigned NOT NULL,
  `url` varchar(255) NOT NULL,
  PRIMARY KEY (`event_id`),
  CONSTRAINT `fk_VirtualEvents_Events1` FOREIGN KEY (`event_id`) REFERENCES `Events` (`event_id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `VirtualEvents`
--

LOCK TABLES `VirtualEvents` WRITE;
/*!40000 ALTER TABLE `VirtualEvents` DISABLE KEYS */;
/*!40000 ALTER TABLE `VirtualEvents` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2026-02-05 15:14:36

/* clubs*/
INSERT INTO Clubs (club_id, name, date_created, is_active) 
VALUES (0, 'organatans', '2050-12-25', 1),
      (1, 'slugs',      '2016-03-04', 0),
     (2, 'beavers',    '2026-02-05', 1);

/* addy*/
INSERT INTO Addresses (address_id, country_code, zip_code, address_ln1, address_ln2, city, state) VALUES
  (0, 'USA', '20500', '1600 Pennsylvania Ave', NULL, 'Washington', 'DC'),
  (1, 'USA', '12345', '200 Abc Pl',            NULL, NULL,         NULL),
  (2, 'USA', '56789', '666 Devil Street',      NULL, NULL,         NULL);

/* people*/
INSERT INTO People (person_id, address_id, email, onid, phone_number, date_of_birth, gender_identity) VALUES
  (0, 0, 'joebiden@ab.c',   'bijo@osu.edu', '1231231234', '1999-03-21', 'Male'),
  (1, 0, 'obama@ab.c',      'obba@osu.edu', '2342342345', '2001-04-12', 'Male'),
  (2, 1, 'president@ab.c',  'prez@osu.edu', '3453453456', '2000-09-24', 'Yo Mama'),
  (3, 2, 'sweetfruit@ab.c', 'frsw@osu.edu', '4564564567', '1989-11-23', 'Female');

/* roles*/
INSERT INTO Roles (role_id, name) VALUES
  (0, 'Vainglorious Leader'),
  (1, 'Grunt'),
  (2, 'Infiltrator');