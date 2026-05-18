//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1347/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1347<F: Float>(t120576: F, t114253: F, t114255: F, t2007: F, t254: F, t114278: F, t32694: F, t6914: F, t114291: F, t32735: F, t6883: F, t114296: F) -> (F, F, F, F, F, F, F, F, F) {
    let t120577 = F::new(0.82246703342411321825e-2) * t120576;
    let t120579 = F::new(0.38381794893125283518e-1) * t114253;
    let t120590 = F::new(0.76763589786250567036e-1) * t114255;
    let t120591 = t2007 * t254;
    let t120594 = F::new(0.16449340668482264365e-1) * t114278;
    let t120605 = t6914 * t32694;
    let t120606 = F::new(0.76763589786250567037e-1) * t120605;
    let t120607 = F::new(0.38381794893125283518e-1) * t114291;
    let t120610 = t6883 * t32735;
    let t120611 = F::new(0.38381794893125283518e-1) * t120610;
    let t120612 = F::new(0.38381794893125283518e-1) * t114296;
    (t120577, t120579, t120590, t120591, t120594, t120606, t120607, t120611, t120612)
}
