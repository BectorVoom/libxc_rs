//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1417/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1417<F: Float>(t78064: F, t78076: F, t1107: F, t43880: F, t78028: F, t43777: F, t50846: F, t71470: F, t71472: F, t71474: F, t78026: F, t78029: F, t78033: F, t78037: F, t78041: F, t78045: F, t78049: F) -> (F, F, F, F) {
    let t78077 = t78064 + t78076;
    let t78078 = t1107 * t78077;
    let t78080 = t43880 * t78028;
    let t78082 = -F::cast_from(0.98115555555555555556e0_f64) * t50846 - F::cast_from(0.98115555555555555555e-1_f64) * t71470 + F::new(0.44152e0) * t71472 - F::new(0.132456e1) * t71474 + t43777 - F::new(0.3883875e1) * t78026 + F::cast_from(0.6189328125e-1_f64) * t78029 - F::cast_from(0.80513333333333333332e0_f64) * t78033 + F::cast_from(0.20128333333333333334e1_f64) * t78037 - F::new(0.72462e1) * t78041 + F::new(0.108693e2) * t78045 + F::new(0.24154e1) * t78049 + F::new(0.16504875e0) * t78078 - F::cast_from(0.485484375e1_f64) * t78080;
    (t78077, t78078, t78080, t78082)
}
