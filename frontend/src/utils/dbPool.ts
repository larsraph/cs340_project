import mysql from 'mysql2/promise';

const dbPassword = process.env.DB_PASSWORD;

if (!dbPassword || !dbPassword.trim()) {
  throw new Error(
    'Missing required environment variable: DB_PASSWORD. On Linux/macOS, run `export DB_PASSWORD=your_password && npm run dev` or create a frontend/.env file with `DB_PASSWORD=your_password`.'
  );
}

export const pool: any = mysql.createPool({
  waitForConnections: true,
  connectionLimit: 10,
  host: 'classmysql.engr.oregonstate.edu',
  user: 'cs340_larsraph',
  password: dbPassword,
  database: 'cs340_larsraph',
});
