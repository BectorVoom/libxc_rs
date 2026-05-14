//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1299/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1299<F: Float>(t11457: F, t5620: F, t11460: F, t11614: F, t11618: F, t11623: F, t11667: F, t61334: F, t61336: F, t61341: F, t61350: F, t61361: F, t61363: F, t61387: F, t11687: F, t5610: F) -> (F, F) {
    let t64401 = t5620 * t11457 / 864.0;
    let t64403 = 5.0 / 5184.0 * t5620 * t11460;
    let t64404 = -t5620 * t11667 / 576.0 + t61334 / 2304.0 - t61336 / 324.0 - t61341 / 5184.0 + 11.0 / 324.0 * t61361 + t61363 / 81.0 + t61387 * t11614 / 256.0 - t61350 * t11618 / 256.0 + t5620 * t11623 / 384.0 - t64401 + t64403;
    let t64420 = t5610 * t11687;
    (t64404, t64420)
}
