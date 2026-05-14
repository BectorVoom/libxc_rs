//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1275/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1275<F: Float>(t122166: F, t6888: F, t6891: F, t115332: F, t7691: F, t6897: F, t8621: F, t90544: F, t22633: F, t22635: F, t31558: F, t97721: F, t120606: F, t120607: F, t120611: F, t120612: F, t120616: F, t120621: F, t26996: F, t27068: F, t31642: F, t33301: F, t3758: F, t5321: F, t6958: F, t6963: F) -> (F,) {
    let t122377 = t6888 * t122166 * t6891;
    let t122384 = t6888 * t115332 * t7691;
    let t122390 = t6897 * t90544 * t8621;
    let t122394 = t22633 * t22635 * t31558 * t97721;
    let t122396 = -t120606 + t120607 - 0.16449340668482264365e-1 * t122377 + 2.0 * t6958 * t26996 + 2.0 * t3758 * t33301 - 0.16449340668482264365e-1 * t122384 + t120611 + t120612 - t120616 - t5321 * t31642 + 2.0 * t27068 * t6963 + 0.41123351671205660912e-2 * t122390 - t120621 - 0.3289868133696452873e-1 * t122394;
    (t122396,)
}
