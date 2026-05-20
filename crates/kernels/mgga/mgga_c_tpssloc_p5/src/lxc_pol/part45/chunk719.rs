//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 719/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk719<F: Float>(t22776: F, t6936: F, t6604: F, t6919: F, t6937: F, t6950: F, t835: F, t1336: F, t1369: F, t3876: F, t6952: F, t3777: F, t6951: F) -> (F, F, F, F, F, F, F) {
    let t22777 = t6936 * t22776;
    let t22779 = t6919 * t6604;
    let t22780 = t22779 * t6937;
    let t22782 = t6950 * t835;
    let t22783 = t1336 * t22782;
    let t22784 = t22783 * t1369;
    let t22785 = F::new(7.0) / F::new(288.0) * t22784;
    let t22786 = t6952 * t3876;
    let t22788 = t3777 * t6951;
    (t22777, t22779, t22780, t22784, t22785, t22786, t22788)
}
