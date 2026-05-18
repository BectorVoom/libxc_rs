//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1333/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1333<F: Float>(t114172: F, t22892: F, t7691: F, t114160: F, t1985: F, t7700: F, t114174: F, t22666: F, t32697: F, t3886: F, t7749: F, t1385: F, t1992: F, t22635: F) -> (F, F, F, F, F) {
    let t120308 = t22892 * t114172 * t7691;
    let t120309 = F::new(0.16449340668482264365e-1) * t120308;
    let t120312 = F::new(0.16449340668482264365e-1) * t1985 * t114160 * t7700;
    let t120313 = F::new(0.82246703342411321825e-2) * t114174;
    let t120316 = F::new(0.16449340668482264365e-1) * t1985 * t22666 * t32697;
    let t120317 = t3886 * t7749;
    let t120321 = F::new(0.3289868133696452873e-1) * t1992 * t22635 * t120317 * t1385;
    (t120309, t120312, t120313, t120316, t120321)
}
