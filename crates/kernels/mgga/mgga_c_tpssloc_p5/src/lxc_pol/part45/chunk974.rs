//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 974/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk974<F: Float>(t1888: F, t232: F, t6646: F, t7084: F, t828: F, t22690: F, t23171: F, t31376: F, t31389: F, t6562: F, t794: F, t23012: F, t8557: F) -> (F, F, F, F) {
    let t114685 = t1888 * t6646 * t7084 * t828 * t232;
    let t114688 = t23171 * t22690 * t31376;
    let t114689 = F::cast_from(0.82246703342411321824e-2_f64) * t114688;
    let t114691 = t6562 * t794 * t31389;
    let t114693 = t23012 * t8557;
    (t114685, t114689, t114691, t114693)
}
