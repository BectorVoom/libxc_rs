//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1291/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1291<F: Float>(t18754: F, t5999: F, t18746: F, t43895: F, t63361: F, t78057: F, t78084: F, t78087: F, t78090: F, t78093: F, t78095: F, t78097: F, t78100: F, t78103: F, t78105: F, t78000: F, t78019: F, t78082: F) -> (F, F, F) {
    let t78107 = t18754 * t5999;
    let t78109 = t18746 * t5999;
    let t78112 = -0.11038e0 * t78084 - 0.99342e0 * t78087 + 0.66228e0 * t78090 + 0.298026e1 * t78093 + 0.258925e1 * t78095 + t43895 + 0.247573125e0 * t78097 + 0.22076e0 * t78100 + 0.16102666666666666667e1 * t63361 + 0.3300975e0 * t78103 - 0.51785e1 * t78105 + 0.11651625e2 * t78107 - 0.247573125e0 * t78109 - 0.72462e1 * t78057;
    let t78114 = t78000 + t78019 + t78082 + t78112;
    (t78107, t78109, t78114)
}
