import { Title } from "@solidjs/meta";
import DbFetchTable from "~/components/DbFetchTable";

export default function Addresses() {
  return (
    <main>
      <Title>Addresses</Title>
      <h1>Addresses</h1>
      <DbFetchTable table="Addresses" />
    </main>
  );
}
