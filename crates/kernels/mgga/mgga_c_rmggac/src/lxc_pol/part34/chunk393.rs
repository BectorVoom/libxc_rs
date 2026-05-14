//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 393/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk393<F: Float>(t262: F, t8631: F, t2350: F, t352: F, t2347: F, t321: F, t333: F) -> (F, F, F, F, F, F) {
    let t8632 = t262 * t8631;
    let t8635 = t2350 * t352;
    let t8636 = t262 * t8635;
    let t8641 = t2347 * t321;
    let t8642 = t262 * t8641;
    let t8645 = t2347 * t333;
    (t8632, t8635, t8636, t8641, t8642, t8645)
}
