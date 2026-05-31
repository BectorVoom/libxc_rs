//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 669/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk669<F: Float>(t6528: F, t6548: F, t6564: F, t6579: F, t6586: F, t6602: F, t6617: F, t2048: F, t225: F) -> (F, F, F, F, F, F, F, F) {
    let t7053 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6528;
    let t7067 = F::cast_from(0.38381794893125283518e-1_f64) * t6548;
    let t7069 = F::cast_from(0.82246703342411321825e-2_f64) * t6564;
    let t7074 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t6579;
    let t7076 = F::cast_from(0.28260929265898273597e-2_f64) * t6586;
    let t7078 = F::cast_from(0.67287926823567318088e-4_f64) * t6602;
    let t7082 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t6617;
    let t7087 = t2048 * t225;
    (t7053, t7067, t7069, t7074, t7076, t7078, t7082, t7087)
}
