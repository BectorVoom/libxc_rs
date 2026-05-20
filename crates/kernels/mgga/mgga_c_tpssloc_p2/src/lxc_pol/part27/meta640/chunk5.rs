//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2168/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2168<F: Float>(t87535: F, t13388: F, t1888: F, t6646: F, t13385: F, t22996: F, t23185: F, t4283: F, t81914: F, t25300: F, t81591: F, t1484: F, t6552: F, t6637: F, t81658: F) -> (F, F, F, F, F, F) {
    let t87536 = F::cast_from(0.38381794893125283518e-1_f64) * t87535;
    let t87538 = t1888 * t6646 * t13388;
    let t87541 = t1888 * t22996 * t13385;
    let t87544 = t23185 * t81914 * t4283;
    let t87545 = F::cast_from(0.16449340668482264365e-1_f64) * t87544;
    let t87546 = t81591 * t25300;
    let t87547 = F::cast_from(0.76763589786250567036e-1_f64) * t87546;
    let t87554 = t6552 * t6637 * t81658 * t1484;
    (t87536, t87538, t87541, t87545, t87547, t87554)
}
