//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1360/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1360<F: Float>(t363: F, t42342: F, t42345: F, t43288: F, t3131: F, t3047: F, t3077: F, t10908: F, t3114: F, t1036: F, t10438: F, t221: F, t339: F, t42813: F) -> (F, F, F, F, F, F) {
    let t43291 = t42342 * t43288 * t363 * t42345;
    let t43292 = t3131 * t3131;
    let t43298 = t3077 * t3047;
    let t43301 = t3114 * t10908;
    let t43303 = t10438 * t1036;
    let t43307 = F::new(5.0) / F::new(486.0) * t339 * t221 * t42813;
    (t43291, t43292, t43298, t43301, t43303, t43307)
}
