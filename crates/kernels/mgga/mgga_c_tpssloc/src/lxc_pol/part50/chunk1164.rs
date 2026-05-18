//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1164/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1164<F: Float>(t2752: F, t32885: F, t1877: F, t2219: F, t8370: F, t25365: F, t25373: F, t1408: F, t6665: F, t1530: F, t16596: F, t113111: F, t113135: F, t118376: F, t118377: F, t118381: F, t118387: F, t118393: F, t23290: F, t25015: F, t25028: F, t2522: F, t25372: F, t25377: F, t25381: F, t25385: F, t30753: F, t30757: F, t30770: F, t32899: F, t6670: F, t6671: F, t7475: F, t7545: F) -> (F, F, F, F) {
    let t118399 = t32885 * t2752;
    let t118406 = t1877 * t8370 * t2219;
    let t118407 = t25373 * t25365;
    let t118410 = t1408 * t6665;
    let t118413 = t1530 * t6665;
    let t118414 = t25373 * t118413;
    let t118417 = t25373 * t16596;
    let t118429 = -t1877 * t23290 * t32899 - F::new(3.0) * t118376 * t118377 + F::new(3.0) * t118381 * t25015 + t1877 * t30753 * t1408 / F::new(2.0) - t1877 * t6670 * t118387 - F::new(3.0) / F::new(2.0) * t2522 * t8370 * t25385 - t1877 * t6670 * t118393 - t1877 * t30757 * t25377 / F::new(2.0) - t1877 * t118399 * t6671 / F::new(2.0) + t1877 * t30770 * t25381 - t118406 + F::new(3.0) * t113135 * t118407 - t1877 * t6670 * t118410 + F::new(2.0) * t25372 * t118414 + F::new(3.0) * t113135 * t118417 + F::new(3.0) / F::new(2.0) * t2522 * t30753 * t7475 - t1877 * t113111 * t7545 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t2522 * t8370 * t25028;
    (t118399, t118406, t118413, t118429)
}
