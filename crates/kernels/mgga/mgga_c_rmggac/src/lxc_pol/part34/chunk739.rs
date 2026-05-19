//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 739/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk739<F: Float>(t69267: F, t69270: F, t69272: F, t69287: F, t69289: F, t69067: F, t69082: F, t69085: F, t69102: F, t1986: F, t2206: F, t2209: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t71270 = F::cast_from(0.56054872888033565654e-2_f64) * t69267;
    let t71271 = F::cast_from(0.11515968254014699841e0_f64) * t69270;
    let t71272 = F::cast_from(0.34974422104785384706e-1_f64) * t69272;
    let t71277 = F::cast_from(0.3064333051212501589e-2_f64) * t69287;
    let t71278 = F::cast_from(0.81431813490554440378e-3_f64) * t69289;
    let t71300 = F::cast_from(0.32526727992809621482e-4_f64) * t69067;
    let t71315 = F::cast_from(0.69390353051327192491e-4_f64) * t69082;
    let t71316 = F::cast_from(0.6505345598561924296e-5_f64) * t69085;
    let t71319 = F::cast_from(0.47433845426916398123e-7_f64) * t69102;
    let t71340 = t1986 * t2206;
    let t71343 = t1986 * t2209;
    (t71270, t71271, t71272, t71277, t71278, t71300, t71315, t71316, t71319, t71340, t71343)
}
