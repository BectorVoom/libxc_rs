//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1410/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1410<F: Float>(t22574: F, t28830: F, t33136: F, t106956: F, t1874: F, t107496: F, t107499: F, t107507: F, t107509: F, t107512: F, t107515: F, t107519: F, t107521: F, t107523: F, t107525: F, t107527: F, t107530: F, t1442: F, t1458: F, t1774: F, t27996: F, t28811: F, t33085: F, t5494: F, t6287: F, t6468: F, t652: F, t7451: F, t7681: F) -> F {
    let t107533 = F::new(18.0) * t22574 * t33136 * t28830;
    let t107539 = F::new(6.0) * t106956 * t1874;
    let t107543 = -F::new(6.0) * t1458 * t28811 * t652 - F::new(3.0) * t1442 * t28811 - F::new(6.0) * t1774 * t27996 - F::new(6.0) * t33085 * t5494 - F::new(3.0) * t6287 * t7451 + F::new(3.0) * t6468 * t7681 - t107496 - t107499 - t107507 - t107509 + t107512 + t107515 - t107519 - t107521 - t107523 - t107525 - t107527 - t107530 - t107533 - t107539;
    t107543
}
