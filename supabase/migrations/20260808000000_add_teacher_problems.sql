-- Generated from python_problem_set_30questions.zip.
-- Teacher Problems is intentionally excluded from the daily quota by default;
-- the app exposes the full catalogue in its dedicated featured section.
begin;

insert into category_configs (category, target_count)
values ('Teacher Problems', 0)
on conflict (category) do update set target_count = excluded.target_count;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_01', 'คำนวณพื้นที่และเส้นรอบรูปสี่เหลี่ยมผืนผ้า', 'Teacher Problems', 'เขียนโปรแกรมรับค่าความกว้าง (width) และความยาว (length) ของสี่เหลี่ยมผืนผ้า
เป็นจำนวนเต็ม 2 ค่า คั่นด้วยช่องว่างในบรรทัดเดียว แล้วคำนวณ **พื้นที่** และ **เส้นรอบรูป**', 'บรรทัดเดียว มีตัวเลขจำนวนเต็ม 2 ค่า คั่นด้วยช่องว่าง: `width length`', 'บรรทัดเดียว: `พื้นที่ เส้นรอบรูป` คั่นด้วยช่องว่าง')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_02', 'ตรวจสอบเลขคู่หรือเลขคี่', 'Teacher Problems', 'รับจำนวนเต็ม n หนึ่งค่า แล้วพิมพ์ว่าเป็นเลขคู่ (`EVEN`) หรือเลขคี่ (`ODD`)', 'บรรทัดเดียว มีจำนวนเต็ม n (n อาจเป็นค่าลบได้)', 'พิมพ์ `EVEN` หรือ `ODD`')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_03', 'ตัดเกรดจากคะแนนสอบ', 'Teacher Problems', 'รับคะแนนสอบ (0-100) เป็นจำนวนเต็ม แล้วตัดสินเกรดตามเกณฑ์ต่อไปนี้
- คะแนน >= 80 : เกรด A
- คะแนน 70-79 : เกรด B
- คะแนน 60-69 : เกรด C
- คะแนน 50-59 : เกรด D
- คะแนนน้อยกว่า 50 : เกรด F', 'บรรทัดเดียว มีจำนวนเต็มคะแนน 1 ค่า', 'พิมพ์ตัวอักษรเกรด (A/B/C/D/F)')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_04', 'ผลรวมและค่าเฉลี่ยของจำนวนในลิสต์', 'Teacher Problems', 'รับจำนวนเต็ม n (จำนวนตัวเลข) ในบรรทัดแรก และตัวเลข n ตัวคั่นด้วยช่องว่างในบรรทัดที่สอง
ให้หาผลรวมและค่าเฉลี่ยของตัวเลขทั้งหมด (ค่าเฉลี่ยปัดเป็นทศนิยม 2 ตำแหน่ง)', 'บรรทัดที่ 1: จำนวนเต็ม n
บรรทัดที่ 2: ตัวเลข n ตัว คั่นด้วยช่องว่าง', 'บรรทัดเดียว: `ผลรวม ค่าเฉลี่ย` (ค่าเฉลี่ยแสดงทศนิยม 2 ตำแหน่ง)')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_05', 'นับจำนวนสระในคำ', 'Teacher Problems', 'รับข้อความ 1 บรรทัด (ตัวอักษรภาษาอังกฤษล้วน ไม่มีช่องว่าง) แล้วนับจำนวนสระ
(a, e, i, o, u ทั้งตัวพิมพ์เล็กและใหญ่)', 'บรรทัดเดียว เป็นข้อความภาษาอังกฤษ', 'จำนวนเต็มแสดงจำนวนสระที่พบ')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_06', 'ตรวจสอบจำนวนเฉพาะ', 'Teacher Problems', 'รับจำนวนเต็มบวก n แล้วตรวจสอบว่าเป็นจำนวนเฉพาะ (Prime) หรือไม่', 'บรรทัดเดียว จำนวนเต็ม n (1 <= n <= 10^6)', 'พิมพ์ `PRIME` หรือ `NOT PRIME`')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_07', 'FizzBuzz ขยาย', 'Teacher Problems', 'รับจำนวนเต็ม n แล้วพิมพ์ตัวเลขตั้งแต่ 1 ถึง n โดยแทนที่ด้วยกฎ:
- หารด้วย 3 ลงตัว พิมพ์ `Fizz`
- หารด้วย 5 ลงตัว พิมพ์ `Buzz`
- หารด้วยทั้ง 3 และ 5 ลงตัว พิมพ์ `FizzBuzz`
- หารด้วย 7 ลงตัว (แต่ไม่ใช่ 3 หรือ 5) พิมพ์ `Bang`
- นอกเหนือจากนั้นพิมพ์ตัวเลขเดิม
พิมพ์แต่ละค่าคั่นด้วยช่องว่างในบรรทัดเดียว', 'บรรทัดเดียว จำนวนเต็ม n (1 <= n <= 100)', 'บรรทัดเดียว ผลลัพธ์คั่นด้วยช่องว่าง')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_08', 'แฟกทอเรียลและการจัดหมู่ (Combination)', 'Teacher Problems', 'รับจำนวนเต็ม n และ r (0 <= r <= n <= 20) แล้วคำนวณค่า `nCr` (จำนวนวิธีเลือก r สิ่งจาก n สิ่ง)
โดยใช้สูตร `nCr = n! / (r! * (n-r)!)`', 'บรรทัดเดียว: `n r` คั่นด้วยช่องว่าง', 'จำนวนเต็มค่า nCr')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_09', 'ค่าสูงสุด ต่ำสุด และฐานนิยม (Mode)', 'Teacher Problems', 'รับจำนวนเต็ม n และตัวเลข n ตัว แล้วหาค่ามากที่สุด ค่าน้อยที่สุด
และฐานนิยม (ค่าที่ปรากฏบ่อยที่สุด ถ้ามีหลายค่าเท่ากันให้เลือกค่าที่น้อยที่สุด)', 'บรรทัดที่ 1: จำนวนเต็ม n
บรรทัดที่ 2: ตัวเลข n ตัวคั่นด้วยช่องว่าง', 'บรรทัดเดียว: `max min mode`')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_10', 'ตรวจสอบพาลินโดรมของประโยค', 'Teacher Problems', 'รับข้อความ 1 บรรทัด (อาจมีช่องว่างและตัวพิมพ์ใหญ่-เล็กปนกัน)
ให้ตรวจสอบว่าข้อความนั้นเป็นพาลินโดรมหรือไม่ โดย **ไม่นับช่องว่างและตัวพิมพ์เล็ก-ใหญ่**', 'บรรทัดเดียว เป็นข้อความ', 'พิมพ์ `YES` หรือ `NO`')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_11', 'จัดการสต็อกสินค้าด้วย Dictionary', 'Teacher Problems', 'ร้านค้ามีสต็อกสินค้าเริ่มต้น รับจำนวนสินค้าตั้งต้น n รายการ (ชื่อ จำนวน) ตามด้วยจำนวนคำสั่งซื้อ m คำสั่ง
แต่ละคำสั่งซื้อระบุ ชื่อสินค้าและจำนวนที่ต้องการซื้อ ให้ตัดสต็อกออกถ้ามีเพียงพอ
ถ้าสต็อกไม่พอหรือไม่มีสินค้านั้น ให้พิมพ์ `FAILED` สำหรับคำสั่งนั้น มิฉะนั้นพิมพ์ `OK`
เมื่อจบให้พิมพ์สต็อกคงเหลือของสินค้าทั้งหมด เรียงตามชื่อ (ก-ฮ/A-Z)', 'บรรทัดที่ 1: จำนวนเต็ม n
n บรรทัดถัดไป: `ชื่อสินค้า จำนวน`
บรรทัดถัดมา: จำนวนเต็ม m
m บรรทัดถัดไป: `ชื่อสินค้า จำนวนที่ต้องการซื้อ`', 'm บรรทัดแรก: ผลลัพธ์ `OK` หรือ `FAILED` ของแต่ละคำสั่งซื้อ ตามลำดับ
บรรทัดถัดไป: สต็อกคงเหลือของสินค้าแต่ละชนิด เรียงตามชื่อ รูปแบบ `ชื่อ จำนวน` บรรทัดละ 1 รายการ')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_12', 'ผลรวมเลขฟีโบนักชี', 'Teacher Problems', 'รับจำนวนเต็ม n แล้วหาผลรวมของเลขฟีโบนักชี n ตัวแรก
โดยกำหนดให้ F(1)=1, F(2)=1, F(3)=2, ...', 'บรรทัดเดียว จำนวนเต็ม n (1 <= n <= 50)', 'จำนวนเต็มผลรวม')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_13', 'ผลรวมแนวทแยงของเมทริกซ์', 'Teacher Problems', 'รับเมทริกซ์จัตุรัสขนาด n x n แล้วหาผลรวมของแนวทแยงหลัก (บนซ้ายลงล่างขวา)
และแนวทแยงรอง (บนขวาลงล่างซ้าย)', 'บรรทัดที่ 1: จำนวนเต็ม n
n บรรทัดถัดไป: แต่ละบรรทัดมีตัวเลข n ตัวคั่นด้วยช่องว่าง แทนแถวของเมทริกซ์', 'บรรทัดเดียว: `ผลรวมแนวทแยงหลัก ผลรวมแนวทแยงรอง`')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_14', 'แปลงเลขฐานสิบเป็นฐานสองและฐานแปด', 'Teacher Problems', 'รับจำนวนเต็มบวก n ในฐานสิบ แล้วแปลงเป็นเลขฐานสอง (binary) และฐานแปด (octal)
**ห้ามใช้ฟังก์ชันสำเร็จรูป** `bin()` หรือ `oct()` ให้เขียนอัลกอริทึมการหารเก็บเศษเอง', 'บรรทัดเดียว จำนวนเต็ม n (0 <= n <= 100000)', 'บรรทัดเดียว: `เลขฐานสอง เลขฐานแปด`')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_15', 'ค่ามัธยฐาน (Median)', 'Teacher Problems', 'รับจำนวนเต็ม n และตัวเลข n ตัว แล้วหาค่ามัธยฐานของชุดข้อมูล
- ถ้า n เป็นเลขคี่ ค่ามัธยฐานคือค่ากลางหลังจากเรียงลำดับ
- ถ้า n เป็นเลขคู่ ค่ามัธยฐานคือค่าเฉลี่ยของสองค่ากลาง', 'บรรทัดที่ 1: จำนวนเต็ม n
บรรทัดที่ 2: ตัวเลข n ตัว คั่นด้วยช่องว่าง', 'ค่ามัธยฐาน แสดงทศนิยม 2 ตำแหน่งเสมอ')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_16', 'เข้ารหัสซีซาร์ (Caesar Cipher)', 'Teacher Problems', 'รับข้อความ (ตัวอักษรภาษาอังกฤษพิมพ์เล็กล้วน ไม่มีช่องว่าง) และจำนวนเต็ม k (shift)
ให้เข้ารหัสด้วยวิธี Caesar Cipher คือเลื่อนตัวอักษรแต่ละตัวไปข้างหน้า k ตำแหน่งตามลำดับ a-z
(วนกลับมาที่ ''a'' เมื่อเลยตัว ''z'')', 'บรรทัดที่ 1: ข้อความ (a-z พิมพ์เล็ก)
บรรทัดที่ 2: จำนวนเต็ม k (0 <= k <= 100)', 'ข้อความที่เข้ารหัสแล้ว')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_17', 'ตรวจสอบแอนนาแกรม (Anagram)', 'Teacher Problems', 'รับข้อความ 2 บรรทัด (ตัวอักษรภาษาอังกฤษ อาจมีตัวพิมพ์ใหญ่-เล็กปนกันและมีช่องว่าง)
ตรวจสอบว่าทั้งสองข้อความเป็นแอนนาแกรมกันหรือไม่ (ตัวอักษรชุดเดียวกัน จำนวนเท่ากัน
โดยไม่นับช่องว่างและไม่สนตัวพิมพ์ใหญ่-เล็ก)', 'บรรทัดที่ 1: ข้อความที่ 1
บรรทัดที่ 2: ข้อความที่ 2', 'พิมพ์ `YES` หรือ `NO`')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_18', 'หาจำนวนเฉพาะด้วย Sieve of Eratosthenes', 'Teacher Problems', 'รับจำนวนเต็ม n แล้วหาจำนวนเฉพาะทั้งหมดที่มีค่าไม่เกิน n โดย **ต้องใช้อัลกอริทึม
Sieve of Eratosthenes** (ห้ามตรวจสอบทีละตัวด้วยการหารแบบ solve6)', 'บรรทัดเดียว จำนวนเต็ม n (2 <= n <= 100000)', 'บรรทัดเดียว: จำนวนเฉพาะทั้งหมดที่ <= n คั่นด้วยช่องว่าง')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_19', 'ตรวจสอบวงเล็บสมดุลด้วย Stack', 'Teacher Problems', 'รับข้อความที่ประกอบด้วยวงเล็บ 3 ชนิด `()`, `[]`, `{}` เท่านั้น
ตรวจสอบว่าวงเล็บทั้งหมดปิดถูกต้องและสมดุลกันหรือไม่ (ต้องปิดตามลำดับที่เปิดล่าสุดก่อน)
**ต้องใช้โครงสร้างข้อมูลแบบ Stack** (ใช้ Python list เป็น stack ได้ ด้วย `append`/`pop`)', 'บรรทัดเดียว ข้อความที่มีเฉพาะอักขระ `()[]{}`', 'พิมพ์ `VALID` หรือ `INVALID`')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_20', 'จำลองคิวธนาคารด้วย Queue', 'Teacher Problems', 'ธนาคารมีเคาน์เตอร์บริการ 1 ช่อง ลูกค้า n คนมาต่อคิวตามลำดับ (คนแรกที่มาได้รับบริการก่อน)
แต่ละคนใช้เวลาให้บริการ (นาที) ตามที่กำหนด ให้หาว่าลูกค้าแต่ละคน **ต้องรอ (waiting time)**
กี่นาทีก่อนจะได้รับบริการ (waiting time ของคนแรกคือ 0)
**ต้องใช้โครงสร้างข้อมูลแบบ Queue** (ใช้ `collections.deque`)', 'บรรทัดที่ 1: จำนวนเต็ม n
บรรทัดที่ 2: เวลาให้บริการของลูกค้าแต่ละคน (นาที) คั่นด้วยช่องว่าง n ค่า', 'บรรทัดเดียว: waiting time ของลูกค้าแต่ละคนตามลำดับ คั่นด้วยช่องว่าง')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_21', 'จำลอง Linked List แบบง่ายด้วยคำสั่งแทรก/ลบ', 'Teacher Problems', 'จำลองรายการโยง (singly linked list) ของจำนวนเต็ม เริ่มต้นจากลิสต์ว่าง
รับจำนวนคำสั่ง m คำสั่ง โดยแต่ละคำสั่งเป็นหนึ่งในสามรูปแบบ:
- `APPEND x` : เพิ่มค่า x ต่อท้ายรายการ
- `PREPEND x` : เพิ่มค่า x ไว้หน้าสุดของรายการ
- `DELETE x` : ลบค่า x ตัวแรกที่พบในรายการ (ถ้ามี)

