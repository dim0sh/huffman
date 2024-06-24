pub fn encrypt(mut input: Vec<u8>, salt: Vec<u8>) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    input = salted(input, salt);
    for i in (0..input.len()).rev() {
        let temp = u16::max_value()-input[i] as u16;
        output.push(temp as u8);
        output.push((temp >> 8) as u8);
    }
    output  
}

pub fn decrypt(input: Vec<u8>, salt: Vec<u8>) -> Vec<u8> {
    let mut output = input.clone();
    let mut temp: Vec<u16> = Vec::new();
    for i in 0..(output.len() / 2) {
        temp.push((output[i*2] as u16) | ((output[i*2+1] as u16) << 8));
    }
    temp.reverse();
    output = temp.iter().map(|x| (u16::max_value()-*x) as u8).collect::<Vec<u8>>();
    desalt(output, salt)
}

fn salted(input: Vec<u8>, salt: Vec<u8>) -> Vec<u8> {
    	
    let mut output: Vec<u8> = Vec::new();
    let input_len = input.len();
    
    let step = (input_len / salt.len()).max(2);
    let mut count = 0;
    for i in 0..(input_len + input_len-1) {
        if i - count >= input_len {
            break;
        }
        if i == 0 {
            output.push(input[0]);
            continue;
        }
        match i % step {
            0 => {
                output.push(salt[i % salt.len()]);
                count += 1;
            },
            _ => {
                output.push(input[i - count]);
            }
        }
    }
    output
}

fn desalt (input: Vec<u8>, salt: Vec<u8>) -> Vec<u8> {
    let input_len = input.len();
    let salt_len = salt.len();
    let step = (((input_len - salt_len ) / salt_len)).max(2);
    let temp = input.iter().enumerate()
        .filter(|(i,_)|i % (step) != 0 || *i == 0)
        .map(|(_,x)|*x).collect::<Vec<u8>>();
    temp
}
