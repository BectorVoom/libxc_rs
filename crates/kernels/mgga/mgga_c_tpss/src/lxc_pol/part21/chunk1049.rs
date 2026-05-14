//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1049/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1049<F: Float>(t11687: F, t946: F, t1407: F, t242: F, t8951: F, t967: F, t2748: F, t3969: F, t2675: F, t3950: F, t11663: F, t11667: F, t11671: F, t11675: F, t11679: F, t11683: F, t1471: F, t2682: F, t2740: F, t3952: F, t8963: F) -> (F, F, F) {
    let t11688 = t946 * t11687;
    let t11691 = t242 * t8951 * t1407;
    let t11692 = t967 * t11691;
    let t11697 = t2748 * t3969 / 648.0;
    let t11701 = t242 * t2675 * t3950;
    let t11703 = t946 * t11701 / 2304.0;
    let t11704 = 5.0 / 5184.0 * t967 * t11663 - t967 * t11667 / 1152.0 - t967 * t11671 / 2304.0 + t2740 * t11675 / 2304.0 + t2740 * t11679 / 4608.0 + 5.0 / 13824.0 * t2740 * t11683 - t11688 / 13824.0 - t11692 / 20736.0 + 19.0 / 2592.0 * t8963 * t1471 - t11697 - t2682 * t3952 / 288.0 + t11703;
    (t11691, t11701, t11704)
}
