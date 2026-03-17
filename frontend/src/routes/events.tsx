import { Title } from "@solidjs/meta";
import DbFetchTable from "~/components/DbFetchTable";

export default function Events() {
  return (
    <main>
      <Title>Events</Title>
      <h1>Events</h1>
      <p class="page-description">
        This page tracks events hosted by clubs. Events include details such as the
        Event ID, Name, Description, scheduled start & end times, Club ID and Organizer ID.
      </p>
      <DbFetchTable table="Events" />
    </main>
  );
}
