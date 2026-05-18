//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 401/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk401<F: Float>(t265: F, t352: F, t2079: F, t262: F, t2067: F, t3851: F, t333: F, t664: F) -> (F, F, F, F) {
    let t7840 = t265 * t352;
    let t7842 = t2079 * t262 * t7840;
    let t7844 = t3851 * t2067;
    let t7879 = t664 * t333;
    (t7840, t7842, t7844, t7879)
}
