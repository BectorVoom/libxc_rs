//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1176/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1176<F: Float>(t1880: F, t21049: F, t6553: F, t82252: F, t1484: F, t22986: F, t23270: F, t98161: F, t7488: F, t98133: F, t105250: F, t1492: F, t17052: F, t259: F, t28317: F, t28406: F, t4147: F, t7517: F, t82070: F, t86911: F, t86916: F, t98213: F) -> (F,) {
    let t105254 = t1880 * t6553 * t82252 * t21049;
    let t105258 = t22986 * t23270 * t98161 * t1484;
    let t105267 = t1880 * t98133 * t7488;
    let t105269 = 0.78134368175290755733e-1 * t86911 + 0.49348022005446793095e-1 * t86916 + t82070 + 6.0 * t4147 * t28317 - 0.82246703342411321825e-2 * t105250 - 0.49348022005446793095e-1 * t105254 - 0.9869604401089358619e-1 * t105258 - 0.49348022005446793095e-1 * t98213 + 6.0 * t17052 * t7517 + 3.0 * t1492 * t28406 * t259 - 0.24674011002723396548e-1 * t105267;
    (t105269,)
}
