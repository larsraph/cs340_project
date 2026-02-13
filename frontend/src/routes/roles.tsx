import { Title } from "@solidjs/meta";
import DbFetchTable from "~/components/DbFetchTable";

export default function Roles() {
  return (
    <main>
      <Title>Roles</Title>
      <h1>Roles</h1>
      <DbFetchTable table="Roles" />
    </main>
  );
}
