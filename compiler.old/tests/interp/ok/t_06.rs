// test support for other bit sizes and other operations

fn rand_u64(reg u64) -> reg u64 = python rand_u64;
fn assert_equal_u64(reg u64, reg u64) = python assert_equal;
fn assert_equal_u64s(stack u64, stack u64) = python assert_equal;
fn assert_equal_u256(reg u256, reg u256) = python assert_equal;

fn add_64x4(reg u256, reg u256)-> reg u256 = python add_64x4;

fn add_32x8(reg u256, reg u256)-> reg u256 = python add_32x8;

fn umul_32x4(reg u256, reg u256)-> reg u256 = python umul_32x4;

fn imul_32x4(reg u256, reg u256)-> reg u256 = python imul_32x4;

fn set_32x8(x7, x6, x5, x4, x3, x2, x1, x0 : reg u32)-> reg u256 = python set_32x8; 

fn set_64x4(a : stack u64[4])-> reg u256 = python set_64x4; 

fn get_64x4(x : reg u256)-> stack u64[4] = python extract_64x4; 

fn shuffle_32x8(x : reg u256, inline u8)-> reg u256 = python shuffle_32x8; 

fn permute_64x4(x : reg u256, inline u8)-> reg u256 = python permute_64x4; 

fn shift_left_64x4(x : reg u256, inline u8) -> reg u256 = python shift_left_64x4;

fn shift_right_64x4(x : reg u256, inline u8) -> reg u256 = python shift_right_64x4;

fn blend_8x32(x, y, z : reg u256) -> reg u256 = python blend_8x32;

