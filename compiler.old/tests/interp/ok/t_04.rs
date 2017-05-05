fn rand_u64(reg u64) -> reg u64 = python rand_u64;
fn assert_equal_u64(reg u64, reg u64) = python assert_equal_u64;

fn test() {
  x, y, z   : reg u64;
  w, r1, r2 : reg u64;
  s         : reg u64;
  i         : inline u64;

  s = 42;

  x = rand_u64(s); s += 1;
  y = rand_u64(s); s += 1;
  z = 10;

  r1 = x*z;
  w  = y*z;
  r1 += w;

  
  w = x + y;
  r2 = 0;
  
  for i in 0..10 {
    r2 += w;
  }

  assert_equal_u64(r1,r2);
}

/*
START:CMD
ARG="typecheck,renumber_fun_unique,interp[][][test][]"
END:CMD
*/