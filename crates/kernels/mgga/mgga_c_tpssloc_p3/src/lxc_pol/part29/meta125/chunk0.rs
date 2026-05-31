//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 746/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk746<F: Float>(t2793: F, t913: F, t2792: F, t273: F, t276: F, t896: F, t2764: F, t2766: F, t2773: F, t2778: F, t2782: F, t894: F) -> (F, F, F, F, F, F, F, F) {
    let t2794 = t2793 * t913;
    let t2796 = F::cast_from(2.0_f64) * t2792 * t2794;
    let t2798 = F::cast_from(1.0_f64) / t276 / t273;
    let t2799 = t896 * t896;
    let t2800 = t2798 * t2799;
    let t2802 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2764;
    let t2807 = t2802 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2766 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2773 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2778 - t2782 / F::cast_from(3.0_f64);
    let t2808 = t894 * t2807;
    (t2794, t2796, t2798, t2799, t2800, t2802, t2807, t2808)
}
