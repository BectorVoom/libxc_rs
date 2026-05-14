//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 986/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk986<F: Float>(t11631: F, t11637: F, t11641: F, t11647: F, t11649: F, t11653: F, t11659: F, t1467: F, t2685: F, t2722: F, t2740: F, t3928: F, t3956: F, t8509: F, t8514: F, t8958: F, t8976: F, t9042: F) -> (F,) {
    let t11660 = -t8509 * t11631 / 4608.0 + t8976 * t3956 / 288.0 + t2722 * t11637 / 768.0 - t11641 / 1296.0 - t2685 * t3928 / 54.0 + t11647 + t9042 - t2740 * t11649 / 2304.0 + t8514 * t11653 / 2304.0 + 19.0 / 1728.0 * t8958 * t1467 - t11659;
    (t11660,)
}
