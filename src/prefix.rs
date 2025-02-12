use std::collections::HashMap;

fn prefix() {
    let mut prefixes: HashMap<&str, f64> = HashMap::new();
    prefixes.insert("E", 18.0);  // 10^18  - exa
    prefixes.insert("P", 15.0);  // 10^15  - peta
    prefixes.insert("T", 12.0);  // 10^12  - tera
    prefixes.insert("G", 9.0);   // 10^9   - giga
    prefixes.insert("M", 6.0);   // 10^6   - mega
    prefixes.insert("k", 3.0);   // 10^3   - kilo
    prefixes.insert("h", 2.0);   // 10^2   - hecto
    prefixes.insert("da",1.0);   // 10^1   - deca
    prefixes.insert("",  0.0);   // ----- 0 -----
    prefixes.insert("d", -1.0);  // 10^-1  - deci
    prefixes.insert("c", -2.0);  // 10^-2  - centi
    prefixes.insert("m", -3.0);  // 10^-3  - mili
    prefixes.insert("u", -6.0);  // 10^-6  - micro
    prefixes.insert("n", -9.0);  // 10^-9  - nano
    prefixes.insert("p", -12.0); // 10^-12 - pico
    prefixes.insert("f", -15.0); // 10^-15 - femto
    prefixes.insert("a", -18.0); // 10^-18 - atto

    let num = 122.5;
    let input = "k";
    let mut result = 0.0;

    //println!("Original number: {}", num);
    for c in prefixes.keys() {
        if *c == input {
            //println!("Prefix value: {}", prefixes[c]);
            let pref = f64::powf(10.0, prefixes[c]);
            result = num * pref;
        }
    }
    //println!("End result: {:.2}", result);
}