fn test() {
  x8   : reg u8;
  x16  : reg u16;
  x32  : reg u32;
  x64  : reg u64;
  x128 : reg u128;
  x256 : reg u256;
  y256 : reg u256;
  z256 : reg u256;
  c    : reg u256;
  s    : reg u64;

  w  : reg u32[8];
  ww : stack u64[4];
  vv : stack u64[4];
  
  i : inline u64;

  s = 0;

  x8   = 42:u8;
  x16  = 42:u16;
  x32  = 42:u32;
  x64  = rand_u64(s); s += 1;
  x128 = 42:u128;
  x256 = 42:u256;
  
  // test: non-overflow on addition
  c    = 1:u256;
  x256 = 0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_fffffffe:u256;
  y256 = add_64x4(x256, c);
  c = 0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff:u256;
  assert_equal_u256(y256,c);

  // test: overflow on 64-bit lane
  c    = 1:u256;
  y256 = add_64x4(y256, c);
  c = 0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_00000000_00000000:u256;
  assert_equal_u256(y256,c);

  // test: overflow on 32-bit lane
  c    = 2:u256;
  y256 = add_32x8(x256, c);
  c = 0xffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_ffffffff_00000000:u256;
  assert_equal_u256(y256,c);

  // test: umul ignores odd 32-bit lanes (* 1)
  x256 = 0x00000001_ffffffff_aabbccdd_ffffffff_00000000_ffffffff_ffffffff_ffffffff:u256;
  y256 = 0x00000001_00000001_aabbccdd_00000001_00000000_00000001_ffffffff_00000001:u256;
  y256 = umul_32x4(x256, y256);

  c = 0x00000000_ffffffff_00000000_ffffffff_00000000_ffffffff_00000000_ffffffff:u256;
  assert_equal_u256(y256,c);

  // test: umul ignores odd 32-bit lanes (* ff)
  x256 = 0x00000001_ffffffff_aabbccdd_ffffffff_00000000_ffffffff_ffffffff_ffffffff:u256;
  y256 = 0x00000001_00000100_aabbccdd_00000100_00000000_00000100_ffffffff_00000100:u256;
  y256 = umul_32x4(x256, y256);

  c = 0x000000ff_ffffff00_000000ff_ffffff00_000000ff_ffffff00_000000ff_ffffff00:u256;
  assert_equal_u256(y256,c);

  // test: imul ignores odd 32-bit lanes (* 1)
  x256 = 0x00000001_ffffffff_aabbccdd_ffffffff_00000000_ffffffff_ffffffff_ffffffff:u256;
  y256 = 0x00000001_00000001_aabbccdd_00000001_00000000_00000001_ffffffff_00000001:u256;
  y256 = imul_32x4(x256, y256);

  c = 0x00000000_ffffffff_00000000_ffffffff_00000000_ffffffff_00000000_ffffffff:u256;
  //assert_equal_u256(y256,c);

  // test: imul multiply with -1
  x256 = 0x00000001_fffffffe_aabbccdd_fffffffe_00000000_fffffffe_ffffffff_fffffffe:u256;
  y256 = 0x00000001_ffffffff_aabbccdd_ffffffff_00000000_ffffffff_ffffffff_ffffffff:u256;
  y256 = imul_32x4(x256, y256);

  c = 0x00000000_00000002_00000000_00000002_00000000_00000002_00000000_00000002:u256;
  assert_equal_u256(y256,c);

  // test: set_32x8
  x256 = 0x00000008_00000007_00000006_00000005_00000004_00000003_00000002_00000001:u256;
  w[0] = 1:u32;
  w[1] = 2:u32;
  w[2] = 3:u32;
  w[3] = 4:u32;
  w[4] = 5:u32;
  w[5] = 6:u32;
  w[6] = 7:u32;
  w[7] = 8:u32;

  y256 = set_32x8(w[7],w[6],w[5],w[4],w[3],w[2],w[1],w[0]);
  assert_equal_u256(x256,y256);

  // test: set_64x4
  x256 = 0x00000008_00000007_00000006_00000005_00000004_00000003_00000002_00000001:u256;
  ww[0] = 0x00000002_00000001:u64;
  ww[1] = 0x00000004_00000003:u64;
  ww[2] = 0x00000006_00000005:u64;
  ww[3] = 0x00000008_00000007:u64;

  y256 = set_64x4(ww);
  assert_equal_u256(x256,y256);

  vv = get_64x4(x256);
  for i in 0..4 {
    assert_equal_u64s(ww[i],vv[i]);
  }

  // test shuffle: identity
  y256 = shuffle_32x8(x256,0xe4:u8); // 0xe4 = 0b11_10_01_00 => identity
  assert_equal_u256(x256,y256);
  
  // test shuffle: increasing to decreasing
  y256 = shuffle_32x8(x256,0x1b:u8); // 0xe4 = 0b00_01_10_11
  x256 = 0x00000005_00000006_00000007_00000008_00000001_00000002_00000003_00000004:u256;
  assert_equal_u256(x256,y256);

  // test permute: identity
  x256 = 0x00000008_00000007_00000006_00000005_00000004_00000003_00000002_00000001:u256;
  y256 = permute_64x4(x256,0xe4:u8); // 0xe4 = 0b11_10_01_00 => identity
  assert_equal_u256(x256,y256);
  
  // test permute: increasing to decreasing
  x256 = 0x00000008_00000007_00000006_00000005_00000004_00000003_00000002_00000001:u256;
  y256 = permute_64x4(x256,0x1b:u8); // 0xe4 = 0b00_01_10_11
  x256 = 0x00000002_00000001_00000004_00000003_00000006_00000005_00000008_00000007:u256;
  assert_equal_u256(x256,y256);

  // test shift-left
  x256 = 0x00100004_00100003_00100002_00100001_00000008_00000004_00000002_00000001:u256;
  y256 = shift_left_64x4(x256,2:u8);
  c    = 0x00400010_0040000c_00400008_00400004_00000020_00000010_00000008_00000004:u256;
  assert_equal_u256(y256,c);

  // test shift-left
  x256 = 0x00100004_00100003_00100002_00100001_00000008_00000004_00000002_00000001:u256;
  y256 = shift_left_64x4(x256,32:u8);
  c    = 0x00100003_00000000_00100001_00000000_00000004_00000000_00000001_00000000:u256;
  assert_equal_u256(y256,c);
 
  // test shift-left
  x256 = 0x80000000_00000000_80000000_00000000_80000000_00000000_80000000_00000000:u256;
  y256 = shift_left_64x4(x256,1:u8);
  c    = 0:u256;
  assert_equal_u256(y256,c);
 
  // test shift-right
  x256 = 0x80000000_00000000_80000000_00000000_80000000_00000000_80000000_00000000:u256;
  y256 = shift_right_64x4(x256,1:u8);
  c    = 0x40000000_00000000_40000000_00000000_40000000_00000000_40000000_00000000:u256;
  assert_equal_u256(y256,c);

  // test shift-right
  x256 = 0x00000000_00000001_00000000_00000001_00000000_00000001_00000000_00000001:u256;
  y256 = shift_right_64x4(x256,1:u8);
  c    = 0:u256;
  assert_equal_u256(y256,c);

  // test blend
  x256 = 0x01_02_03_04_05_06_07_08__09_10_11_12_13_14_15_16__17_18_19_20_21_22_23_24__25_26_27_28_29_30_31_32:u256;
  y256 = 0x71_72_73_74_75_76_77_78__79_80_81_82_83_84_85_86__87_88_89_60_61_62_63_64__65_66_67_68_69_50_51_52:u256;
  z256 = 0x01_00_01_00_01_00_01_00__01_00_01_00_01_00_01_00__11_11_00_00_11_11_00_00__11_11_00_00_11_11_00_00:u256;

  z256 = blend_8x32(x256,y256,z256);
  x256 =
   0x71_02_73_04_75_06_77_08_79_10_81_12_83_14_85_16_87_88_19_20_61_62_23_24_65_66_27_28_69_50_31_32:u256;
  assert_equal_u256(z256,x256);

}

/*
START:CMD
ARG="typecheck,renumber_fun_unique,interp[][][test][]"
END:CMD
*/