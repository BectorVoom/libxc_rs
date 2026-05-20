//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1021/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1021<F: Float>(t1372: F, t3752: F, t1376: F, t68: F, t1385: F, t3888: F, t3911: F, t3887: F, t225: F, t3753: F, t3880: F, t1323: F, t3879: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12016 = t3752 * t1372;
    let t12019 = t1376 * t1376;
    let t12020 = F::new(1.0) / t12019;
    let t12021 = t68 * t12020;
    let t12022 = t3888 * t1385;
    let t12023 = t12021 * t12022;
    let t12026 = t1385 * t3911;
    let t12027 = t3887 * t12026;
    let t12030 = t3753 * t225;
    let t12033 = t3880 * t225;
    let t12036 = t1323 * t3879;
    (t12016, t12019, t12020, t12021, t12022, t12023, t12026, t12027, t12030, t12033, t12036)
}
