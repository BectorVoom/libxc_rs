//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1829/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1829<F: Float>(t1404: F, t7222: F, t24447: F, t580: F, t2098: F, t3946: F, t1395: F, t7240: F, t1453: F, t81439: F, t26129: F, t81442: F) -> (F, F, F, F, F, F) {
    let t85381 = t7222 * t1404;
    let t85392 = t24447 * t580;
    let t85394 = t2098 * t3946;
    let t85397 = t1395 * t7240;
    let t86586 = t81439 * t1453;
    let t86588 = t81442 * t26129;
    (t85381, t85392, t85394, t85397, t86586, t86588)
}
