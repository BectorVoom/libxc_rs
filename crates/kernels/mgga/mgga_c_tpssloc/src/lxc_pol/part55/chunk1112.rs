//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1112/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1112<F: Float>(t4072: F, t576: F, t8319: F, t75795: F, t1395: F, t1458: F, t25994: F, t7266: F, t652: F, t6534: F, t8103: F, t26168: F, t8690: F, t33746: F, t6880: F, t7263: F) -> (F, F, F, F, F, F, F, F) {
    let t120833 = t576 * t4072;
    let t120835 = 27.0 * t120833 * t8319;
    let t120848 = 27.0 * t75795 * t8319;
    let t120849 = t1395 * t1458;
    let t120851 = 27.0 * t120849 * t8319;
    let t122875 = t7266 * t25994;
    let t122897 = t652 * t8103 * t6534;
    let t122910 = t8690 * t26168;
    let t122914 = t33746 * t6880;
    let t122917 = t7263 * t1458;
    (t120835, t120848, t120851, t122875, t122897, t122910, t122914, t122917)
}
