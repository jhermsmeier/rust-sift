#[ crate_id = "sift#0.1.0" ];
#[ crate_type = "lib" ];
#[ desc = "Computes the SIFT string distance between two strings" ];

pub fn distance( s1: &str, s2: &str ) -> f64 {
  
  let s1l = s1.len();
  let s2l = s2.len();
  
  if( s1l == 0 ) { return s2l as f64 }
  if( s2l == 0 ) { return s1l as f64 }
  
  let mut i = 0;
  let mut c = 0;
  
  let max_offset = if( s1l > s2l ) { s1l } else { s2l };
  
  let mut offset1 = 0;
  let mut offset2 = 0;
  
  let mut lcs = 0;
  
  while ( c + offset1 < s1l ) && ( c + offset2 < s2l ) {
    if( s1[ c + offset1 ] == s2[ c + i ] ) {
      lcs += 1;
    } else {
      i = 0;
      offset1 = 0;
      offset2 = 0;
      while i < max_offset {
        if( ( c + i < s1l ) && ( s1[ c + i ] == s2[c] ) ) {
          offset1 = i;
          break;
        } else if( ( c + i < s2l ) && ( s1[c] == s2[ c + i ] ) ) {
          offset2 = i;
          break;
        }
        i = i + 1;
      }
    }
    c = c + 1;
  }
  
  ( s1l + s2l ) as f64 / 2.0 - lcs as f64
  
}
