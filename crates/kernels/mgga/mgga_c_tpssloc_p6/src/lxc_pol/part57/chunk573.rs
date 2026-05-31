//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 573/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk573<F: Float>(t109: F, t652: F, t7461: F, t1453: F, t6530: F, t6529: F) -> (F, F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t7463 = F::cast_from(2.0_f64) * t652 * t7461;
    let t7464 = t6530 * t1453;
    let t7467 = piecewise3::<F>(t110, F::cast_from(0.0_f64), -t6529 - t7464 / F::cast_from(8.0_f64));
    (t7463, t7464, t7467)
}
