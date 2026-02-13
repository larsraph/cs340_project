import { Title } from "@solidjs/meta";
import DbFetchTable from "~/components/DbFetchTable";

export default function PhysicalEvents() {
  return (
    <main>
      <Title>PhysicalEvents</Title>
      <h1>PhysicalEvents</h1>
      <DbFetchTable table="PhysicalEvents" />
    </main>
  );
}
