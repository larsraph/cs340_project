/* Sourced from Activity 2 - Connect a Webapp */

import { json } from '@solidjs/router';
import mysql from 'mysql2/promise';

const pool = mysql.createPool({
    waitForConnections: true,
    connectionLimit   : 10,
    host              : 'classmysql.engr.oregonstate.edu',
    user              : 'cs340_larsraph',
    password          : '5310',
    database          : 'cs340_larsraph'
});

export async function GET({ params }: { params: { table: string } }) {
  try {
    console.log(`Fetching data from table: ${params.table}`);
    const tableName = params.table;

    const [rows] = await pool.query(`SELECT * FROM ${tableName}`);

    return json(rows);
  } catch (error) {
    console.error('Database error:', error);
    return json({ error: 'Failed to fetch data' }, { status: 500 });
  }
}
