//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 564/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk564<F: Float>(t6528: F, t6548: F, t6564: F, t6579: F, t6586: F, t6602: F, t6617: F, t2048: F, t225: F) -> (F, F, F, F, F, F, F, F) {
    let t7053 = F::new(2.0) / F::new(3.0) * t6528;
    let t7067 = F::new(0.38381794893125283518e-1) * t6548;
    let t7069 = F::new(0.82246703342411321825e-2) * t6564;
    let t7074 = F::new(7.0) / F::new(144.0) * t6579;
    let t7076 = F::new(0.28260929265898273597e-2) * t6586;
    let t7078 = F::new(0.67287926823567318088e-4) * t6602;
    let t7082 = F::new(7.0) / F::new(1152.0) * t6617;
    let t7087 = t2048 * t225;
    (t7053, t7067, t7069, t7074, t7076, t7078, t7082, t7087)
}
