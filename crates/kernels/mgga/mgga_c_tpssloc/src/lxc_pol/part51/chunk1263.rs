//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1263/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1263<F: Float>(t1377: F, t7936: F, t1307: F, t22633: F, t22635: F, t1992: F, t31558: F, t5353: F, t33310: F, t6883: F, t113941: F, t115306: F, t115308: F, t115318: F, t115331: F, t120201: F, t120209: F, t120213: F, t122121: F, t16030: F, t8637: F) -> (F,) {
    let t122124 = t1377 * t7936;
    let t122127 = t22633 * t22635 * t122124 * t1307;
    let t122131 = t1992 * t22635 * t31558 * t5353;
    let t122133 = t6883 * t33310;
    let t122137 = -t115306 + 0.41123351671205660912e-2 * t122121 + 0.41123351671205660912e-2 * t115308 + 0.16449340668482264365e-1 * t122127 + 0.16449340668482264365e-1 * t122131 + t120201 - t113941 + 0.19190897446562641759e-1 * t122133 - 0.82246703342411321824e-2 * t115318 - t16030 * t8637 - t115331 + t120209 + t120213;
    (t122137,)
}
