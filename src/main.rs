
static  BASE64_TABLE : &[char] = &[
    'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P',
    'Q','R','S','T','U','V','W','X','Y','Z','a','b','c','d','e','f',
    'g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v',
    'w','x','y','z','0','1','2','3','4','5','6','7','8','9','+','/'                	
];

fn ascci_to_bit_str(chr :char) -> String {
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
fn bit_str_to_int(str:String ) -> u8 {
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


fn ceil(a : u64,b : u64 ) -> u64 {
    if (a % b) == 0 {
        return a / b;
    }
    return (a / b) + 1;
}
fn encode_base64(str : String) -> String {
    let mut out_str = String::new();
    
    
    let mut tmp_str = String::new();
    let mut sliced = String::new();

    for chr in str.chars() {
        tmp_str  += &ascci_to_bit_str(chr);
    }
    
    tmp_str = tmp_str.chars().rev().collect();
    while tmp_str.len() != 0 {
        sliced.clear();

        sliced.clear();
        let mut i = 0;
        while i < 6 && tmp_str.len() > 0 {
            sliced.push( tmp_str.pop().unwrap());
            i+=1;
        }
        
        if i < 6 {
            while sliced.len() < 6 { sliced.push('0'); }
        } 

        sliced = sliced.chars().rev().collect();
        out_str.push(BASE64_TABLE[bit_str_to_int(sliced.clone()) as usize]);
    } 

    while (out_str.len() % 4) != 0  {
        out_str.push('=');               
    }

    out_str
}


fn main() {

    let mut input_str: String = String::new();
    std::io::stdin().read_line(&mut input_str).unwrap();
    input_str.pop();

    print!("{}\n",encode_base64(input_str));
}
