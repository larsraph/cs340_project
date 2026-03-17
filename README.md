# Beaver Club Management System

## Overview

The Beaver Club Management System is a database-driven web application designed to manage student organizations and events at Oregon State University.

Many student organizations rely on spreadsheets to track membership, events, and participation. As organizations grow, spreadsheets become difficult to manage and prone to data inconsistencies. This project provides a structured database system that allows users to manage clubs, members, events, and roles in a centralized application.

The system supports multiple relationships among members, clubs, and events and allows users to perform full CRUD (Create, Read, Update, Delete) operations through a web interface.

---

## Team Members

- **Raphael Larsen**
- **America Pacheco**

---

## Features

The system allows users to manage the following entities:

### Clubs
- Store club information
- Track creation date and active status
- Link clubs to events and members

### People
- Store member information, including contact details
- Track participation across clubs and events

### Events
- Manage club events
- Track organizers and event schedules
- Support both **physical events** and **virtual events**

### Membership
- Connect people to clubs
- Assign roles to members within clubs

### Roles
- Define positions such as President, Organizer, or Member

### Addresses
- Store address information for members and physical events

---

## Database Design

The database supports several relationships:

- A **club can host many events**
- **Members can belong to multiple clubs**
- **Members can attend multiple events**
- Events can be **virtual or physical**
- Membership connects **people, clubs, and roles**

This structure allows the system to track participation across organizations while maintaining data consistency.

---

## Setup Instructions

### Prerequisites
- Node.js (v18 or higher recommended)
- npm
- MySQL

### Installation

git clone <repository-url>
cd cs340_project/frontend
npm install
npm run dev

## Citations 
This project was developed with reference to course materials and external tools. Portions of the code and design were adapted from the following sources:

** Module 4: Exploration - SQL JOINS**
Source code and structure adapted from the provided materials 
[Canvas Link] https://canvas.oregonstate.edu/courses/2031764/pages/exploration-sql-joins?module_item_id=26243394

** Module 5: Exploration - MYSQL Cascade**
Source code and structure adapted from the provided materials 
[Canvas Link] https://canvas.oregonstate.edu/courses/2031764/pages/exploration-mysql-cascade?module_item_id=26243410 

** Module 6: Database Application Design**
Source code and structure adapted from the provided materials 
[Canvas] https://canvas.oregonstate.edu/courses/2031764/pages/exploration-database-application-design?module_item_id=26243417 

** Module 8: Implementing CUD operations in your app**
Source code and structure adapted from the provided materials 
[Canvas] https://canvas.oregonstate.edu/courses/2031764/pages/exploration-implementing-cud-operations-in-your-app?module_item_id=26243436 

** AI-Assisted Development Tools**
GitHub Copilot (VS Code Extension)
Used to assist with code suggestions, syntax completion, and debugging. All generated code was reviewed and modified by the authors.

### Design Inspiration

**OSU Ideal-Logic Club Management System**  
This project was conceptually inspired by Oregon State University's student organization management platform, including its approach to organizing clubs and events.

  - https://see.oregonstate.edu/ela/clubs-organizations  
  - https://see.oregonstate.edu/ela/find-clubs  
