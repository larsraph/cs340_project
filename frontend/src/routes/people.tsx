import { Title } from "@solidjs/meta";
import DbFetchTable from "~/components/DbFetchTable";

export default function People() {
  return (
    <main>
      <Title>People</Title>
      <h1>People</h1>
        <p class="page-description">
    This page manages member records in the Beaver Club system. You can view,
    add, update, and remove people while storing contact information such as
    email, ONID, phone number, DOB, Gender, and address.
  </p>
      <DbFetchTable table="People" />
    </main>
  );
}
