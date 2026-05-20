//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1256/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1256<F: Float>(t11998: F, t517: F, t1376: F, t68: F, t225: F, t3753: F, t3880: F, t522: F, t9212: F, t9214: F, t3824: F, t592: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12000 = F::new(1.0) / t517 / t11998;
    let t12019 = t1376 * t1376;
    let t12020 = F::new(1.0) / t12019;
    let t12021 = t68 * t12020;
    let t12030 = t3753 * t225;
    let t12033 = t3880 * t225;
    let t12044 = F::new(24.0) * t9212 * t522;
    let t12045 = t9214 * t522;
    let t12048 = F::new(12.0) * t592 * t3824;
    (t12000, t12019, t12020, t12021, t12030, t12033, t12044, t12045, t12048)
}