หลังทำคำสั่งครบทั้งหมดแล้ว ให้พิมพ์รายการที่เหลือตามลำดับจากหัวถึงท้าย', 'บรรทัดที่ 1: จำนวนเต็ม m
m บรรทัดถัดไป: คำสั่งตามรูปแบบด้านบน', 'บรรทัดเดียว: ค่าที่เหลือในรายการ คั่นด้วยช่องว่าง (ถ้าว่างให้พิมพ์ `EMPTY`)')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_22', 'หา GCD และ LCM ของจำนวนหลายตัว', 'Teacher Problems', 'รับจำนวนเต็ม n และตัวเลข n ตัว ให้หา **ห.ร.ม. (GCD)** และ **ค.ร.น. (LCM)**
ของตัวเลขทั้งหมดพร้อมกัน', 'บรรทัดที่ 1: จำนวนเต็ม n
บรรทัดที่ 2: ตัวเลข n ตัวคั่นด้วยช่องว่าง (ทุกตัวเป็นจำนวนเต็มบวก)', 'บรรทัดเดียว: `GCD LCM`')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_23', 'การคูณเมทริกซ์ (Matrix Multiplication)', 'Teacher Problems', 'รับเมทริกซ์ A ขนาด n x m และเมทริกซ์ B ขนาด m x p แล้วคำนวณผลคูณเมทริกซ์ A x B
**ห้ามใช้ไลบรารีสำเร็จรูปเช่น numpy** ให้เขียนลูปคูณเมทริกซ์เอง', 'บรรทัดที่ 1: `n m p`
n บรรทัดถัดไป: เมทริกซ์ A (แต่ละแถวมี m ค่า)
m บรรทัดถัดไป: เมทริกซ์ B (แต่ละแถวมี p ค่า)', 'n บรรทัด แสดงเมทริกซ์ผลลัพธ์ขนาด n x p แต่ละแถวคั่นด้วยช่องว่าง')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_24', 'หาความยาว Longest Common Subsequence (LCS)', 'Teacher Problems', 'รับข้อความ 2 บรรทัด (ตัวอักษรภาษาอังกฤษพิมพ์เล็กล้วน) ให้หาความยาวของ
Longest Common Subsequence (สายอักขระย่อยร่วมที่ยาวที่สุด ไม่จำเป็นต้องต่อเนื่องกัน
แต่ต้องเรียงลำดับตามต้นฉบับ) ของทั้งสองข้อความ', 'บรรทัดที่ 1: ข้อความที่ 1
บรรทัดที่ 2: ข้อความที่ 2', 'จำนวนเต็มความยาวของ LCS')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_25', 'จัดกลุ่มคำตามแอนนาแกรม', 'Teacher Problems', 'รับจำนวนเต็ม n และคำ n คำ (ตัวอักษรภาษาอังกฤษพิมพ์เล็ก) ให้จัดกลุ่มคำที่เป็นแอนนาแกรมกัน
ไว้ในกลุ่มเดียวกัน โดยแสดงแต่ละกลุ่มในหนึ่งบรรทัด เรียงคำภายในกลุ่มตามลำดับที่พบในข้อมูลนำเข้า
และเรียงกลุ่มตามลำดับที่คำแรกของกลุ่มปรากฏในข้อมูลนำเข้า', 'บรรทัดที่ 1: จำนวนเต็ม n
บรรทัดที่ 2: คำ n คำ คั่นด้วยช่องว่าง', 'พิมพ์แต่ละกลุ่มในหนึ่งบรรทัด คำในกลุ่มคั่นด้วยช่องว่าง')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_26', 'ระบบจัดการคะแนนนักเรียนด้วย Nested Dictionary', 'Teacher Problems', 'รับข้อมูลนักเรียน n คน แต่ละคนมีชื่อและคะแนนสอบ 3 วิชา (คณิต วิทย์ อังกฤษ)
ให้สร้างโครงสร้าง nested dictionary เก็บข้อมูล แล้วคำนวณ **คะแนนเฉลี่ยของแต่ละคน**
จากนั้นพิมพ์ชื่อและคะแนนเฉลี่ยของนักเรียน โดยเรียงจากคะแนนเฉลี่ย **มากไปน้อย**
(ถ้าคะแนนเท่ากันให้เรียงตามชื่อ ก-ฮ/A-Z)', 'บรรทัดที่ 1: จำนวนเต็ม n
n บรรทัดถัดไป: `ชื่อ คณิต วิทย์ อังกฤษ`', 'n บรรทัด: `ชื่อ ค่าเฉลี่ย` (ค่าเฉลี่ยแสดงทศนิยม 2 ตำแหน่ง) เรียงจากมากไปน้อย')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_27', 'คำนวณค่าเบี่ยงเบนมาตรฐาน (Standard Deviation)', 'Teacher Problems', 'รับจำนวนเต็ม n และตัวเลข n ตัว ให้คำนวณ **ค่าเฉลี่ย (mean)**, **ความแปรปรวน (variance)**
และ **ส่วนเบี่ยงเบนมาตรฐาน (standard deviation)** ของกลุ่มตัวอย่างประชากร (population)
โดยใช้สูตร:
variance = (Σ(x - mean)^2) / n
sd = sqrt(variance)', 'บรรทัดที่ 1: จำนวนเต็ม n
บรรทัดที่ 2: ตัวเลข n ตัว คั่นด้วยช่องว่าง', 'บรรทัดเดียว: `mean variance sd` (แต่ละค่าแสดงทศนิยม 2 ตำแหน่ง)')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_28', 'ตรวจสอบปีอธิกสุรทินและหาวันที่ในปีถัดไป', 'Teacher Problems', 'รับปี ค.ศ. (year), เดือน (month, 1-12), และวัน (day) เป็นจำนวนเต็ม 3 ค่า
1. ตรวจสอบว่าปีนั้นเป็นปีอธิกสุรทิน (leap year) หรือไม่
   (ปีอธิกสุรทิน: หารด้วย 4 ลงตัว และ (ไม่หารด้วย 100 ลงตัว หรือ หารด้วย 400 ลงตัว))
