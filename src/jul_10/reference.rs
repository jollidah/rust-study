// fn borrowing_local_variable() {
//     {
//         let r;
//         {
//             let x = 1;
//             r = &x;
//         }
//         // assert_eq!(*r, 1);
//     }
// }

// static mut STASH: &i32 = &128;
// // fn static_lifetime_func(p: &i32) {
// fn static_lifetime_func(p: &'static i32) {
//     // still not good enough
//     unsafe {
//         // STASH = p;
//     }
// }

// fn get_front(v: &[i32]) -> &i32 {   // As just one parameter is required, rust can infer lifetime
// // fn get_front(v: &'a [i32]) -> &'a i32 {
//     &v[1]
// }

// fn reference_outlive() {
//     let s;
//     {
//         let parabola = [9, 4, 1, 0, 1, 4, 9];
//         s = get_front(&parabola);
//         assert_eq!(*s, 1);
//     }
//     // assert_eq!(*s, 1);
// }

// fn sharing_vs_mutation_err() {
//     let v = vec![4, 8, 19, 27, 34, 10];
//     let r = &v;
//     // let aside = v; // move vector to aside
//     r[0]; // bad: uses `v`, which is now uninitialized
// }

// fn sharing_vs_mutation() {
//     let v = vec![4, 8, 19, 27, 34, 10];
//     {
//         let r = &v;
//         r[0]; // ok: vector is still there
//     }
//     let aside = v;
// }

// fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
//     for elt in slice {
//         vec.push(*elt); // cannot borrow a mutable reference to a read-only value
//     }
// }

// fn extend_vec() {
//     let mut wave = Vec::new();
//     let head = vec![0.0, 1.0];
//     let tail = [0.0, -1.0];
//     extend(&mut wave, &head); // extend wave with another vector
//     extend(&mut wave, &tail); // extend wave with an array
//     extend(&mut wave, &wave);
//     assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);
// }

// fn comlex_sharing() {
//     let mut x = 10;
//     let r1 = &x;
//     let r2 = &x; // ok: multiple shared borrows permitted
//     x += 10; // error: cannot assign to `x` because it is borrowed
//     let m = &mut x; // error: cannot borrow `x` as mutable because it is
//                     // also borrowed as immutable
//     println!("{}, {}, {}", r1, r2, m); // the references are used here,
//                                        // so their lifetimes must last
//                                        // at least this long

//     let mut y = 20;
//     let m1 = &mut y;
//     let m2 = &mut y; // error: cannot borrow as mutable more than once
//     let z = y; // error: cannot use `y` because it was mutably borrowed
//     println!("{}, {}, {}", m1, m2, z); // references are used here
// }

// fn reborrow() {
//     let mut w = (107, 109);
//     let r = &w; // ok: reborrowing shared as shared
//     let r0 = &r.0; // error: can't reborrow shared as mutable
//     let m1 = &mut r.1; // r0 gets used here
//     println!("{}", r0);

//     let mut v = (136, 139);
//     let m = &mut v;
//     let m0 = &mut m.0; // ok: reborrowing mutable from mutable
//     *m0 = 137;
//     let r1 = &m.1; // ok: reborrowing shared from mutable,
//                    // and doesn't overlap with m0
//     v.1; // error: access through other paths still forbidden
//     println!("{}", r1); // r1 gets used here
// }

// //* Test!
// // fn a() {
// //     let mut a = 1;
// //     let b = &mut a;
// //     a = 2;

// //     println!("{:?}", b);
// // }
