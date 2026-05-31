//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 780/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk780<F: Float>(t6936: F, t7712: F, t1814: F, t2002: F, t559: F, t1827: F, t6945: F, t1831: F, t6952: F, t6915: F, t6922: F, t6935: F, t6949: F, t7706: F, t7710: F) -> (F, F) {
    let t7713 = t6936 * t7712;
    let t7715 = t1814 * t2002;
    let t7716 = t7715 * t559;
    let t7718 = t6945 * t1827;
    let t7720 = t6952 * t1831;
    let t7722 = -t6915 - t7706 / F::cast_from(48.0_f64) - t6922 - F::cast_from(0.12111826828242117256e-2_f64) * t7710 - t6935 - F::cast_from(0.20186378047070195427e-3_f64) * t7713 + t7716 / F::cast_from(1536.0_f64) - t7718 / F::cast_from(1536.0_f64) - t6949 - t7720 / F::cast_from(384.0_f64);
    (t7715, t7722)
}
