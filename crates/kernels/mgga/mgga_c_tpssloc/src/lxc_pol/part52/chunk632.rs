//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 632/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk632<F: Float>(t25: F, t28: F, t2: F, t514: F, t5134: F, t584: F, t606: F, t1649: F, t3672: F, t517: F, t1081: F, t157: F, t182: F, t172: F, t1787: F, t763: F, t67: F, t758: F, zeta_threshold: F) -> (F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t5137 = t514 * t2;
    let t5141 = piecewise3(t26, 0.0, 4.0 / 9.0 * t5134 * t606 + 8.0 / 3.0 * t5137 * t584);
    let t5142 = t3672 * t1649;
    let t5145 = t517 * t2;
    let t5149 = piecewise3(t29, 0.0, 4.0 / 9.0 * t5142 * t1081 - 8.0 / 3.0 * t5145 * t584);
    let t5151 = (t5141 + t5149) * t157;
    let t5153 = 0.19751673498613801407e-1 * t5151 * t182;
    let t5154 = t1787 * t172;
    let t5155 = t5154 * t763;
    let t5156 = 0.5848223622634646207e0 * t5155;
    let t5157 = t1787 * t67;
    let t5158 = t5157 * t758;
    (t5151, t5153, t5156, t5158)
}