2. หาว่าวันถัดไป (วันที่ + 1) คือวันที่เท่าไหร่ เดือนอะไร ปีอะไร
   (ถ้าวันที่ที่รับมาเกินจำนวนวันในเดือนนั้นให้ถือว่า input ไม่ถูกต้อง ไม่ต้องตรวจสอบกรณีนี้)', 'บรรทัดเดียว: `year month day`', 'บรรทัดที่ 1: `LEAP` หรือ `NOT LEAP`
บรรทัดที่ 2: `year month day` ของวันถัดไป')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_29', 'ค้นหาแบบ Binary Search', 'Teacher Problems', 'รับจำนวนเต็ม n, ตัวเลข n ตัวที่**เรียงลำดับจากน้อยไปมากแล้ว**, และค่าที่ต้องการค้นหา target
ให้หาตำแหน่ง (index เริ่มจาก 0) ของ target ในลิสต์โดยใช้อัลกอริทึม **Binary Search**
ถ้าไม่พบให้พิมพ์ `-1`
**ห้ามใช้ `list.index()` หรือวนลูปเชิงเส้นตรงตรวจทีละตัว (linear search)**', 'บรรทัดที่ 1: จำนวนเต็ม n
บรรทัดที่ 2: ตัวเลข n ตัวเรียงจากน้อยไปมาก คั่นด้วยช่องว่าง
บรรทัดที่ 3: จำนวนเต็ม target', 'ตำแหน่ง (index) ของ target หรือ `-1` หากไม่พบ')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into problems (id, title, category, description, input_specification, output_specification) values ('teacher_30', 'หอคอยฮานอย (Tower of Hanoi)', 'Teacher Problems', 'รับจำนวนแผ่นดิสก์ n ให้จำลองการแก้ปัญหาหอคอยฮานอยด้วย **recursion**
(ย้ายแผ่นดิสก์ n แผ่นจากเสา A ไปเสา C โดยใช้เสา B เป็นตัวช่วย)
พิมพ์ลำดับขั้นตอนการย้ายทั้งหมด และจำนวนขั้นตอนรวม', 'บรรทัดเดียว จำนวนเต็ม n (1 <= n <= 10)', 'พิมพ์ทุกขั้นตอนการย้าย รูปแบบ `Move disk k from X to Y` (k คือหมายเลขแผ่นดิสก์ นับจากขนาดเล็กสุด=1)
บรรทัดสุดท้าย พิมพ์ `Total moves: จำนวนขั้นตอนทั้งหมด`')
on conflict (id) do update set
  title = excluded.title, category = excluded.category, description = excluded.description,
  input_specification = excluded.input_specification, output_specification = excluded.output_specification;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_01_sample', 'teacher_01', '4 6', '24 20')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_01_case_1', 'teacher_01', '3 8', '24 22')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_01_case_2', 'teacher_01', '10 2', '20 24')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_01_case_3', 'teacher_01', '7 7', '49 28')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_01_case_4', 'teacher_01', '1 1', '1 4')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_01_case_5', 'teacher_01', '15 9', '135 48')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_02_sample', 'teacher_02', '7', 'ODD')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_02_case_1', 'teacher_02', '13', 'ODD')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_02_case_2', 'teacher_02', '10', 'EVEN')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_02_case_3', 'teacher_02', '-3', 'ODD')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_02_case_4', 'teacher_02', '0', 'EVEN')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_02_case_5', 'teacher_02', '-8', 'EVEN')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_03_sample', 'teacher_03', '85', 'A')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_03_case_1', 'teacher_03', '91', 'A')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_03_case_2', 'teacher_03', '72', 'B')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_03_case_3', 'teacher_03', '63', 'C')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_03_case_4', 'teacher_03', '50', 'D')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_03_case_5', 'teacher_03', '42', 'F')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_04_sample', 'teacher_04', '4
