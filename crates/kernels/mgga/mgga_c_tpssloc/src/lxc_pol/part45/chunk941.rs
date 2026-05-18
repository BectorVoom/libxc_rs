//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 941/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk941<F: Float>(t6665: F, t868: F, t25373: F, t23285: F, t28: F, t1081: F, t25927: F, t113069: F, t23788: F, t2240: F, t2244: F, t32: F) -> (F, F, F, F, F, F, F) {
    let t113123 = t6665 * t868;
    let t113124 = t25373 * t113123;
    let t113741 = t28 * t23285;
    let t113751 = t1081 * t6665;
    let t113764 = t25927 * t113123;
    let t113772 = t23788 * t113069;
    let t113824 = t2240 * t32 * t2244;
    (t113123, t113124, t113741, t113751, t113764, t113772, t113824)
}
