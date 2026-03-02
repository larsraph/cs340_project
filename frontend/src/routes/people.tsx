import { Title } from "@solidjs/meta";
import DbFetchTable from "~/components/DbFetchTable";

export default function People() {
  return (
    <main>
      <Title>People</Title>
      <h1>People</h1>
      <DbFetchTable table="People" />
    </main>
  );
}
