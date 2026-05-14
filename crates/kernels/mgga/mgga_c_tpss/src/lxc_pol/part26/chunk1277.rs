//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1277/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1277<F: Float>(t13627: F, t1760: F, t5754: F, t13119: F, t6274: F, t1270: F, t13671: F, t5708: F, t1268: F, t5458: F, t19579: F, t65898: F, t19620: F, t26620: F, t4478: F, t19605: F, t6243: F) -> (F, F, F, F, F, F) {
    let t68817 = 2.0 * t1760 * t5754 * t13627;
    let t68822 = 2.0 * t1760 * t6274 * t13119;
    let t68823 = t1270 * t13671;
    let t68826 = 3.0 * t1760 * t5708 * t68823;
    let t68827 = t5458 * t1268;
    let t68830 = 6.0 * t19579 * t65898 * t68827;
    let t68833 = 12.0 * t19620 * t26620 * t4478;
    let t68835 = 6.0 * t6243 * t19605;
    (t68817, t68822, t68826, t68830, t68833, t68835)
}
