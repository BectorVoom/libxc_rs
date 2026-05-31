//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2116/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2116<F: Float>(t2: F, t870: F, t584: F, t776: F, t22959: F, t1408: F, t2553: F, t10143: F, t606: F, t25374: F, t1877: F, t1915: F) -> (F, F, F, F) {
    let t86753 = t870 * t2;
    let t86755 = t86753 * t584 * t776;
    let t86757 = F::cast_from(6.0_f64) * t22959 * t86755;
    let t86764 = t1408 * t2553;
    let t86770 = t10143 * t606;
    let t86771 = t86770 * t25374;
    let t86775 = t1877 * t1915 * t584;
    (t86757, t86764, t86771, t86775)
}
