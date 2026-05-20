//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2096/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2096<F: Float>(t23012: F, t7485: F, t1888: F, t23270: F, t2719: F, t46488: F, t25046: F, t6579: F, t1484: F, t2717: F, t22986: F, t82099: F) -> (F, F, F, F, F) {
    let t86955 = t23012 * t7485;
    let t86961 = t1888 * t23270 * t46488 * t2719;
    let t86967 = t6579 * t25046;
    let t86968 = F::cast_from(0.76763589786250567036e-1_f64) * t86967;
    let t86969 = t2717 * t1484;
    let t86972 = t22986 * t23270 * t86969 * t2719;
    let t86983 = F::cast_from(0.52089578783527170489e-1_f64) * t82099;
    (t86955, t86961, t86968, t86972, t86983)
}
