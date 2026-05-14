//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1024/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1024<F: Float>(t112840: F, t30719: F, t808: F, t8344: F, t226: F, t235: F, t2690: F, t23139: F, t8339: F, t23171: F, t23228: F, t8335: F, t30623: F, t81651: F, t82074: F, t2717: F, t6662: F) -> (F, F, F, F, F, F, F) {
    let t112841 = 0.6728792682356731809e-4 * t112840;
    let t112846 = t808 * t30719 * t8344;
    let t112850 = t226 * t235 * t2690 * t8344;
    let t112851 = 119.0 / 6912.0 * t112850;
    let t112855 = t23139 * t8339;
    let t112856 = 0.45217486825437237757e-1 * t112855;
    let t112863 = 0.16449340668482264365e-1 * t23171 * t23228 * t8335;
    let t112867 = t81651 * t82074 * t30623;
    let t112873 = t2717 * t6662;
    (t112841, t112846, t112851, t112856, t112863, t112867, t112873)
}
