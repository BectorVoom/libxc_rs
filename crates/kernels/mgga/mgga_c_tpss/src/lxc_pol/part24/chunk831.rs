//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 831/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk831<F: Float>(t30: F, t33: F, t259: F, t479: F, t1742: F, t45: F, t5598: F, t5665: F, t581: F, t750: F, t821: F, t1006: F, t1692: F, t1713: F, t2439: F, t5586: F, t5590: F, t5664: F, t1749: F, t57: F, dens_threshold: F, rho0: F, rho1: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t5670 = piecewise3(t120, t5598, t1742 * t581 / 2.0 + t5665 * t45 / 2.0);
    let t5671 = t33 * t750;
    let t5678 = t33 * t821;
    let t5685 = 3.0 / 2.0 * t2439 * t1713 * t5671 + t1692 * t5586 * t33 / 2.0 - t1692 * t5590 * t5678 / 2.0 + t1692 * t1713 * t1006 / 2.0;
    let t5686 = piecewise3(t480, 0.0, t5664);
    let t5691 = piecewise3(t386, t5685, -t1749 * t581 / 2.0 + t5686 * t57 / 2.0);
    (t5670, t5671, t5678, t5686, t5691)
}
