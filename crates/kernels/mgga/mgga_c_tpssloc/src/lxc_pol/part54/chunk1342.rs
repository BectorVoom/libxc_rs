//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1342/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1342<F: Float>(t114057: F, t114060: F, t22751: F, t32741: F, t114069: F, t1799: F, t6637: F, t6888: F, t31193: F, t5187: F, t22892: F, t22893: F, t32740: F) -> (F, F, F, F, F, F) {
    let t120468 = F::new(0.76763589786250567036e-1) * t114057;
    let t120469 = F::new(0.16449340668482264365e-1) * t114060;
    let t120470 = t22751 * t32741;
    let t120471 = F::new(0.76763589786250567037e-1) * t120470;
    let t120483 = F::new(0.3289868133696452873e-1) * t6888 * t6637 * t114069 * t1799;
    let t120487 = F::new(0.3289868133696452873e-1) * t6888 * t6637 * t31193 * t5187;
    let t120490 = t22892 * t22893 * t32740;
    (t120468, t120469, t120471, t120483, t120487, t120490)
}
