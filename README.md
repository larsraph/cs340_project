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


## Project Structure
