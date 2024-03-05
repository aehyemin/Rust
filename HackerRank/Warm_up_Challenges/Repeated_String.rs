fn repeatedString(s: &str, n: i64) -> i64 {
   let mut afreq:i64=0;
   let mut remain = s.chars();
   let repeat = n/i64::try_from(s.len()).unwrap() ;

   if s.len() < n.try_into().unwrap(){       
       for i in s.chars(){
           match i {
               'a' => afreq+=1, 
               _ => (),
           }
        }
    afreq = afreq * repeat;
   }   
  
    for i in 0..n % s.len()as i64 {
        match remain.next().unwrap(){
            'a' => afreq +=1,
            _ => (),
        }
    }
   afreq
}