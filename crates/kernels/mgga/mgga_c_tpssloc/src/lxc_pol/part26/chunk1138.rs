//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1138/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1138<F: Float>(t2553: F, t606: F, t25: F, t9516: F, t2249: F, t868: F, t1877: F, t1915: F, t22951: F, t22959: F, t22961: F, t22964: F, t22968: F, t23286: F, t25013: F, t2522: F, t25372: F, t4314: F, t6542: F, t6666: F, t6670: F, t81470: F, t81476: F, t81483: F, t81486: F, t81489: F, t81492: F, t81501: F) -> (F,) {
    let t81505 = t606 * t2553;
    let t81509 = t25 * t9516;
    let t81513 = t2249 * t868;
    let t81520 = 9.0 * t25013 * t81470 + 9.0 / 2.0 * t2522 * t23286 * t6542 + 9.0 * t22959 * t81476 + 3.0 / 2.0 * t1877 * t6666 * t2249 - 9.0 * t81483 * t22961 - 9.0 * t25013 * t81486 - 9.0 / 2.0 * t22959 * t81489 + 3.0 * t25372 * t81492 + 9.0 * t2522 * t6666 * t22964 + 9.0 * t4314 * t6666 * t22951 + 9.0 / 2.0 * t2522 * t1915 * t81501 + 9.0 / 2.0 * t2522 * t1915 * t81505 + 3.0 / 2.0 * t2522 * t1915 * t81509 - 3.0 / 2.0 * t1877 * t6670 * t81513 + 9.0 / 2.0 * t2522 * t6666 * t22968;
    (t81520,)
}
