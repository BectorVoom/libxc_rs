//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1215/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1215<F: Float>(t33: F, t823: F, t3683: F, t14076: F, t18246: F, t1006: F, t1364: F, t3610: F, t19809: F, t8096: F) -> (F, F, F, F, F, F, F) {
    let t20011 = t823 * t33;
    let t20012 = t20011 * t3683;
    let t20018 = t18246 * t14076;
    let t20021 = t1006 * t1364;
    let t20025 = t33 * t3610;
    let t20041 = t18246 * t19809;
    let t20047 = t8096 * t33;
    (t20011, t20012, t20018, t20021, t20025, t20041, t20047)
}
