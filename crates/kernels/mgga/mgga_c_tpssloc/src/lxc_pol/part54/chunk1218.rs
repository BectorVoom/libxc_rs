//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1218/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1218<F: Float>(t6562: F, t8547: F, t86893: F, t214: F, t7823: F, t6552: F, t6555: F, t225: F, t33412: F, t1880: F, t25224: F, t31419: F, t118639: F, t118650: F, t118654: F, t118662: F, t118664: F, t118667: F, t118672: F, t2054: F, t2597: F, t33452: F, t866: F, t87837: F) -> (F, F) {
    let t121399 = t6562 * t86893 * t8547;
    let t121401 = t214 * t7823;
    let t121403 = t6552 * t121401 * t6555;
    let t121405 = t33412 * t225;
    let t121409 = t1880 * t25224 * t31419;
    let t121411 = t118639 + 2.0 * t2597 * t33452 + t118650 + 0.41123351671205660912e-2 * t121399 - 0.16449340668482264365e-1 * t121403 + t118654 - t118662 - t118664 + t118667 - t121405 * t866 - t87837 * t2054 - 0.82246703342411321825e-2 * t121409 - t118672;
    (t121401, t121411)
}
