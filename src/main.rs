use std::ops::Add;

static  BASE64_TABLE : &[char] = &[
    'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P',
    'Q','R','S','T','U','V','W','X','Y','Z','a','b','c','d','e','f',
    'g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v',
    'w','x','y','z','0','1','2','3','4','5','6','7','8','9','+','/'                	
];


fn bit_count(number:  u64) -> u64 {
    let mut n = number;
    let mut counter = 0;
    while n != 0 {
        n >>= 1;
        counter += 1;
    }
    counter
} 
fn char_to_bits_str(chr :char) -> String {
    let mut ascci = chr as u8;
    let mut out: String =  String::new();
    while ascci != 0 {
        out.push((ascci % 2).to_string().chars().nth(0).unwrap());
        ascci >>= 1;
    }

    while out.len() < 8 {
        out.push('0');
    }

    out.chars().rev().collect()
}

fn from_bits_str_to_int(str:String ) -> u8 {
    let mut out  = 0;

    let mut power = 0;
    for chr in str.chars() {
        out += match chr {
            '1' => 1,
             _  => 0
        } << power;
        power += 1;
    }

    out
} 


fn encode_base64(str : String) -> String {
    let mut out_str = String::new();
    
    let mut counter : usize = 0;
    let mut tmp_str = String::new();
    let mut sliced = String::new();


    let mut bin_str: String = String::new();

    let mut _bits : u64 = 0; 
    while counter <= str.len() {
        if tmp_str.len() == 24 {
            tmp_str = tmp_str.chars().rev().collect();
            
            while tmp_str.len() > 0 {
                sliced.push(
                    tmp_str.pop().unwrap()
                );
                if sliced.len() == 6 {
                    sliced = sliced.chars().rev().collect();
                    out_str.push(BASE64_TABLE[from_bits_str_to_int(sliced.clone()) as usize]);
                    sliced.clear();
                
                } 
            }

            if sliced.len() != 0 {
                sliced = sliced.chars().rev().collect();

                while sliced.len() < 6 {
                    sliced.push('0');
                }

                sliced = sliced.chars().rev().collect();
                out_str.push(BASE64_TABLE[from_bits_str_to_int(sliced.clone()) as usize]);
                sliced.clear();
            }

            

        }

        if counter == str.len() {break;}
        tmp_str  += &char_to_bits_str(str.chars().nth(counter).unwrap());
        bin_str  += &char_to_bits_str(str.chars().nth(counter).unwrap());
        
        counter += 1;
    }
    


    while tmp_str.len() > 0 {
        sliced = sliced.chars().rev().collect();
        
        sliced.push(
            tmp_str.pop().unwrap()
        );
        
        if sliced.len() == 6 {
            sliced = sliced.chars().rev().collect();
            out_str.push(BASE64_TABLE[from_bits_str_to_int(sliced.clone()) as usize]);
            sliced.clear();
        } 
    }
    if sliced.len() != 0 {
        sliced = sliced.chars().rev().collect();

        while sliced.len() < 6 {
            sliced.push('0');
        }
        sliced = sliced.chars().rev().collect();


        out_str.push(BASE64_TABLE[from_bits_str_to_int(sliced.clone()) as usize]);
        sliced.clear();
    }    



    while (out_str.len() % 4) != 0  {
        out_str.push('=');               
    }

    // bin_str = bin_str.chars().rev().collect();

    out_str
}


fn main() {

    let mut input_str: String = String::new();
    std::io::stdin().read_line(&mut input_str).unwrap();

    input_str.pop();

    print!("{}\n",encode_base64(input_str));
}
