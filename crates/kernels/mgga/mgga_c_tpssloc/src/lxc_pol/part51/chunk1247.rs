//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1247/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1247<F: Float>(t1880: F, t25216: F, t31366: F, t121401: F, t6572: F, t114944: F, t114945: F, t118913: F, t118916: F, t118917: F, t118918: F, t13053: F, t13065: F, t1912: F, t26713: F, t31400: F, t31423: F, t4147: F, t4301: F, t6663: F, t8563: F, t92439: F) -> (F,) {
    let t121713 = t1880 * t31366 * t25216;
    let t121716 = t1880 * t121401 * t6572;
    let t121725 = -t118913 - 0.82246703342411321825e-2 * t121713 + t118916 - 0.82246703342411321825e-2 * t121716 - t4147 * t31400 - t26713 * t6663 + t114944 - t31423 * t4301 - t92439 * t1912 + t118917 + t118918 - t13053 * t8563 - t13065 * t8563 + 0.19190897446562641759e-1 * t114945;
    (t121725,)
}
