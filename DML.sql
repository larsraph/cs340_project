-- Full getters
SELECT `club_id`, `name`, `date_created`, `is_active`
FROM `Clubs`;

SELECT `address_id`, `country_code`, `zip_code`, `address_ln1`, `address_ln2`, `city`, `state`
FROM `Addresses`;

SELECT `person_id`, `address_id`, `email`, `onid`, `phone_number`, `date_of_birth`, `gender_identity`
FROM `People`;

SELECT `role_id`, `name`
FROM `Roles`;

SELECT `membership_id`, `club_id`, `person_id`, `role_id`
FROM `Membership`;

SELECT `event_id`, `club_id`, `organizer_id`, `name`, `description`, `time_start`, `time_end`
FROM `Events`;

SELECT `virtual_event_id`, `event_id`, `url`
FROM `VirtualEvents`;

SELECT `physical_event_id`, `event_id`, `address_id`
FROM `PhysicalEvents`;

-- Full setters (pk excluded)
INSERT INTO `Clubs` (`name`, `date_created`, `is_active`) 
VALUES (@club_name, @date_created, @is_active);

INSERT INTO `Addresses` (`country_code`, `zip_code`, `address_ln1`, `address_ln2`, `city`, `state`) 
VALUES (@country_code, @zip_code, @address_ln1, @address_ln2, @city, @state);

INSERT INTO `People` (`address_id`, `email`, `onid`, `phone_number`, `date_of_birth`, `gender_identity`) 
VALUES (@address_id, @email, @onid, @phone_number, @date_of_birth, @gender_identity);

INSERT INTO `Roles` (`name`) 
VALUES (@role_name);

INSERT INTO `Membership` (`club_id`, `person_id`, `role_id`) 
VALUES (@club_id, @person_id, @role_id);

INSERT INTO `Events` (`club_id`, `organizer_id`, `name`, `description`, `time_start`, `time_end`) 
VALUES (@club_id, @organizer_id, @event_name, @description, @time_start, @time_end);

INSERT INTO `VirtualEvents` (`event_id`, `url`) 
VALUES (@event_id, @url);

INSERT INTO `PhysicalEvents` (`event_id`, `address_id`) 
VALUES (@event_id, @address_id);

-- Full deleters
DELETE FROM `Clubs` 
WHERE `club_id` = @club_id;

DELETE FROM `Addresses` 
WHERE `address_id` = @address_id;

DELETE FROM `People` 
WHERE `person_id` = @person_id;

DELETE FROM `Roles` 
WHERE `role_id` = @role_id;

DELETE FROM `Membership` 
WHERE `membership_id` = @membership_id;

DELETE FROM `Events` 
WHERE `event_id` = @event_id;

DELETE FROM `VirtualEvents` 
WHERE `event_id` = @event_id;

DELETE FROM `PhysicalEvents` 
WHERE `event_id` = @event_id;
