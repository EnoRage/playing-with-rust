// pub fn test1() {
//     let mut a: Vec<i32> = vec![1, 2, 3];
//     let mut b: Vec<i32> = vec![4, 5, 6];
//
//     a.append(&mut b);
//
//     println!("{:?}", &a[..]);
// }
//
// pub fn test2() {
//     let mut aa: Vec<i32> = vec![1, 2, 3];
//     let mut b: Vec<i32> = vec![4, 5, 6];
//
//     aa.append(&mut b);
//
//     // for a in &mut aa {*a *= 2}
//     // for a in aa.iter_mut() { *a *= 2 };
//     // let result = a.iter().map(|&x| x * 2).collect::<Vec<_>>();
//
//     aa.iter_mut().for_each(|el| *el *= 2); // best as for me
//
//     println!("{:?}", &aa[..]);
// }
//
// pub fn test3() {
//     let mut aa: Vec<i32> = vec![1, 2, 3];
//     let mut b: Vec<i32> = vec![4, 5, 6];
//
//     aa.append(&mut b);
//
//     println!("{}", aa.into_iter().sum::<i32>());
//
//     let mut anyCaseStrings: Vec<String> = vec![String::from("0x14djAjk"), String::from("0xllAA")];
//     // anyCaseStrings.iter_mut().for_each(|v|  *v = v.to_uppercase().as_str());
//
//     for a in anyCaseStrings.iter_mut() { *a = a.to_uppercase() };
//
//     println!("{:?}", &&anyCaseStrings[..]);
// }

// pub fn test4() {
//     let mut aa: Vec<i32> = vec![1, 2, 3];
//     let mut b: Vec<i32> = vec![4, 5, 6];
//
//     aa.append(&mut b);
//
//     // let result2 = aa.iter_mut().filter(|x| **x > 2).collect::<Vec<_>>();
//     let result = aa.into_iter().filter(|&x| *x > 2).collect::<Vec<_>>();
//
//     println!("{:?}", &&result[..]);
//     // println!("{:?}", &&result2[..]);
//     // println!("{:?}", &&aa[..]); will be killed
// }

// pub fn test5() {
//     let mut aa: Vec<i32> = vec![1, 2, 3];
//     let mut b: Vec<i32> = vec![5, 4, 6];
//
//     aa.append(&mut b);
//
//     // let result = aa.into_iter().collect::<Vec<_>>();
//     aa.sort();
//
//     println!("{:?}", &&aa[..]);
//     // println!("{:?}", &&result2[..]);
//     // println!("{:?}", &&aa[..]); will be killed
// }

// #[derive(Debug)]
// struct Animal {
//     name: String,
//     age: i32,
// }
//
// #[derive(Debug)]
// struct Cat {
//     animal: Animal,
//     status: String,
// }
//
// pub fn test6() {
//     let animals = vec![
//         Animal { name: String::from("Homyak"), age: 1 },
//         Animal { name: String::from("Starui ded"), age: 23 },
//         Animal { name: String::from("Default"), age: 23 },
//     ];
//
//     let cats = animals
//         .into_iter()
//         .map(|x| {
//             let age = x.age;
//             Cat { animal: x, status: String::from(if age > 1 { "dead" } else { "alive" }) }
//         })
//         .collect::<Vec<_>>();
//
//     println!("{:?}", cats.into_iter().filter(|cat| cat.status == "alive").collect::<Vec<Cat>>());
// }