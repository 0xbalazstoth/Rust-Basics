fn isLowerCase(l: &char) -> bool {
    let mut charCode: i32 = l.trim().parse().unwrap();
    if(charCode >= 97 && charCode <= 122) {
        return true;
    } else {
        return false;
    }
}

fn isUpperCase(l: &char) -> bool {
    let mut charCode: i32 = l.trim().parse().unwrap();
    if(charCode >= 65 && charCode <= 90) {
        return true;
    } else {
        return false;
    }
}

fn to_alternating_case(s: &str) -> String {
    let mut result: String = "";
    for i in 0..s.len() {
        if(isUpperCase(s[i])) {
            result += s[i].to_lowercase();
        } else if(isLowerCase(s[i])) {
            result += s[i].to_uppercase();
        }
    }
    return result;
}

fn main() {
    println!(to_alternating_case("{}, aAsdDDs"));
}