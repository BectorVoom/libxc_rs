//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 467/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk467<F: Float>(t1395: F, t1401: F, t577: F, t671: F, t582: F, t586: F, t589: F, t593: F, t596: F, t600: F, t4: F, t581: F) -> (F, F, F) {
    let t1404 = F::cast_from(0.45e1_f64) * t1395 * t577 + F::cast_from(0.135e2_f64) * t1401 * t671;
    let t1406 = -t582 - t586 - t589 - t593 - t596 - t600;
    let t1408 = -t4 - t581;
    (t1404, t1406, t1408)
}
