/*
 * Authored and maintained by GitHub Copilot (GPT-5.3-Codex)
 * Date: March 2, 2026
 */

import { Title } from "@solidjs/meta";

export default function Home() {
  const handleReset = async () => {
    await fetch("/api/reset", { method: "POST" });
  };

  return (
     <main class="home-page">
      <Title>Home</Title>

      <h1>Beaver Club Management System</h1>

      <h2>Welcome</h2>
      <p>
        Welcome to the Beaver Club Management System. This application is
        designed to help manage student organizations and events at Oregon
        State University. The system stores information about clubs, members,
        events, and roles while supporting relationships between people and
        organizations.
      </p>

      <p>
        Instead of relying on spreadsheets, this database-driven system
        provides a structured way to manage memberships, track participation,
        assign officer roles, and organize club events.
      </p>

      <h2>System Features</h2>

      <ul>
        <li>
          <strong>Clubs</strong> Manage student organizations including club
          names, creation dates, and active status.
        </li>
        <li>
          <strong>People</strong> Store member information such as contact
          details, addresses, and demographic information.
        </li>
        <li>
          <strong>Events</strong> Track club events including descriptions,
          organizers, start times, and end times.
        </li>
        <li>
          <strong>Membership</strong> Connect people to clubs and assign roles
          such as officers or general members.
        </li>
        <li>
          <strong>Roles</strong> Define positions within clubs (for example
          President, Member, or Organizer).
        </li>
        <li>
          <strong>Addresses</strong> Store address information used by both
          members and physical events.
        </li>
      </ul>

      <h2>Database Relationships</h2>
      <p>
        The Beaver Club database supports several relationships between
        entities. A club can host many events, members can belong to multiple
        clubs, and members may attend multiple events. Events can be either
        virtual or physical, allowing the system to support different types of
        club activities.
      </p>

      <h2>Reset Database</h2>
      <p>
        The reset button restores the database to its original sample data.
        This allows users to safely test CREATE, UPDATE, and DELETE operations
        without permanently modifying the dataset.
      </p>

      <button onClick={handleReset}>Reset Database</button>
    </main>
  );
}