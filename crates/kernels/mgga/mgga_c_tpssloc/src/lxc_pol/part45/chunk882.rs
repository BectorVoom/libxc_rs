//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 882/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk882<F: Float>(t2752: F, t31429: F, t1914: F, t2553: F, t193: F, t201: F, t8565: F, t2749: F, t10143: F, t31441: F, t868: F, t113114: F, t1877: F, t2249: F, t22951: F, t22960: F, t22961: F, t22964: F, t23296: F, t24191: F, t24339: F, t2522: F, t25373: F, t26756: F, t30767: F, t31442: F, t31448: F, t4314: F, t6671: F, t7114: F, t81547: F, t84791: F, t84797: F, t8566: F, t8569: F, t86716: F, t86770: F) -> (F, F, F, F, F, F, F) {
    let t114992 = t31429 * t2752;
    let t115000 = t1914 * t2553;
    let t115009 = t193 * t201 * t8565;
    let t115012 = t1914 * t2749;
    let t115027 = t8565 * t10143;
    let t115030 = t31441 * t868;
    let t115040 = -t1877 * t114992 * t6671 - t1877 * t7114 * t113114 / 2.0 - 3.0 * t84797 * t31442 - 3.0 / 2.0 * t24191 * t22960 * t115000 - t1877 * t7114 * t2249 * t1914 / 2.0 - 3.0 * t115009 * t22961 - 3.0 * t26756 * t86716 * t115012 - t1877 * t84791 * t8569 / 2.0 + 2.0 * t26756 * t86770 * t31448 - t1877 * t24339 * t30767 + 3.0 * t2522 * t8566 * t22964 + t1877 * t115027 * t23296 + 6.0 * t24191 * t25373 * t115030 + 3.0 * t4314 * t8566 * t22951 - 3.0 * t24191 * t81547 * t31441;
    (t114992, t115000, t115009, t115012, t115027, t115030, t115040)
}