2 4 6 8', '20 5.00')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_04_case_1', 'teacher_04', '3
5 10 15', '30 10.00')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_04_case_2', 'teacher_04', '3
10 20 30', '60 20.00')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_04_case_3', 'teacher_04', '5
1 2 3 4 5', '15 3.00')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_04_case_4', 'teacher_04', '1
7', '7 7.00')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_04_case_5', 'teacher_04', '6
5 5 5 5 5 5', '30 5.00')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_05_sample', 'teacher_05', 'ProgrammingIsFun', '5')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_05_case_1', 'teacher_05', 'ArtificialIntelligence', '10')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_05_case_2', 'teacher_05', 'Beautiful', '5')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_05_case_3', 'teacher_05', 'Rhythm', '0')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_05_case_4', 'teacher_05', 'AEIOUaeiou', '10')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_05_case_5', 'teacher_05', 'PythonLanguage', '5')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_06_sample', 'teacher_06', '29', 'PRIME')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_06_case_1', 'teacher_06', '13', 'PRIME')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_06_case_2', 'teacher_06', '1', 'NOT PRIME')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_06_case_3', 'teacher_06', '97', 'PRIME')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_06_case_4', 'teacher_06', '100', 'NOT PRIME')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_06_case_5', 'teacher_06', '997', 'PRIME')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_07_sample', 'teacher_07', '16', '1 2 Fizz 4 Buzz Fizz Bang 8 Fizz Buzz 11 Fizz 13 Bang FizzBuzz 16')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_07_case_1', 'teacher_07', '20', '1 2 Fizz 4 Buzz Fizz Bang 8 Fizz Buzz 11 Fizz 13 Bang FizzBuzz 16 17 Fizz 19 Buzz')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_07_case_2', 'teacher_07', '1', '1')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_07_case_3', 'teacher_07', '21', '1 2 Fizz 4 Buzz Fizz Bang 8 Fizz Buzz 11 Fizz 13 Bang FizzBuzz 16 17 Fizz 19 Buzz Fizz')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_07_case_4', 'teacher_07', '35', '1 2 Fizz 4 Buzz Fizz Bang 8 Fizz Buzz 11 Fizz 13 Bang FizzBuzz 16 17 Fizz 19 Buzz Fizz 22 23 Fizz Buzz 26 Fizz Bang 29 FizzBuzz 31 32 Fizz 34 Buzz')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_07_case_5', 'teacher_07', '10', '1 2 Fizz 4 Buzz Fizz Bang 8 Fizz Buzz')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_08_sample', 'teacher_08', '5 2', '10')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_08_case_1', 'teacher_08', '7 3', '35')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_08_case_2', 'teacher_08', '10 5', '252')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_08_case_3', 'teacher_08', '6 0', '1')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_08_case_4', 'teacher_08', '6 6', '1')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_08_case_5', 'teacher_08', '20 10', '184756')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_09_sample', 'teacher_09', '6
4 2 4 6 2 4', '6 2 4')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_09_case_1', 'teacher_09', '7
3 5 3 8 5 3 9', '9 3 3')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_09_case_2', 'teacher_09', '5
1 1 2 2 3', '3 1 1')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_09_case_3', 'teacher_09', '4
7 7 7 7', '7 7 7')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_09_case_4', 'teacher_09', '3
-5 0 5', '5 -5 -5')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_09_case_5', 'teacher_09', '8
3 1 4 1 5 9 2 6', '9 1 1')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_10_sample', 'teacher_10', 'Was it a car or a cat I saw', 'YES')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_10_case_1', 'teacher_10', 'No lemon no melon', 'YES')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_10_case_2', 'teacher_10', 'Hello World', 'NO')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_10_case_3', 'teacher_10', 'Never Odd Or Even', 'YES')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_10_case_4', 'teacher_10', 'Python Programming', 'NO')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_10_case_5', 'teacher_10', 'A man a plan a canal Panama', 'YES')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_11_sample', 'teacher_11', '2
apple 10
banana 5
2
apple 3
banana 8', 'OK
FAILED
apple 7
banana 5')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_11_case_1', 'teacher_11', '3
mango 8
orange 4
lime 6
3
mango 2
orange 5
lime 6', 'OK
FAILED
OK
lime 0
mango 6
orange 4')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_11_case_2', 'teacher_11', '1
rice 20
3
rice 5
rice 10
rice 10', 'OK
OK
FAILED
rice 5')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_11_case_3', 'teacher_11', '3
pen 5
book 3
eraser 8
2
pen 5
book 4', 'OK
FAILED
book 3
eraser 8
pen 0')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_11_case_4', 'teacher_11', '1
milk 0
1
milk 1', 'FAILED
milk 0')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_11_case_5', 'teacher_11', '4
a 1
b 2
c 3
d 4
4
a 1
b 2
c 3
d 5', 'OK
OK
OK
FAILED
a 0
b 0
c 0
d 4')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_12_sample', 'teacher_12', '6', '20')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_12_case_1', 'teacher_12', '8', '54')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_12_case_2', 'teacher_12', '1', '1')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_12_case_3', 'teacher_12', '2', '2')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_12_case_4', 'teacher_12', '10', '143')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_12_case_5', 'teacher_12', '50', '32951280098')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_13_sample', 'teacher_13', '3
1 2 3
4 5 6
7 8 9', '15 15')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_13_case_1', 'teacher_13', '3
2 0 1
0 3 0
1 0 4', '9 5')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_13_case_2', 'teacher_13', '2
1 2
3 4', '5 5')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_13_case_3', 'teacher_13', '1
5', '5 5')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_13_case_4', 'teacher_13', '4
1 0 0 1
0 1 1 0
0 1 1 0
1 0 0 1', '4 4')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_13_case_5', 'teacher_13', '3
2 4 6
1 3 5
7 9 8', '13 16')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_14_sample', 'teacher_14', '25', '11001 31')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_14_case_1', 'teacher_14', '77', '1001101 115')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_14_case_2', 'teacher_14', '0', '0 0')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_14_case_3', 'teacher_14', '255', '11111111 377')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_14_case_4', 'teacher_14', '1024', '10000000000 2000')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_14_case_5', 'teacher_14', '100000', '11000011010100000 303240')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_15_sample', 'teacher_15', '5
7 1 4 9 2', '4.00')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_15_case_1', 'teacher_15', '6
3 1 4 1 5 9', '3.50')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_15_case_2', 'teacher_15', '4
1 2 3 4', '2.50')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_15_case_3', 'teacher_15', '1
10', '10.00')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_15_case_4', 'teacher_15', '6
5 3 8 1 9 2', '4.00')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_15_case_5', 'teacher_15', '3
100 1 50', '50.00')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_16_sample', 'teacher_16', 'hello
3', 'khoor')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_16_case_1', 'teacher_16', 'attack
5', 'fyyfhp')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_16_case_2', 'teacher_16', 'xyz
2', 'zab')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_16_case_3', 'teacher_16', 'abcdefg
26', 'abcdefg')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_16_case_4', 'teacher_16', 'python
13', 'clguba')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_16_case_5', 'teacher_16', 'zzzz
1', 'aaaa')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_17_sample', 'teacher_17', 'Listen
Silent', 'YES')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_17_case_1', 'teacher_17', 'Elvis
Lives', 'YES')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_17_case_2', 'teacher_17', 'Hello
World', 'NO')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_17_case_3', 'teacher_17', 'Dormitory
Dirty Room', 'YES')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_17_case_4', 'teacher_17', 'The Morse Code
Here Come Dots', 'YES')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_17_case_5', 'teacher_17', 'abc
abd', 'NO')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_18_sample', 'teacher_18', '30', '2 3 5 7 11 13 17 19 23 29')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_18_case_1', 'teacher_18', '40', '2 3 5 7 11 13 17 19 23 29 31 37')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_18_case_2', 'teacher_18', '2', '2')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_18_case_3', 'teacher_18', '50', '2 3 5 7 11 13 17 19 23 29 31 37 41 43 47')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_18_case_4', 'teacher_18', '100', '2 3 5 7 11 13 17 19 23 29 31 37 41 43 47 53 59 61 67 71 73 79 83 89 97')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_18_case_5', 'teacher_18', '17', '2 3 5 7 11 13 17')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_19_sample', 'teacher_19', '{[()()]}', 'VALID')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_19_case_1', 'teacher_19', '([{}])', 'VALID')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_19_case_2', 'teacher_19', '([)]', 'INVALID')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_19_case_3', 'teacher_19', '((()))', 'VALID')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_19_case_4', 'teacher_19', '{[}]', 'INVALID')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_19_case_5', 'teacher_19', '()[]{}', 'VALID')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_20_sample', 'teacher_20', '4
5 2 8 3', '0 5 7 15')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_20_case_1', 'teacher_20', '3
4 4 4', '0 4 8')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_20_case_2', 'teacher_20', '1
10', '0')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_20_case_3', 'teacher_20', '3
1 1 1', '0 1 2')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_20_case_4', 'teacher_20', '5
2 4 6 8 10', '0 2 6 12 20')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_20_case_5', 'teacher_20', '2
0 5', '0 0')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_21_sample', 'teacher_21', '5
APPEND 1
APPEND 2
PREPEND 0
APPEND 3
DELETE 2', '0 1 3')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_21_case_1', 'teacher_21', '6
APPEND 10
APPEND 20
PREPEND 5
APPEND 30
DELETE 20
PREPEND 1', '1 5 10 30')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_21_case_2', 'teacher_21', '3
APPEND 5
DELETE 5
DELETE 5', 'EMPTY')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_21_case_3', 'teacher_21', '4
PREPEND 1
PREPEND 2
PREPEND 3
APPEND 4', '3 2 1 4')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_21_case_4', 'teacher_21', '2
APPEND 9
DELETE 100', '9')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_21_case_5', 'teacher_21', '6
APPEND 1
APPEND 1
APPEND 1
DELETE 1
DELETE 1
DELETE 1', 'EMPTY')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_22_sample', 'teacher_22', '3
4 6 8', '2 24')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_22_case_1', 'teacher_22', '4
10 20 30 40', '10 120')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_22_case_2', 'teacher_22', '2
7 5', '1 35')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_22_case_3', 'teacher_22', '4
2 4 8 16', '2 16')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_22_case_4', 'teacher_22', '3
12 18 24', '6 72')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_22_case_5', 'teacher_22', '5
3 6 9 12 15', '3 180')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_23_sample', 'teacher_23', '2 2 2
1 2
3 4
5 6
7 8', '19 22
43 50')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_23_case_1', 'teacher_23', '2 2 2
2 0
1 3
4 1
0 2', '8 2
4 7')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_23_case_2', 'teacher_23', '1 3 1
1 2 3
4
5
6', '32')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_23_case_3', 'teacher_23', '2 3 2
1 0 2
-1 3 1
3 1
2 1
1 0', '5 1
4 2')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_23_case_4', 'teacher_23', '3 1 3
2
1
3
1 4 5', '2 8 10
1 4 5
3 12 15')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_23_case_5', 'teacher_23', '2 2 3
0 1
1 0
2 3 4
5 6 7', '5 6 7
2 3 4')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_24_sample', 'teacher_24', 'abcde
ace', '3')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_24_case_1', 'teacher_24', 'abcdef
fbd', '2')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_24_case_2', 'teacher_24', 'abc
abc', '3')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_24_case_3', 'teacher_24', 'abc
def', '0')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_24_case_4', 'teacher_24', 'aggtab
gxtxayb', '4')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_24_case_5', 'teacher_24', 'programming
gaming', '6')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_25_sample', 'teacher_25', '6
eat tea tan ate nat bat', 'eat tea ate
tan nat
bat')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_25_case_1', 'teacher_25', '7
cinema iceman act cat rat art tar', 'cinema iceman
act cat
rat art tar')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_25_case_2', 'teacher_25', '4
abc bca cab bad', 'abc bca cab
bad')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_25_case_3', 'teacher_25', '3
xyz zyx abc', 'xyz zyx
abc')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_25_case_4', 'teacher_25', '5
listen silent enlist google gogole', 'listen silent enlist
google gogole')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_25_case_5', 'teacher_25', '1
solo', 'solo')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_26_sample', 'teacher_26', '3
Somchai 80 70 90
Somsri 90 90 90
Anan 80 70 90', 'Somsri 90.00
Anan 80.00
Somchai 80.00')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_26_case_1', 'teacher_26', '3
Nina 88 92 79
Om 75 60 65
Pim 88 92 79', 'Nina 86.33
Pim 86.33
Om 66.67')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_26_case_2', 'teacher_26', '2
A 100 100 100
B 0 0 0', 'A 100.00
B 0.00')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_26_case_3', 'teacher_26', '4
John 60 60 60
Jane 70 80 90
Jim 70 80 90
Jill 50 50 50', 'Jane 80.00
Jim 80.00
John 60.00
Jill 50.00')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_26_case_4', 'teacher_26', '1
Solo 33 33 34', 'Solo 33.33')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_26_case_5', 'teacher_26', '5
A 90 80 70
B 70 80 90
C 80 80 80
D 60 70 80
E 100 90 80', 'E 90.00
A 80.00
B 80.00
C 80.00
D 70.00')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_27_sample', 'teacher_27', '5
2 4 4 4 5', '3.80 0.96 0.98')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_27_case_1', 'teacher_27', '4
10 12 14 16', '13.00 5.00 2.24')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_27_case_2', 'teacher_27', '4
1 2 3 4', '2.50 1.25 1.12')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_27_case_3', 'teacher_27', '3
10 10 10', '10.00 0.00 0.00')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_27_case_4', 'teacher_27', '6
1 2 3 4 5 6', '3.50 2.92 1.71')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_27_case_5', 'teacher_27', '2
0 100', '50.00 2500.00 50.00')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_28_sample', 'teacher_28', '2024 2 28', 'LEAP
2024 2 29')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_28_case_1', 'teacher_28', '2021 4 30', 'NOT LEAP
2021 5 1')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_28_case_2', 'teacher_28', '2023 2 28', 'NOT LEAP
2023 3 1')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_28_case_3', 'teacher_28', '2000 12 31', 'LEAP
2001 1 1')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_28_case_4', 'teacher_28', '1900 2 28', 'NOT LEAP
1900 3 1')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_28_case_5', 'teacher_28', '2024 1 31', 'LEAP
2024 2 1')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_29_sample', 'teacher_29', '7
1 3 5 7 9 11 13
9', '4')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_29_case_1', 'teacher_29', '8
2 4 6 8 10 12 14 16
15', '-1')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_29_case_2', 'teacher_29', '5
2 4 6 8 10
5', '-1')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_29_case_3', 'teacher_29', '1
5
5', '0')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_29_case_4', 'teacher_29', '6
-10 -5 0 5 10 15
-10', '0')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_29_case_5', 'teacher_29', '10
1 2 3 4 5 6 7 8 9 10
10', '9')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into public_test_cases (id, problem_id, input, expected_output) values ('teacher_30_sample', 'teacher_30', '2', 'Move disk 1 from A to B
Move disk 2 from A to C
Move disk 1 from B to C
Total moves: 3')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_30_case_1', 'teacher_30', '4', 'Move disk 1 from A to B
Move disk 2 from A to C
Move disk 1 from B to C
Move disk 3 from A to B
Move disk 1 from C to A
Move disk 2 from C to B
Move disk 1 from A to B
Move disk 4 from A to C
Move disk 1 from B to C
Move disk 2 from B to A
Move disk 1 from C to A
Move disk 3 from B to C
Move disk 1 from A to B
Move disk 2 from A to C
Move disk 1 from B to C
Total moves: 15')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_30_case_2', 'teacher_30', '1', 'Move disk 1 from A to C
Total moves: 1')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_30_case_3', 'teacher_30', '3', 'Move disk 1 from A to C
Move disk 2 from A to B
Move disk 1 from C to B
Move disk 3 from A to C
Move disk 1 from B to A
Move disk 2 from B to C
Move disk 1 from A to C
Total moves: 7')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_30_case_4', 'teacher_30', '6', 'Move disk 1 from A to B
Move disk 2 from A to C
Move disk 1 from B to C
Move disk 3 from A to B
Move disk 1 from C to A
Move disk 2 from C to B
Move disk 1 from A to B
Move disk 4 from A to C
Move disk 1 from B to C
Move disk 2 from B to A
Move disk 1 from C to A
Move disk 3 from B to C
Move disk 1 from A to B
Move disk 2 from A to C
Move disk 1 from B to C
Move disk 5 from A to B
Move disk 1 from C to A
Move disk 2 from C to B
Move disk 1 from A to B
Move disk 3 from C to A
Move disk 1 from B to C
Move disk 2 from B to A
Move disk 1 from C to A
Move disk 4 from C to B
Move disk 1 from A to B
Move disk 2 from A to C
Move disk 1 from B to C
Move disk 3 from A to B
Move disk 1 from C to A
Move disk 2 from C to B
Move disk 1 from A to B
Move disk 6 from A to C
Move disk 1 from B to C
Move disk 2 from B to A
Move disk 1 from C to A
Move disk 3 from B to C
Move disk 1 from A to B
Move disk 2 from A to C
Move disk 1 from B to C
Move disk 4 from B to A
Move disk 1 from C to A
Move disk 2 from C to B
Move disk 1 from A to B
Move disk 3 from C to A
Move disk 1 from B to C
Move disk 2 from B to A
Move disk 1 from C to A
Move disk 5 from B to C
Move disk 1 from A to B
Move disk 2 from A to C
Move disk 1 from B to C
Move disk 3 from A to B
Move disk 1 from C to A
Move disk 2 from C to B
Move disk 1 from A to B
Move disk 4 from A to C
Move disk 1 from B to C
Move disk 2 from B to A
Move disk 1 from C to A
Move disk 3 from B to C
Move disk 1 from A to B
Move disk 2 from A to C
Move disk 1 from B to C
Total moves: 63')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

insert into private_test_cases (id, problem_id, input, expected_output) values ('teacher_30_case_5', 'teacher_30', '5', 'Move disk 1 from A to C
Move disk 2 from A to B
Move disk 1 from C to B
Move disk 3 from A to C
Move disk 1 from B to A
Move disk 2 from B to C
Move disk 1 from A to C
Move disk 4 from A to B
Move disk 1 from C to B
Move disk 2 from C to A
Move disk 1 from B to A
Move disk 3 from C to B
Move disk 1 from A to C
Move disk 2 from A to B
Move disk 1 from C to B
Move disk 5 from A to C
Move disk 1 from B to A
Move disk 2 from B to C
Move disk 1 from A to C
Move disk 3 from B to A
Move disk 1 from C to B
Move disk 2 from C to A
Move disk 1 from B to A
Move disk 4 from B to C
Move disk 1 from A to C
Move disk 2 from A to B
Move disk 1 from C to B
Move disk 3 from A to C
Move disk 1 from B to A
Move disk 2 from B to C
Move disk 1 from A to C
Total moves: 31')
on conflict (id) do update set
  problem_id = excluded.problem_id, input = excluded.input, expected_output = excluded.expected_output;

commit;
