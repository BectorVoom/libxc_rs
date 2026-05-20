//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2088/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2088<F: Float>(t225: F, t3787: F, t562: F, t22751: F, t26385: F, t26389: F, t26467: F, t6914: F, t26426: F, t81046: F, t22690: F, t7732: F, t81195: F) -> (F, F, F, F, F, F, F) {
    let t91005 = t225 * t3787;
    let t91006 = t91005 * t562;
    let t91010 = t22751 * t26385;
    let t91011 = F::cast_from(0.76763589786250567036e-1_f64) * t91010;
    let t91064 = t22751 * t26389;
    let t91065 = F::cast_from(0.76763589786250567036e-1_f64) * t91064;
    let t91076 = t6914 * t26467;
    let t91077 = F::cast_from(0.38381794893125283518e-1_f64) * t91076;
    let t91078 = t81046 * t26426;
    let t91081 = t81195 * t22690 * t7732;
    (t91005, t91006, t91011, t91065, t91077, t91078, t91081)
}
