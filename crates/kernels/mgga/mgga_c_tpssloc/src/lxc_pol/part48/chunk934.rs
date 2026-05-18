//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 934/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk934<F: Float>(t114160: F, t6888: F, t6891: F, t2006: F, t794: F, t6897: F, t6907: F, t22724: F, t31127: F, t80645: F, t8458: F, t1985: F, t22934: F, t31137: F) -> (F, F, F, F, F, F) {
    let t114171 = F::new(0.6579736267392905746e-1) * t6888 * t114160 * t6891;
    let t114172 = t794 * t2006;
    let t114174 = t6897 * t114172 * t6907;
    let t114175 = F::new(0.16449340668482264365e-1) * t114174;
    let t114178 = F::new(0.52089578783527170489e-1) * t22724 * t31127;
    let t114187 = t6897 * t80645 * t8458;
    let t114188 = F::new(0.16449340668482264365e-1) * t114187;
    let t114193 = F::new(0.3289868133696452873e-1) * t1985 * t31137 * t22934;
    (t114171, t114172, t114175, t114178, t114188, t114193)
}
