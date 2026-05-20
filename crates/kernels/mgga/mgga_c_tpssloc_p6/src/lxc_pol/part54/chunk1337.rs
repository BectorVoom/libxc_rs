//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1337/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1337<F: Float>(t1369: F, t32717: F, t1831: F, t31165: F, t5314: F, t8466: F, t22804: F, t32711: F, t22759: F, t26318: F, t6936: F, t1799: F, t22690: F, t22792: F, t6950: F) -> (F, F, F, F, F, F) {
    let t120377 = t32717 * t1369;
    let t120379 = t31165 * t1831;
    let t120381 = t8466 * t5314;
    let t120383 = t22804 * t32711;
    let t120388 = t6936 * t22759 * t26318;
    let t120393 = t22792 * t22690 * t6950 * t1799;
    (t120377, t120379, t120381, t120383, t120388, t120393)
}
