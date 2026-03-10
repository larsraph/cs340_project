/*
 * Authored and maintained by GitHub Copilot (GPT-5.3-Codex)
 * Date: March 2, 2026
 */

/* Sourced from Activity 2 - Connect a Webapp */

import { json } from '@solidjs/router';
import mysql from 'mysql2/promise';
import { broadcastDbUpdate } from '~/utils/realtime';

const pool = mysql.createPool({
    waitForConnections: true,
    connectionLimit   : 10,
    host              : 'classmysql.engr.oregonstate.edu',
    user              : 'cs340_larsraph',
    password          : '5310',
    database          : 'cs340_larsraph'
});

const escapeValue = (value: any): string => {
  if (value === null || value === undefined) {
    return 'NULL';
  }
  if (typeof value === 'string') {
    return `'${value.replace(/'/g, "''")}'`;
  }
  if (typeof value === 'boolean') {
    return value ? '1' : '0';
  }
  return String(value);
};

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

export async function POST({ params, request }: { params: { table: string }; request: Request }) {
  try {
    const tableName = params.table;
    const body = await request.json();
    const { data } = body;

    if (!data || typeof data !== 'object') {
      return json({ error: 'Invalid request body' }, { status: 400 });
    }

    const columns = Object.keys(data);
    const values = columns.map(col => escapeValue(data[col]));
    
    const query = `INSERT INTO \`${tableName}\` (${columns.map(c => `\`${c}\``).join(', ')}) VALUES (${values.join(', ')})`;
    console.log('INSERT query:', query);

    const connection = await pool.getConnection();
    await connection.query(query);
    connection.release();

    broadcastDbUpdate({ table: tableName, action: 'insert' });

    return json({ success: true, message: 'Row created successfully' });
  } catch (error) {
    console.error('Insert error:', error);
    return json({ error: String(error) }, { status: 500 });
  }
}

export async function PUT({ params, request }: { params: { table: string }; request: Request }) {
  try {
    const tableName = params.table;
    const body = await request.json();
    const { data, where } = body;

    if (!data || !where) {
      return json({ error: 'Invalid request body' }, { status: 400 });
    }

    const setClauses = Object.entries(data)
      .map(([col, val]) => `\`${col}\` = ${escapeValue(val)}`)
      .join(', ');

    const whereClauses = Object.entries(where)
      .map(([col, val]) => `\`${col}\` = ${escapeValue(val)}`)
      .join(' AND ');

    const query = `UPDATE \`${tableName}\` SET ${setClauses} WHERE ${whereClauses}`;
    console.log('UPDATE query:', query);

    const connection = await pool.getConnection();
    await connection.query(query);
    connection.release();

    broadcastDbUpdate({ table: tableName, action: 'update' });

    return json({ success: true, message: 'Row updated successfully' });
  } catch (error) {
    console.error('Update error:', error);
    return json({ error: String(error) }, { status: 500 });
  }
}

export async function DELETE({ params, request }: { params: { table: string }; request: Request }) {
  try {
    const tableName = params.table;
    const body = await request.json();
    const { where } = body;

    if (!where) {
      return json({ error: 'Invalid request body' }, { status: 400 });
    }

    const whereClauses = Object.entries(where)
      .map(([col, val]) => `\`${col}\` = ${escapeValue(val)}`)
      .join(' AND ');

    const query = `DELETE FROM \`${tableName}\` WHERE ${whereClauses}`;
    console.log('DELETE query:', query);

    const connection = await pool.getConnection();
    await connection.query(query);
    connection.release();

    broadcastDbUpdate({ table: tableName, action: 'delete' });

    return json({ success: true, message: 'Row deleted successfully' });
  } catch (error) {
    console.error('Delete error:', error);
    return json({ error: String(error) }, { status: 500 });
  }
}
