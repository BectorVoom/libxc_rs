//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1366/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1366<F: Float>(t4072: F, t652: F, t8595: F, t1983: F, t27144: F, t8643: F, t31526: F, t7685: F, t33483: F, t868: F, t1914: F, t26756: F, t584: F, t86730: F) -> (F, F, F, F, F) {
    let t121240 = F::new(2.0) * t652 * t8595 * t4072;
    let t121253 = t1983 * t27144 * t8643;
    let t121254 = t7685 * t31526;
    let t121258 = t33483 * t868;
    let t121264 = t26756 * t86730 * t584 * t1914;
    (t121240, t121253, t121254, t121258, t121264)
}
