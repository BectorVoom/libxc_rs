//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 946/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk946<F: Float>(t22690: F, t6638: F, t23171: F, t206: F, t268: F, t6559: F) -> (F, F, F) {
    let t23172 = t22690 * t6638;
    let t23173 = t23171 * t23172;
    let t23174 = F::cast_from(0.82246703342411321824e-2_f64) * t23173;
    let t23185 = t6559 * t206 * t268;
    (t23173, t23174, t23185)
}
