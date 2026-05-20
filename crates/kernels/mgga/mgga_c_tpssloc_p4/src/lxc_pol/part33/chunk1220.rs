//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1220/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1220<F: Float>(t213: F, t80893: F, t12328: F, t2003: F, t12248: F, t59: F, t1336: F, t240: F, t2690: F, t6943: F, t22865: F, t6604: F) -> (F, F, F, F, F) {
    let t80894 = t80893 * t213;
    let t80899 = t2003 * t12328;
    let t80900 = F::new(595.0) / F::new(5184.0) * t80899;
    let t80901 = t12248 * t59;
    let t80903 = t1336 * t80901 * t240;
    let t80914 = t1336 * t6943 * t2690;
    let t80939 = t22865 * t6604;
    (t80894, t80900, t80903, t80914, t80939)
}
