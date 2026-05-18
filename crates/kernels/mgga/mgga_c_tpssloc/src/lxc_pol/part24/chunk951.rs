//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 951/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk951<F: Float>(t2884: F, t307: F, t302: F, t10743: F, t2888: F, t10294: F, t10544: F, t10296: F, t10298: F, t10300: F, t10302: F, t10307: F, t10314: F, t10320: F, t10323: F, t10530: F, t10538: F, t10547: F, t10550: F) -> (F, F, F) {
    let t10770 = F::new(1.0) / t2884 / t307;
    let t10771 = t302 * t10770;
    let t10772 = t10743 * t2888;
    let t10784 = F::new(0.46308888888888888888e0) * t10294;
    let t10785 = F::new(0.16068111111111111111e1) * t10544;
    let t10789 = -F::new(0.103295e1) * t10530 - F::new(0.34731666666666666667e0) * t10296 + F::new(0.20839e0) * t10302 + F::new(0.69463333333333333335e-1) * t10298 - F::new(0.46308888888888888889e-1) * t10307 - F::new(0.104195e0) * t10323 + F::new(0.309885e1) * t10538 - F::new(0.104195e0) * t10314 + F::new(0.62517e0) * t10320 - t10784 - t10785 - F::new(0.52945875e1) * t10547 + F::new(0.94674375e0) * t10550 - F::new(0.41678000000000000001e0) * t10300;
    (t10771, t10772, t10789)
}
