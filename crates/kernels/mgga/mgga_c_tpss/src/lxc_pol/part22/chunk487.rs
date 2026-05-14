//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 487/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk487<F: Float>(t118: F, t1760: F, t1796: F, t1800: F, t1830: F, t1834: F, t1846: F, t485: F, t544: F, t626: F, t3: F) -> (F, F, F) {
    let t1848 = -t118 * t1830 + t1760 * t1846 - t1796 * t485 - 2.0 * t1800 * t626 + t1834 * t544;
    let t1849 = t3 * t1848;
    let t1851 = param_d * t1848;
    (t1848, t1849, t1851)
}
