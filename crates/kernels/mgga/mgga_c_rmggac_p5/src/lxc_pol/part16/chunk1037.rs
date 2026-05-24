//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1037/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1037<F: Float>(t1756: F, t2084: F, t2145: F, t27: F, t1818: F, t236: F, t3351: F, t40168: F, t498: F, t10018: F, t7255: F, t1910: F, t495: F, t7230: F, t7231: F) -> (F, F, F, F) {
    let t47616 = t2145 * t27 * t2084 * t1756;
    let t47621 = t3351 * t40168 * t236 * t1818 * t498;
    let t47623 = t7255 * t10018;
    let t47629 = t7230 * t7231 * t236 * t1910 * t495;
    (t47616, t47621, t47623, t47629)
}
