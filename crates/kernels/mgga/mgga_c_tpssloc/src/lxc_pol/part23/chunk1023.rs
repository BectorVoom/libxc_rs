//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1023/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1023<F: Float>(t1441: F, t1458: F, t1799: F, t1824: F, t1484: F, t1530: F, t1409: F, t1615: F, t1845: F, t5456: F, t576: F, t460: F, t6144: F, t20: F, t60: F, t9108: F, t94: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28002 = t1441 * t1458;
    let t28099 = t1799 * t1824;
    let t28248 = t1484 * t1530;
    let t28651 = t1409 * t1615;
    let t28830 = t1799 * t1845;
    let t28893 = t576 * t5456;
    let t29614 = t6144 * t460;
    let t32253 = 1.0 / t60 / t20;
    let t35577 = t94 * t9108;
    (t28002, t28099, t28248, t28651, t28830, t28893, t29614, t32253, t35577)
}
