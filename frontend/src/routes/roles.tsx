import { Title } from "@solidjs/meta";
import DbFetchTable from "~/components/DbFetchTable";

export default function Roles() {
  return (
    <main>
      <Title>Roles</Title>
      <h1>Roles</h1>
      <p class="page-description">
The Roles page defines positions that members can hold within clubs, such as
Role ID, and Name. These roles help track responsibilities within
each organization.
</p>
      <DbFetchTable table="Roles" />
    </main>
  );
}
