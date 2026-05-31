//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1199/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1199<F: Float>(t23384: F, t32993: F, t113149: F, t113207: F, t14545: F, t1539: F, t1599: F, t1955: F, t23327: F, t23329: F, t23394: F, t25420: F, t25429: F, t25452: F, t25731: F, t25755: F, t25757: F, t2770: F, t3026: F, t30782: F, t30800: F, t30900: F, t3169: F, t32965: F, t32969: F, t3961: F, t43603: F, t4542: F, t4660: F, t4664: F, t6687: F, t6704: F, t6705: F, t6771: F, t6776: F, t82502: F, t8376: F, t8380: F, t8396: F, t8397: F, t88112: F, t88162: F, t89598: F) -> F {
    let t119033 = t23384 * t32993;
    let t119065 = -t3169 * t32965 + F::cast_from(24.0_f64) * t25757 * t43603 * t8396 * t4664 + F::cast_from(4.0_f64) * t6771 * t25420 - t3026 * t32965 - F::cast_from(0.73108180748810063844e-2_f64) * t25429 * t88112 * t1955 * t2770 * t3961 + F::cast_from(0.54831135561607547883e-2_f64) * t23327 * t82502 * t32969 - F::cast_from(0.54831135561607547883e-2_f64) * t119033 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t4542 * t8376 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t1599 * t30800 + F::cast_from(0.18277045187202515961e-2_f64) * t113207 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t6704 * t6705 * t25731 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t89598 * t8380 + F::cast_from(0.54831135561607547883e-2_f64) * t23327 * t88162 * t30782 + F::cast_from(0.3289868133696452873e-1_f64) * t6687 * t6704 * t23394 * t25452 - F::cast_from(0.54831135561607547883e-2_f64) * t23327 * t23329 * t113149 * t1539 + F::cast_from(4.0_f64) * t25755 * t6776 + F::cast_from(2.0_f64) * t14545 * t8397 - t4660 * t30900;
    t119065
}
