//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 520/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk520<F: Float>(t1539: F, t882: F, t123: F, t881: F, t291: F, t880: F) -> (F, F, F, F, F) {
    let t1540 = t882 * t1539;
    let t1541 = t123 * t1540;
    let t1543 = -t881 - F::cast_from(0.17808333333333333333e-1_f64) * t1541;
    let t1545 = F::cast_from(0.621814e-1_f64) * t1543 * t291;
    let t1547 = -t880 / F::cast_from(3.0_f64) - t1541 / F::cast_from(3.0_f64);
    (t1540, t1541, t1543, t1545, t1547)
}
