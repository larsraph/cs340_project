/*
 * Authored and maintained by GitHub Copilot (GPT-5.3-Codex)
 * Date: March 2, 2026
 */

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

export async function POST() {
  try {
    console.log('Calling reset() procedure');
    const connection = await pool.getConnection();
    
    await connection.query('CALL reset()');
    connection.release();

    broadcastDbUpdate({ table: '*', action: 'reset' });

    return json({ success: true, message: 'Database reset successfully' });
  } catch (error) {
    console.error('Reset error:', error);
    return json({ error: 'Failed to reset database' }, { status: 500 });
  }
}
