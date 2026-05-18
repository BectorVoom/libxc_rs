//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1293/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1293<F: Float>(t1656: F, t18967: F, t20155: F, t219: F, t65551: F, t65561: F, t65570: F, t65592: F, t65600: F, t65616: F, t65628: F, t65639: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t67061 = t18967 * t1656;
    let t67083 = t20155 * t219;
    let t67138 = F::new(7.0) / F::new(576.0) * t65551;
    let t67143 = F::new(7.0) / F::new(144.0) * t65561;
    let t67150 = F::new(7.0) / F::new(36.0) * t65570;
    let t67160 = F::new(7.0) / F::new(288.0) * t65592;
    let t67162 = F::new(7.0) / F::new(12.0) * t65600;
    let t67169 = F::new(35.0) / F::new(144.0) * t65616;
    let t67175 = F::new(7.0) / F::new(576.0) * t65628;
    let t67183 = F::new(7.0) / F::new(144.0) * t65639;
    (t67061, t67083, t67138, t67143, t67150, t67160, t67162, t67169, t67175, t67183)
}
