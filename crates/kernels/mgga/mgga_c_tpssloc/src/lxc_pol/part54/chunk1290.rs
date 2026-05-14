//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1290/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1290<F: Float>(t1437: F, t31860: F, t32343: F, t8513: F, t117480: F, t1433: F, t8663: F, t63: F, t641: F, t116082: F, t116124: F, t117483: F, t117499: F, t117516: F, t117518: F, t117527: F, t122960: F, t122964: F, t122988: F, t123001: F, t31857: F, t31868: F, t32328: F, t32338: F, t32340: F, t33669: F, t33677: F, t34122: F, t34132: F, t4017: F, t4021: F, t8824: F, t8825: F) -> (F,) {
    let t124834 = t31860 * t8513 * t32343 * t1437;
    let t124838 = t8663 * t8513 * t117480 * t1433;
    let t124844 = t641 * t63;
    let t124860 = 10.0 / 27.0 * t117483 + 5.0 / 12.0 * t122988 * t32328 - 5.0 / 36.0 * t33669 * t32340 + 5.0 / 12.0 * t116124 * t34122 + 5.0 / 12.0 * t116082 * t34122 + 5.0 / 12.0 * t31860 * t8513 * t8824 * t4021 + 5.0 / 12.0 * t123001 * t32328 - 5.0 / 36.0 * t33677 * t32340 - 10.0 / 9.0 * t124834 + 10.0 / 27.0 * t124838 - 5.0 / 36.0 * t31857 * t34132 - 5.0 / 36.0 * t31868 * t34132 - 5.0 / 36.0 * t8663 * t8513 * t124844 * t1433 - 5.0 / 36.0 * t8663 * t8513 * t32338 * t4017 - 20.0 / 27.0 * t117499 + 5.0 / 27.0 * t117516 + 5.0 / 27.0 * t117518 - t117527 - 5.0 / 72.0 * t122960 * t8825 - 5.0 / 72.0 * t122964 * t8825;
    (t124860,)
}
