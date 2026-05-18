//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 726/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk726<F: Float>(t2610: F, t6581: F, t2230: F, t6589: F, t213: F, t6593: F, t1894: F, t236: F, t2553: F, t6591: F, t229: F, t6546: F) -> (F, F, F, F, F) {
    let t23059 = t6581 * t2610;
    let t23061 = t2230 * t6589;
    let t23062 = t23061 * t213;
    let t23063 = t23062 * t6593;
    let t23066 = t1894 * t236 * t2553;
    let t23067 = t6591 * t23066;
    let t23069 = t6546 * t229;
    (t23059, t23062, t23063, t23067, t23069)
}
