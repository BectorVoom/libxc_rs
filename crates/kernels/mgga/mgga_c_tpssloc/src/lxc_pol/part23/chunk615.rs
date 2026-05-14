//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 615/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk615<F: Float>(t1827: F, t3799: F, t1788: F, t588: F, t592: F, t546: F, t68: F, t1365: F, t1799: F, t1831: F, t3866: F, t1835: F, t225: F, t3787: F, t544: F, t1824: F, t562: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5255 = t3799 * t1827;
    let t5264 = t588 * t1788;
    let t5266 = t592 * t1788;
    let t5278 = t546 * t68;
    let t5279 = t1365 * t1799;
    let t5306 = t3866 * t1831;
    let t5321 = t1835 * t225;
    let t5333 = t68 * t3787;
    let t5334 = t544 * t5333;
    let t5335 = t562 * t1824;
    (t5255, t5264, t5266, t5278, t5279, t5306, t5321, t5333, t5334, t5335)
}
