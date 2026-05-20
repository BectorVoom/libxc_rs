//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 797/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk797<F: Float>(t2141: F, t3540: F, t3: F, t7324: F, t1184: F, t52: F, t460: F, t3548: F, t7310: F, t3469: F, t7320: F, t2134: F, t24650: F, t24655: F, t24659: F, t24664: F, t24670: F, t24675: F, t24677: F, t3552: F, t3557: F, t3562: F, t3587: F, t488: F, t7316: F, t7321: F, t7326: F, t7331: F, t7345: F) -> (F, F) {
    let t24681 = t2141 * t3540 / F::new(6912.0);
    let t24682 = t7324 * t3;
    let t24683 = t52 * t1184;
    let t24684 = t24683 * t460;
    let t24685 = t24682 * t24684;
    let t24690 = t7310 * t3548;
    let t24698 = t3469 * t460;
    let t24699 = t24698 * t7320;
    let t24702 = -F::cast_from(0.20186378047070195428e-3_f64) * t24650 * t7331 + F::cast_from(0.10093189023535097714e-3_f64) * t7326 * t24655 + F::cast_from(0.20186378047070195428e-3_f64) * t24659 * t24664 - F::cast_from(0.10093189023535097714e-3_f64) * t24659 * t24670 + t7310 * t3562 / F::new(216.0) + t24675 / F::new(1152.0) + t24677 * t488 / F::new(1536.0) - t24681 - F::cast_from(0.20186378047070195428e-3_f64) * t24685 * t7331 + F::new(5.0) / F::new(6912.0) * t7345 * t3587 - t24690 / F::new(432.0) - t7310 * t3552 / F::new(288.0) - t7310 * t3557 / F::new(144.0) + F::cast_from(0.20186378047070195428e-3_f64) * t7316 * t7321 - F::cast_from(0.10093189023535097714e-3_f64) * t2134 * t24699;
    (t24683, t24702)
}
