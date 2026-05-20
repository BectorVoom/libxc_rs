//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1434/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1434<F: Float>(t1354: F, t16288: F, t12211: F, t5223: F, t3804: F, t820: F, t1351: F, t1824: F, t3792: F, t12345: F, t1831: F, t1362: F, t16060: F) -> (F, F, F, F, F, F, F) {
    let t16290 = F::new(7.0) / F::new(2304.0) * t16288 * t1354;
    let t16294 = F::new(7.0) / F::new(24.0) * t12211 * t5223;
    let t16305 = t3804 * t820;
    let t16306 = t1824 * t1351;
    let t16311 = t1824 * t3792;
    let t16317 = t12345 * t1831;
    let t16321 = t16060 * t1362;
    (t16290, t16294, t16305, t16306, t16311, t16317, t16321)
}
