//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 913/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk913<F: Float>(t1825: F, t32136: F, t33822: F, t553: F, t1336: F, t1814: F, t32130: F, t32132: F, t33278: F, t33282: F, t33286: F, t544: F, t8798: F) -> (F, F, F) {
    let t33839 = t32136 * t1825;
    let t33841 = t553 * t33822;
    let t33843 = -t32130 - F::cast_from(0.6579736267392905746e-1_f64) * t33278 - t32132 - F::cast_from(0.3289868133696452873e-1_f64) * t33282 + F::cast_from(0.3289868133696452873e-1_f64) * t33286 + t1814 * t8798 - t1336 * t33839 + t544 * t33841;
    (t33839, t33841, t33843)
}
