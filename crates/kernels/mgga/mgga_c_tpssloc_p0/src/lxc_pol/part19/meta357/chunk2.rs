//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1296/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1296<F: Float>(t41678: F, t41682: F, t41684: F, t41690: F, t41699: F, t41703: F, t41711: F, t41863: F, t41865: F, t41868: F, t41870: F, t41872: F, t41874: F, t41876: F) -> F {
    let t42187 = -F::cast_from(0.27545333333333333333e1_f64) * t41678 + F::new(0.41318e1) * t41682 + F::cast_from(0.21424148148148148148e1_f64) * t41684 + F::cast_from(0.68863333333333333334e1_f64) * t41690 - F::new(0.123954e2) * t41699 - F::new(0.103295e1) * t41703 + F::new(0.123954e2) * t41711 + F::cast_from(0.12349037037037037037e1_f64) * t41863 - F::cast_from(0.55570666666666666668e0_f64) * t41865 + F::cast_from(0.55570666666666666666e0_f64) * t41868 - F::cast_from(0.69463333333333333334e0_f64) * t41870 - F::cast_from(0.23154444444444444445e0_f64) * t41872 + F::cast_from(0.27785333333333333333e0_f64) * t41874 + F::cast_from(0.12349037037037037037e0_f64) * t41876;
    t42187
}
