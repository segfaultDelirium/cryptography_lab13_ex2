#![allow(non_snake_case)]

// ocena koncowa // dodac punkty podzielic przez liczbe punktow / 20 * 100 procent i to jest ocena

/*

jesli n jest liczba zlozona czyli ma dzielniki
to n ma dzielnik pierwszy p <= podloga(sqrt(n))

n = n1 * n2 gdzie 1 < n1 < n i 1 < n2 < n
wystarczy znalezc jedna z tych liczb

w algorytmie pollarda
d to nietrywialny dzielnik n

B jest ustalane heurystycznie

B powinno być mniejsze niż sqrt(n) żeby algorytm działał szybko

przyklad n = 299 = 13 * 23
j   a   a^j mod n   d
2   2   4           1
3   4   64          1
4   64  27          13

przyjmijmy B rowne 4

kolejne a dostaje 27

jezeli wybierzemy za male B to nie otrzymamy wyniku,
jezeli wybierzemy za duze B to będziemy dlugo czekac


algorytm RHO

zalozmy ze p jest najmniejszym dzielnikiem pierwszym n

zalozmy ze istnieja dwie liczby calkowite ktore maja te same reszty mod p ale są innymi liczbami
wtedy p <= NWD(x - x', n) < n
wybieramy losowy podzbior x a potem obliczamy powyzej dla wszystkich roznych wartosci x, x'
trzeba znalezc takie x ze trafiamy w cykl graniczny czyli krecimy sie w kolko

jezeli |X| to okolo 1.17 * sqrt(p) to jest 50% ze bedzie co najmniej jedna
kolizja (czyli wrocenie do punktu w ktorym juz sie bylo)
wtedy obliczamy NWD(x-x', n)

f(x) = x^2 + 1
x = f(x) mod p zachowuje sie jak odwzorowanie losowe

przyjmijmy ze xi przystaje do xj mod p (czyli xi oraz xj maja takie same reszty po redukcji modulo p)
f(xi) === f(xj) mod p (f(x) zatem tez przystaje

xi+1 = f(xi) mod n

zatem iterujemy po i oraz j funkcje f(x) = x^2 + 1

przyklad n = 7171 = 71 & 101 f(x) = x^2 + 1, x1 = 1

ciag xi zaczyna sie:
1, 2, 5, 26, 677

5 = 2 * 2 + 1
26 = 5 * 5 + 1

pierwsza kolizja to x7 mod 71 = x18 mod 71 = 58
cykl sklada sie z ogona 7 i cyklu 11

aby przyspieszyc szukanie kolizji przyjmujemy j = 2*i

zatem w rozkladzie pierwsza kolizja pojawia sie dla i = 7 i j = 18

film rainman, potrafi liczyc wykalaczki

*/

// fn f(x: u128) -> u128 {
//     x * x + 1
// }
//
// fn rho(n: u128, x1: u128) -> (Option<u128>, u128) {
//     let mut x = x1;
//     let mut xprim = modulo_euclid(f(x) as i128, n as i128) as u128;
//     let mut p = NWD((x as i128 - xprim as i128).abs() as u128, n);
//     let mut iteration_counter = 0;
//     while p == 1 {
//         iteration_counter += 1;
//         x = modulo_euclid(f(x) as i128, n as i128) as u128;
//         xprim = modulo_euclid(f(xprim) as i128, n as i128) as u128;
//         xprim = modulo_euclid(f(xprim) as i128, n as i128) as u128;
//         p = NWD((x as i128 - xprim as i128).abs() as u128, n);
//     }
//     if p == n {
//         return (None, iteration_counter);
//     }
//
//     return (Some(p), iteration_counter);
// }

fn pollard(n: u128, B: u128) -> (Option<u128>, u128){
    let mut a: u128 = 2;
    let mut iteration_counter = 2;
    // println!("B = {B}");
    for j in 2..=B{
        iteration_counter = j;
        // println!("Iteration: j = {j}");
        // println!("a = {a}");
        a = modulo_pow(a as f64, j, n) as u128;
        // a = modulo_euclid( (a as f64).powf(j as f64) as i128, n as i128) as u128;
        // println!("a = {a}");
        let d = NWD(a - 1, n);
        // println!("d = {d}");

        if d > 1 && d < n{
            return (Some(d), iteration_counter);
        }
    }

    return (None, iteration_counter);
}

fn modulo_pow(a: f64, j: u128, n: u128) -> f64{
    let mut res = a;
    for _i in 1..j{
        res = modulo_euclid((res * a) as i128, n as i128) as f64;
    }

    res
    // a
}

fn find_dividers(n: u128, B: u128){
    println!("n = {n}");
    println!("B = {B}");
    let (p, iteration_count) = pollard(n, B);
    match p {
        Some(p_value) => {
            println!("found p = {p_value} with iteration count: {iteration_count}");
            // println!("found p = {p_value}");
            let other_p = n / p_value;
            println!("other p = {other_p}");
        }
        None => {
            // println!("failed to find p ");
            println!("failed to find p with iteration count: {iteration_count}");
        }
    }
    println!();
}

fn main() {
    println!("pollard p-1");

    // let res = modulo_pow(2.0, 3, 100);
    // println!("res = {res}");

    let n = 262063;
    // let B =  ((n as f64).sqrt()) as u128;
    // let B = (n as f64).sqrt() as u128;
    let B = 100;
    find_dividers(n, B);

    let n = 9420457;
    // let B =  ((n as f64).sqrt()) as u128;
    find_dividers(n, B);
}



fn NWD(j: u128, k: u128) -> u128 {
    if j == 0 {
        return k
    }
    let r = k % j;
    return NWD(r, j);
}

fn modulo_euclid(j: i128, k: i128) -> i128 {
    let res =  j % k;
    if res < 0 {return res + k} else {return res}
}


