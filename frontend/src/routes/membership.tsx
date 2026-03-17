import { Title } from "@solidjs/meta";
import DbFetchTable from "~/components/DbFetchTable";

export default function Membership() {
  return (
    <main>
      <Title>Membership</Title>
      <h1>Membership</h1>
      <p class="page-description">
        This page manages the relationship between people and clubs. Membership records
        connect members to organizations and assign their roles within those clubs.
      </p>
      <DbFetchTable table="Membership" />
    </main>
  );
}
