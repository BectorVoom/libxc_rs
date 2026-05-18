//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 413/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk413<F: Float>(t27: F, t8532: F, t2084: F, t551: F, t1614: F, t649: F, t1652: F, t674: F, t8450: F) -> (F, F, F, F, F) {
    let t8533 = t27 * t8532;
    let t8536 = t2084 * t551;
    let t8537 = t27 * t8536;
    let t8561 = t649 * t1614;
    let t8562 = t27 * t8561;
    let t8567 = t649 * t1652;
    let t8568 = t27 * t8567;
    let t8571 = t8450 * t674;
    (t8533, t8537, t8562, t8568, t8571)
}
