use std::collections::HashMap;
use std::io::{self,Write};

fn main() {
    let mut variables: HashMap<String, f64>=HashMap::new();
    println!("Hesap Makinesi(Ana hesap makinem hakim oldugum dilde)");
    println!("Desteklenen işlemler: +, -, *, /, ()");
    println!("Değişken atamak için: x = 5 veya x = y * 2-1 vs expresion gir.");
    println!("Çıkmak için: exit yaz");

    loop {
        print!("> ");

        io::stdout().flush().unwrap(); let mut input =String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().replace(" ","");

        if input == "exit" {
            break;
        }
        if let Some((var, expr)) = pars(&input) {
            match eval(expr,&variables) {
                Ok(value) =>{
                    variables.insert(var.to_string(),value);
                    println!("{} = {}",var,value);
                }
                Err(e) => println!("Hata:{}", e),
            }
            continue;
        }

        match eval(&input,&variables) {
            Ok(result) => println!("Sonuç: {}", result),
            Err(e) => println!("Hata: {}",e),
        }
    }
}
fn pars(input: &str) -> Option<(&str,&str)> {
    let parts: Vec<&str> = input.split('=').collect();
    if parts.len() == 2 {
        let var = parts[0].trim();
        let expr = parts[1].trim();
        return Some((var,expr));
    }
    None
}


fn eval(expr: &str, variables:&HashMap<String, f64>) -> Result<f64, String> {
    let tokens = tokenize(expr);
    let postfix=infix_to_post_struct(&tokens)?;
    evaluate_postfix(&postfix,variables)
}
fn tokenize(expr: &str) -> Vec<String> {
    let mut tokens = vec![];    
    let mut cur = String::new();  
    for c in expr.chars() {
        if "+-*/()".contains(c) {      
            if !cur.is_empty() {   
                tokens.push(cur.clone());   
                cur.clear();   
            }   
            tokens.push(c.to_string());  
        } else {    
            cur.push(c);
        }
    }
    if !cur.is_empty() {
        tokens.push(cur);
    }
    tokens
}
fn infix_to_post_struct(tokens:&[String])-> Result<Vec<String>, String> {
    let mut output = vec![];
    let mut operators: Vec<String> =vec![];
    let precedence = |op: &str| match op {
        "(" => 0,
        "+" | "-" =>1,
        "*" | "/" =>2,
        _ => -1,
    };
    for token in tokens {//sp8
        if token.chars().all(|c| c.is_alphanumeric() || c == '.') {
            output.push(token.clone());
        } else if ["+","-","*","/"].contains(&token.as_str()) {
            while let Some(top) = operators.last() {
                if precedence(top) >= precedence(token) {
                    output.push(operators.pop().unwrap());
                } else {
                    break;
                }
            }
            operators.push(token.clone());
        } else if token == "(" {
            operators.push(token.clone());
        } else if token == ")" {
            while let Some(top) = operators.pop() {
                if top == "(" {
                    break;
                }
                output.push(top);
            }
        }
    }
    while let Some(op)=operators.pop() {
        output.push(op);
    }
    Ok(output)
}

fn evaluate_postfix(tokens: &[String], variables:&HashMap<String, f64>) ->Result<f64, String> {
    let mut stack = vec![];
    for token in tokens {//sp10
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else if let Some(&val) = variables.get(token) {
            stack.push(val);
        } else if ["+","-","*","/"].contains(&token.as_str()) {
            if stack.len() < 2 {
                return Err("Geçersiz ifade!".to_string());
            }
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            let result = match token.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => {
                    if b == 0.0 {
                        return Err("Sıfira bölme hatası!".to_string());
                    }
                    a / b
                }
                _ => return Err("Bilinmeyen operatör!".to_string()),
            };
            stack.push(result);
        } else {
            return Err(format!("Tanımsız değişken veya hata: {}", token));
        }
    }

    if stack.len() == 1 {//sp11
        Ok(stack.pop().unwrap())
    } else {
        Err("Geçersiz ifade!".to_string())
    }
}