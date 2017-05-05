// ssa form, use before def

extern fn test(cf : reg bool) {
  x,y,z : reg u64;
  
  if cf {
    y = 0;
  } else {
    y = 1;
  }
  x = y; // this is OK
  x += z;
}

/*
START:CMD
ARG="renumber_fun_unique,typecheck,save[/tmp/ssa1.mil][]"
ARG="$ARG,merge_blocks,register_liveness[test],local_ssa[test]"
END:CMD
*/