//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1707/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1707<F: Float>(t5614: F, t6614: F, t5617: F, t815: F, t6605: F, t2628: F, t5585: F, t23146: F, t5593: F, t1894: F, t236: F, t5544: F) -> (F, F, F, F, F, F, F) {
    let t28370 = t6614 * t5614;
    let t28372 = t815 * t5617;
    let t28373 = t6605 * t28372;
    let t28375 = t2628 * t5585;
    let t28376 = t6605 * t28375;
    let t28380 = t23146 * t5593;
    let t28383 = t1894 * t236 * t5544;
    (t28370, t28372, t28373, t28375, t28376, t28380, t28383)
}
