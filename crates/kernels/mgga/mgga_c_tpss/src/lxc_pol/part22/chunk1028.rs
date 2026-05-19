//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1028/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1028<F: Float>(t1415: F, t8684: F, t2488: F, t8678: F, t2487: F, t3781: F, t849: F, t2496: F, t3773: F, t2504: F, t3789: F, t11024: F, t11028: F, t11033: F, t11037: F, t11080: F) -> (F, F, F, F, F, F, F) {
    let t11082 = t8684 * t1415;
    let t11083 = t11082 * t2488;
    let t11085 = t8678 * t1415;
    let t11086 = t11085 * t2488;
    let t11088 = t2487 * t3781;
    let t11089 = t11088 * t849;
    let t11091 = t3773 * t2496;
    let t11093 = t2504 * t3781;
    let t11094 = t11093 * t849;
    let t11096 = t3789 * t2496;
    let t11098 = -F::cast_from(0.19931111111111111111e0_f64) * t11024 - F::new(0.17938e1) * t11028 + F::cast_from(0.11958666666666666667e1_f64) * t11033 + F::cast_from(0.59793333333333333334e0_f64) * t11037 + F::new(0.3071625e0) * t11080 + F::cast_from(0.142419375e1_f64) * t11083 - F::new(0.76790625e-1) * t11086 - F::new(0.1898925e1) * t11089 - F::new(0.9494625e0) * t11091 + F::new(0.3071625e0) * t11094 + F::new(0.15358125e0) * t11096;
    (t11083, t11086, t11089, t11091, t11094, t11096, t11098)
}
