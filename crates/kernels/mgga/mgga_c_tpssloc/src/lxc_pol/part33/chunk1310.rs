//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1310/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1310<F: Float>(t105250: F, t105254: F, t105258: F, t105267: F, t1492: F, t17052: F, t259: F, t28317: F, t28406: F, t4147: F, t7517: F, t82070: F, t86911: F, t86916: F, t98213: F) -> F {
    let t105269 = F::cast_from(0.78134368175290755733e-1_f64) * t86911 + F::cast_from(0.49348022005446793095e-1_f64) * t86916 + t82070 + F::new(6.0) * t4147 * t28317 - F::cast_from(0.82246703342411321825e-2_f64) * t105250 - F::cast_from(0.49348022005446793095e-1_f64) * t105254 - F::cast_from(0.9869604401089358619e-1_f64) * t105258 - F::cast_from(0.49348022005446793095e-1_f64) * t98213 + F::new(6.0) * t17052 * t7517 + F::new(3.0) * t1492 * t28406 * t259 - F::cast_from(0.24674011002723396548e-1_f64) * t105267;
    t105269
}
