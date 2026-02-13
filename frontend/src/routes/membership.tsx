import { Title } from "@solidjs/meta";
import DbFetchTable from "~/components/DbFetchTable";

export default function Membership() {
  return (
    <main>
      <Title>Membership</Title>
      <h1>Membership</h1>
      <DbFetchTable table="Membership" />
    </main>
  );
}
