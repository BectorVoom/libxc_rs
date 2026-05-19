//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 875/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk875<F: Float>(t1894: F, t2047: F, t214: F, t1880: F, t235: F, t8543: F, t226: F, t8359: F) -> (F, F, F, F) {
    let t8556 = t1894 * t2047;
    let t8557 = t214 * t8556;
    let t8558 = t1880 * t8557;
    let t8560 = t235 * t8543;
    let t8562 = t8359 + F::cast_from(0.82246703342411321825e-2_f64) * t8558 + t226 * t8560;
    (t8556, t8557, t8560, t8562)
}
