//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1741/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1741<F: Float>(t1354: F, t22765: F, t3858: F, t6945: F, t1339: F, t3851: F, t6936: F, t3856: F, t3788: F, t3793: F, t6604: F, t6919: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22766 = t22765 * t1354;
    let t22767 = F::new(7.0) / F::new(1152.0) * t22766;
    let t22768 = t6945 * t3858;
    let t22770 = t1339 * t3851;
    let t22771 = t6936 * t22770;
    let t22773 = t1339 * t3856;
    let t22774 = t6936 * t22773;
    let t22776 = t3788 * t3793;
    let t22777 = t6936 * t22776;
    let t22779 = t6919 * t6604;
    (t22766, t22767, t22768, t22770, t22771, t22773, t22774, t22776, t22777, t22779)
}
