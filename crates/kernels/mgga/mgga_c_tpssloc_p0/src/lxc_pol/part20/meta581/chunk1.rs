//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2149/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2149<F: Float>(t10908: F, t3114: F, t1036: F, t10438: F, t221: F, t339: F, t42813: F, t10283: F, t995: F, t10931: F, t135: F, t973: F) -> (F, F, F, F, F) {
    let t43301 = t3114 * t10908;
    let t43303 = t10438 * t1036;
    let t43307 = F::new(5.0) / F::new(486.0) * t339 * t221 * t42813;
    let t43310 = t10283 * t995;
    let t43313 = t973 * t135 * t10931;
    (t43301, t43303, t43307, t43310, t43313)
}
