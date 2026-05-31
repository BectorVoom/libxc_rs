//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2205/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2205<F: Float>(t4616: F, t6764: F, t23544: F, t4571: F, t23482: F, t25682: F, t25588: F, t344: F, t6740: F, t1046: F, t14093: F, t14174: F, t14230: F, t23419: F, t23483: F, t25679: F, t6747: F, t6765: F, t7583: F, t82883: F, t82885: F, t82893: F, t82897: F, t83114: F) -> F {
    let t88277 = t4616 * t6764;
    let t88281 = t23544 * t4571 / F::cast_from(1728.0_f64);
    let t88286 = t23482 * t25682;
    let t88290 = t6740 * t25588 * t344;
    let t88303 = t88277 * t1046 / F::cast_from(1152.0_f64) + t88281 - F::cast_from(0.16149102437656156342e-2_f64) * t83114 * t7583 - F::cast_from(0.16149102437656156342e-2_f64) * t23483 * t25679 - F::cast_from(0.16149102437656156342e-2_f64) * t88286 * t6747 + F::cast_from(0.20186378047070195428e-3_f64) * t88290 * t6747 + t82883 / F::cast_from(2304.0_f64) + t82885 / F::cast_from(648.0_f64) + F::cast_from(0.20186378047070195428e-3_f64) * t82893 - F::cast_from(0.10093189023535097714e-3_f64) * t82897 - t23419 * t14230 / F::cast_from(576.0_f64) - F::cast_from(5.0_f64) / F::cast_from(1152.0_f64) * t6765 * t14174 + t6765 * t14093 / F::cast_from(2304.0_f64);
    t88303
}
