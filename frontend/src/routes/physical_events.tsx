import { Title } from "@solidjs/meta";
import DbFetchTable from "~/components/DbFetchTable";

export default function PhysicalEvents() {
  return (
    <main>
      <Title>PhysicalEvents</Title>
      <h1>PhysicalEvents</h1>
      <p class="page-description">
Physical events represent in-person club activities. These events are connected
to a Event ID and Address ID of where the event will take place.
</p>
      <DbFetchTable table="PhysicalEvents" />
    </main>
  );
}
