import { createClient } from '@supabase/supabase-js';
import fs from 'fs';
import path from 'path';

// Manual env parser in case dotenv is not installed
function loadEnv() {
  const envPaths = ['.env', 'src-tauri/.env', '../.env'];
  for (const envPath of envPaths) {
    const fullPath = path.resolve(envPath);
    if (fs.existsSync(fullPath)) {
      const content = fs.readFileSync(fullPath, 'utf-8');
      const config = {};
      content.split('\n').forEach(line => {
        const parts = line.split('=');
        if (parts.length >= 2) {
          const key = parts[0].trim();
          const value = parts.slice(1).join('=').trim().replace(/^['"]|['"]$/g, '');
          config[key] = value;
        }
      });
      return config;
    }
  }
  return {};
}

const env = loadEnv();
const supabaseUrl = env.SUPABASE_URL || process.env.SUPABASE_URL;
const supabaseKey = env.SUPABASE_ANON_KEY || process.env.SUPABASE_ANON_KEY;

if (!supabaseUrl || !supabaseKey) {
  console.error('❌ Error: SUPABASE_URL or SUPABASE_ANON_KEY not found in .env file.');
  console.log('Please copy .env.example to .env and fill in your Supabase credentials.');
  process.exit(1);
}

const supabase = createClient(supabaseUrl, supabaseKey);

const mockProblems = [
  {
    id: "prob_hello",
    title: "สวัสดีชาวโลก (Hello World)",
    category: "Input / Output",
    description: "เขียนโปรแกรมที่แสดงข้อความ 'Hello, World!' ออกทางจอภาพหลัก (standard output)",
    input_specification: "ไม่มีข้อมูลเข้าสำหรับโจทย์ข้อนี้",
    output_specification: "แสดงข้อความ 'Hello, World!' บนบรรทัดเดียว"
  },
  {
    id: "prob_echo",
    title: "สะท้อนข้อมูลเข้า (Echo Input)",
    category: "Input / Output",
    description: "อ่านข้อความ 1 บรรทัดจากอุปกรณ์อินพุตหลัก (standard input) แล้วแสดงผลข้อความดังกล่าวออกไปเหมือนเดิมทุกประการ",
    input_specification: "ข้อความ 1 บรรทัด",
    output_specification: "แสดงข้อความเดิมที่รับเข้ามา"
  },
  {
    id: "prob_square",
    title: "กำลังสองของตัวเลข (Square of Number)",
    category: "Conditions",
    description: "รับจำนวนเต็ม N หาก N เป็นจำนวนเต็มบวก ให้แสดงกำลังสองของ N หาก N มีค่าน้อยกว่าหรือเท่ากับศูนย์ ให้แสดง 0",
    input_specification: "จำนวนเต็ม N จำนวน 1 ตัว",
    output_specification: "แสดงกำลังสองของ N หรือเลข 0"
  },
  {
    id: "prob_sum",
    title: "ผลรวมของตัวเลข 1 ถึง N (Sum of N Numbers)",
    category: "Loops",
    description: "เขียนโปรแกรมที่คำนวณหาผลรวมของจำนวนเต็มทั้งหมดตั้งแต่ 1 ถึง N (รวม N ด้วย)",
    input_specification: "จำนวนเต็มบวก N จำนวน 1 ตัว",
    output_specification: "แสดงจำนวนเต็ม 1 ตัวซึ่งเป็นผลรวมที่คำนวณได้"
  },
  {
    id: "prob_double",
    title: "เพิ่มค่าสองเท่า (Double it)",
    category: "Functions",
    description: "เขียนโปรแกรมที่รับตัวเลข N และแสดงผลลัพธ์เป็นสองเท่าของ N",
    input_specification: "ตัวเลข N จำนวน 1 ตัว",
    output_specification: "แสดงผลลัพธ์ N คูณด้วย 2"
  },
  {
    id: "prob_max",
    title: "ค้นหาค่าสูงสุด (Find Maximum)",
    category: "Lists",
    description: "รับชุดตัวเลขที่คั่นด้วยช่องว่าง ค้นหาและแสดงผลลัพธ์ที่มีค่ามากที่สุดในชุดตัวเลขนั้น",
    input_specification: "ชุดจำนวนเต็มคั่นด้วยช่องว่างในบรรทัดเดียว",
    output_specification: "แสดงค่าที่มากที่สุด"
  }
];

const mockTestCases = [
  // Hello World
  { id: "tc_hello_pub", problem_id: "prob_hello", input: "", expected_output: "Hello, World!\n", is_public: true },
  { id: "tc_hello_priv", problem_id: "prob_hello", input: "", expected_output: "Hello, World!\n", is_public: false },
  
  // Echo Input
  { id: "tc_echo_pub", problem_id: "prob_echo", input: "antigravity", expected_output: "antigravity\n", is_public: true },
  { id: "tc_echo_priv", problem_id: "prob_echo", input: "test_run", expected_output: "test_run\n", is_public: false },

  // Square of Number
  { id: "tc_sq_pub1", problem_id: "prob_square", input: "5", expected_output: "25\n", is_public: true },
  { id: "tc_sq_pub2", problem_id: "prob_square", input: "-3", expected_output: "0\n", is_public: true },
  { id: "tc_sq_priv1", problem_id: "prob_square", input: "10", expected_output: "100\n", is_public: false },
  { id: "tc_sq_priv2", problem_id: "prob_square", input: "0", expected_output: "0\n", is_public: false },

  // Sum of N
  { id: "tc_sum_pub", problem_id: "prob_sum", input: "5", expected_output: "15\n", is_public: true },
  { id: "tc_sum_priv1", problem_id: "prob_sum", input: "10", expected_output: "55\n", is_public: false },
  { id: "tc_sum_priv2", problem_id: "prob_sum", input: "1", expected_output: "1\n", is_public: false },

  // Double it
  { id: "tc_db_pub", problem_id: "prob_double", input: "4.5", expected_output: "9.0\n", is_public: true },
  { id: "tc_db_priv", problem_id: "prob_double", input: "7", expected_output: "14.0\n", is_public: false },

  // Find Maximum
  { id: "tc_max_pub", problem_id: "prob_max", input: "3 7 2 9 1", expected_output: "9\n", is_public: true },
  { id: "tc_max_priv1", problem_id: "prob_max", input: "-5 -10 -2 -8", expected_output: "-2\n", is_public: false },
  { id: "tc_max_priv2", problem_id: "prob_max", input: "42", expected_output: "42\n", is_public: false }
];

async function seed() {
  console.log('🚀 Seeding Supabase database...');

  // 1. Seed problems
  console.log('Inserting problems...');
  const { error: probError } = await supabase
    .from('problems')
    .upsert(mockProblems, { onConflict: 'id' });

  if (probError) {
    console.error('❌ Error inserting problems:', probError.message);
    return;
  }
  console.log('✅ Problems seeded successfully!');

  // 2. Seed test cases (Splitting into public and private)
  console.log('Inserting public test cases...');
  const publicCases = mockTestCases
    .filter(tc => tc.is_public)
    .map(({ id, problem_id, input, expected_output }) => ({ id, problem_id, input, expected_output }));

  const { error: pubError } = await supabase
    .from('public_test_cases')
    .upsert(publicCases, { onConflict: 'id' });

  if (pubError) {
    console.error('❌ Error inserting public test cases:', pubError.message);
    return;
  }
  console.log('✅ Public test cases seeded successfully!');

  console.log('Inserting private test cases...');
  const privateCases = mockTestCases
    .filter(tc => !tc.is_public)
    .map(({ id, problem_id, input, expected_output }) => ({ id, problem_id, input, expected_output }));

  const { error: privError } = await supabase
    .from('private_test_cases')
    .upsert(privateCases, { onConflict: 'id' });

  if (privError) {
    console.error('❌ Error inserting private test cases:', privError.message);
    return;
  }
  console.log('✅ Private test cases seeded successfully!');
  console.log('🎉 Database seeding completed successfully!');
}

seed();
