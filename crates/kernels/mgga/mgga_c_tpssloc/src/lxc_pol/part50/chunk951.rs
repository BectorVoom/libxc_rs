//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 951/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk951<F: Float>(t1025: F, t1046: F, t1935: F, t30813: F, t30817: F, t30821: F, t30824: F, t30829: F, t30833: F, t30837: F, t30840: F, t378: F, t6723: F, t6730: F, t6742: F, t8384: F) -> (F,) {
    let t30843 = -0.32298204875312312685e-2 * t6723 * t8384 + t30813 + 0.40372756094140390856e-3 * t6730 * t8384 - 0.40372756094140390856e-3 * t1935 * t30817 + 0.40372756094140390856e-3 * t6742 * t30821 + t30824 * t378 / 1536.0 + t30829 * t1025 / 1536.0 - t30833 * t378 / 288.0 + t30837 + t30840 * t1046 / 2304.0;
    (t30843,)
}
