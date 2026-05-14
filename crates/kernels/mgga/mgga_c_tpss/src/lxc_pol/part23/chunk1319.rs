//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1319/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1319<F: Float>(t1279: F, t20128: F, t117: F, t547: F, t65458: F, t20985: F, t550: F, t21007: F, t546: F, t13136: F, t1338: F, t1897: F, t19187: F, t20950: F, t3499: F, t626: F, t63712: F, t63715: F, t63718: F, t63725: F, t63728: F, t63730: F, t63740: F, t63742: F, t63744: F, t63746: F, t63748: F, t63751: F, t63753: F, t65055: F, t65058: F, t65059: F) -> (F, F, F, F, F) {
    let t66131 = 6.0 * t1279 * t20128;
    let t66134 = 3.0 * t547 * t117 * t65458;
    let t67886 = 2.0 * t20985 * t550;
    let t67888 = 2.0 * t546 * t21007;
    let t67904 = -2.0 * t1338 * t19187 * t626 - 2.0 * t13136 * t1897 - 4.0 * t20950 * t3499 + t63712 - t63715 - t63718 - t63725 - t63728 - t63730 - t63740 - t63742 - t63744 - t63746 - t63748 - t63751 - t63753 + t65055 + t65058 + t65059;
    (t66131, t66134, t67886, t67888, t67904)
}
