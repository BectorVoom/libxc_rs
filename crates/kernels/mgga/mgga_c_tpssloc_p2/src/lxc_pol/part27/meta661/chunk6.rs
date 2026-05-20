//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2320/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2320<F: Float>(t22751: F, t26389: F, t1992: F, t22897: F, t3792: F, t90870: F, t26467: F, t6914: F, t26426: F, t81046: F, t22690: F, t7732: F, t81195: F) -> (F, F, F, F, F) {
    let t91064 = t22751 * t26389;
    let t91065 = F::cast_from(0.76763589786250567036e-1_f64) * t91064;
    let t91074 = t1992 * t22897 * t90870 * t3792;
    let t91076 = t6914 * t26467;
    let t91077 = F::cast_from(0.38381794893125283518e-1_f64) * t91076;
    let t91078 = t81046 * t26426;
    let t91081 = t81195 * t22690 * t7732;
    (t91065, t91074, t91077, t91078, t91081)
}
