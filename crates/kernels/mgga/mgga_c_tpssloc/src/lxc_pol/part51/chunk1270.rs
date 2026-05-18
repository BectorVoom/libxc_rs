//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1270/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1270<F: Float>(t1902: F, t794: F, t23164: F, t6555: F, t6562: F, t6572: F, t234: F, t6624: F, t22893: F, t30677: F, t23168: F, t30678: F) -> (F, F, F, F, F, F) {
    let t112943 = t794 * t1902;
    let t112945 = t23164 * t112943 * t6555;
    let t112948 = t6562 * t112943 * t6572;
    let t112951 = t234 * t6624;
    let t112961 = t23164 * t22893 * t30677;
    let t112968 = t23168 * t30678;
    (t112943, t112945, t112948, t112951, t112961, t112968)
}
