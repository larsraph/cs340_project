import { Title } from "@solidjs/meta";
import DbFetchTable from "~/components/DbFetchTable";

export default function VirtualEvents() {
  return (
    <main>
      <Title>VirtualEvents</Title>
      <h1>VirtualEvents</h1>
      <p class="page-description">
Virtual events represent online club activities. Each virtual event stores an Event ID and a
URL/meeting link used for participants to join remotely.
</p>
      <DbFetchTable table="VirtualEvents" />
    </main>
  );
}
