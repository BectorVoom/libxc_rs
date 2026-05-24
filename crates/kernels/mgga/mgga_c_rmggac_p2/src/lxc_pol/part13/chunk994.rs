//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 994/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk994<F: Float>(t1978: F, t7228: F, t8511: F, t236: F, t495: F, t1981: F, t676: F, t498: F, t3134: F, t8512: F, t1982: F, t7428: F) -> (F, F, F) {
    let t41799 = t8511 * t7228 * t1978;
    let t41800 = t236 * t495;
    let t41803 = t41799 * t1981 * t676 * t41800;
    let t41805 = t236 * t498;
    let t41808 = t8512 * t1981 * t3134 * t41805;
    let t41811 = t8511 * t7428 * t1982;
    (t41803, t41808, t41811)
}
