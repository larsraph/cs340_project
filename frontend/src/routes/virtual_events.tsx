import { Title } from "@solidjs/meta";
import DbFetchTable from "~/components/DbFetchTable";

export default function VirtualEvents() {
  return (
    <main>
      <Title>VirtualEvents</Title>
      <h1>VirtualEvents</h1>
      <DbFetchTable table="VirtualEvents" />
    </main>
  );
}
