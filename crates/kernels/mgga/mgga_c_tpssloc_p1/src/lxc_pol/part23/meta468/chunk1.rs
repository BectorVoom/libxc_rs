//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1375/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1375<F: Float>(t59657: F, t60168: F, t60173: F, t60204: F, t68502: F, t68504: F, t68506: F, t76877: F, t76880: F, t76887: F, t76890: F, t77042: F, t77073: F, t77076: F) -> F {
    let t77272 = F::cast_from(0.27785333333333333333e0_f64) * t68502 + F::new(0.166712e1) * t68504 - F::cast_from(0.55570666666666666668e0_f64) * t68506 + F::new(0.94674375e0) * t77042 + F::cast_from(0.13892666666666666667e1_f64) * t60168 - F::cast_from(0.69463333333333333334e0_f64) * t60173 - F::cast_from(0.91817777777777777776e0_f64) * t59657 + F::new(0.125034e1) * t76880 + F::new(0.6311625e0) * t77073 - F::cast_from(0.6618234375e1_f64) * t77076 - F::cast_from(0.23154444444444444445e0_f64) * t60204 - F::new(0.104195e0) * t76877 - F::cast_from(0.10805407407407407407e0_f64) * t76887 - F::new(0.104195e0) * t76890;
    t77272
}
