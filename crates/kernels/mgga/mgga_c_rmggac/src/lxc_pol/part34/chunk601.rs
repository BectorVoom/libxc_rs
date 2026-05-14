//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 601/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk601<F: Float>(t13980: F, t2160: F, t638: F, t13984: F, t3061: F, t7184: F, t1008: F, t211: F, t220: F, t1966: F) -> (F, F, F, F, F, F) {
    let t68514 = t638 * t2160 * t13980;
    let t68517 = t638 * t2160 * t13984;
    let t68520 = t638 * t7184 * t3061;
    let t68522 = t211 * t1008;
    let t68523 = t68522 * t220;
    let t68524 = t1966 * t68523;
    (t68514, t68517, t68520, t68522, t68523, t68524)
}
