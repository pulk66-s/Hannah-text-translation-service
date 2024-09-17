DROP DATABASE IF EXISTS HannahTTLSF;
CREATE DATABASE HannahTTLSF;
USE HannahTTLSF;

CREATE TABLE Adjectives (
    `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `value` VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE Adverbes (
    `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `value` VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE Nouns (
    `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `value` VARCHAR(255) NOT NULL UNIQUE,
    `type` ENUM('common', 'proper') NOT NULL
);

CREATE TABLE Determinants (
    `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `value` VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE Pronouns (
    `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `value` VARCHAR(255) NOT NULL UNIQUE,
    `type` ENUM('personal', 'demonstrative', 'indefinite', 'possessive', 'relative', 'interrogative') NOT NULL
);

CREATE TABLE Verbs (
    `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `value` VARCHAR(255) NOT NULL UNIQUE,
    `tense` ENUM('present', 'past', 'future', 'infinitive') NOT NULL
);

CREATE TABLE Prepositions (
    `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `value` VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE CoordinatingConjuctions (
    `id` INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `value` VARCHAR(255) NOT NULL UNIQUE
);
