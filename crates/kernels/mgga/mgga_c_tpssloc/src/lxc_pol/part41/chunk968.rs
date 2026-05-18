//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 968/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk968<F: Float>(t12858: F, t763: F, t1472: F, t2517: F, t4303: F, t870: F, t2430: F, t4205: F, t1409: F, t750: F, t607: F, t4194: F) -> (F, F, F, F, F) {
    let t12860 = F::new(0.11696447245269292414e1) * t12858 * t763;
    let t12861 = t1472 * t2517;
    let t12895 = t4303 * t870;
    let t12922 = F::new(8.0) * t4205 * t2430;
    let t12923 = t750 * t1409;
    let t12924 = t12923 * t607;
    let t12926 = F::new(24.0) * t4194 * t12924;
    (t12860, t12861, t12895, t12922, t12926)
}
