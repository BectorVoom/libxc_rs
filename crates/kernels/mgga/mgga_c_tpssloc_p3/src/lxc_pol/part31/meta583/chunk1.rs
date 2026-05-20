//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1824/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1824<F: Float>(t81080: F, t26462: F, t6914: F, t22705: F, t26414: F, t81228: F, t26415: F, t81159: F, t26418: F, t7736: F, t80854: F, t81064: F) -> (F, F, F, F, F, F) {
    let t90925 = F::cast_from(0.10417915756705434098e0_f64) * t81080;
    let t90956 = t6914 * t26462;
    let t90961 = t81228 * t22705 * t26414;
    let t90963 = t81159 * t26415;
    let t90970 = t6914 * t26418;
    let t90980 = t81064 * t80854 * t7736;
    (t90925, t90956, t90961, t90963, t90970, t90980)
}
