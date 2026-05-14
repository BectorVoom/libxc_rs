//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 954/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk954<F: Float>(t8400: F, t986: F, t6800: F, t6810: F, t6799: F, t1948: F, t6768: F, t345: F, t1022: F, t8391: F, t1060: F, t30843: F, t383: F, t1003: F, t1058: F, t1920: F, t30876: F, t30879: F, t353: F, t6680: F, t6687: F, t6797: F, t8401: F, t8404: F) -> (F, F, F, F, F, F, F) {
    let t30882 = t986 * t8400;
    let t30885 = t6810 * t6800;
    let t30886 = t6799 * t30885;
    let t30889 = t1948 * t6768;
    let t30890 = t345 * t30889;
    let t30894 = t8391 * t1022;
    let t30895 = t30894 * t1060;
    let t30897 = t383 * t30843;
    let t30899 = -0.43864908449286038307e-1 * t6680 * t8401 + t30876 + 0.54831135561607547883e-2 * t6687 * t30879 - 0.16449340668482264365e-1 * t6687 * t30882 + 0.16449340668482264365e-1 * t6797 * t30886 + 0.16449340668482264365e-1 * t1920 * t30890 + t1003 * t8404 + t1058 * t30895 + t353 * t30897;
    (t30882, t30885, t30886, t30889, t30895, t30897, t30899)
}
