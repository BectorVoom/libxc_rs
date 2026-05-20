//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1119/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1119<F: Float>(t5250: F, t5335: F, t1825: F, t3901: F, t1380: F, t5287: F, t1338: F, t68: F, t544: F) -> (F, F, F, F, F) {
    let t5336 = t5335 * t5250;
    let t5339 = t3901 * t1825;
    let t5341 = t1380 * t5287;
    let t5343 = t68 * t1338;
    let t5344 = t544 * t5343;
    (t5336, t5339, t5341, t5343, t5344)
}
