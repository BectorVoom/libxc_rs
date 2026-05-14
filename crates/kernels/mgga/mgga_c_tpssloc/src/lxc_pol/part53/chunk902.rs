//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 902/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk902<F: Float>(t114680: F, t114691: F, t114752: F, t116686: F, t116688: F, t121541: F, t121546: F, t121550: F, t121560: F, t121563: F, t121574: F, t121612: F, t123584: F, t123626: F, t1499: F, t226: F, t235: F, t2617: F, t31993: F, t31996: F, t33969: F, t33971: F, t4162: F, t4182: F, t4234: F, t4281: F, t808: F, t812: F, t8738: F) -> (F,) {
    let t123663 = -t2617 * t33969 + 0.16449340668482264365e-1 * t114680 - t116686 - 0.16449340668482264365e-1 * t114691 + t116688 + 2.0 * t4281 * t123626 * t4182 + 0.6579736267392905746e-1 * t121541 - 0.3289868133696452873e-1 * t121546 + 0.3289868133696452873e-1 * t121550 - t812 * t31993 * t4234 + t4162 * t8738 + t808 * t33971 - 0.3289868133696452873e-1 * t121560 - 0.3289868133696452873e-1 * t121563 + 0.76763589786250567037e-1 * t114752 - 0.76763589786250567037e-1 * t121574 + 0.19739208802178717238e0 * t121612 + t226 * t235 * t123584 + t1499 * t31996;
    (t123663,)
}
