import { Title } from "@solidjs/meta";
import DbFetchTable from "~/components/DbFetchTable";

export default function Events() {
  return (
    <main>
      <Title>Events</Title>
      <h1>Events</h1>
      <DbFetchTable table="Events" />
    </main>
  );
}
