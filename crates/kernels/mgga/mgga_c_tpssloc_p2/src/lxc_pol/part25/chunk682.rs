//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 682/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk682<F: Float>(t2075: F, t671: F, t6548: F, t6564: F, t2047: F, t798: F, t6579: F, t6586: F, t6602: F, t6617: F, t6582: F, t6594: F, t6607: F, t6610: F, t6615: F, t6622: F) -> (F, F, F, F, F) {
    let t7061 = t2075 * t671;
    let t7067 = F::cast_from(0.38381794893125283518e-1_f64) * t6548;
    let t7069 = F::cast_from(0.82246703342411321825e-2_f64) * t6564;
    let t7072 = t798 * t2047;
    let t7074 = F::new(7.0) / F::new(144.0) * t6579;
    let t7076 = F::cast_from(0.28260929265898273597e-2_f64) * t6586;
    let t7078 = F::cast_from(0.67287926823567318088e-4_f64) * t6602;
    let t7082 = F::new(7.0) / F::new(1152.0) * t6617;
    let t7084 = -t7074 - t6582 / F::new(24.0) - t7076 - F::cast_from(0.24223653656484234512e-2_f64) * t6594 - t7078 - F::cast_from(0.40372756094140390853e-3_f64) * t6607 + t6610 / F::new(768.0) - t6615 / F::new(768.0) - t7082 - t6622 / F::new(192.0);
    (t7061, t7067, t7069, t7072, t7084)
}
