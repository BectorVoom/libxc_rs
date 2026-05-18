//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 915/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk915<F: Float>(t212: F, t562: F, t6890: F, t22642: F, t225: F, t6911: F, t1372: F, t214: F) -> (F, F, F, F, F) {
    let t22643 = t212 * t562;
    let t22644 = t22643 * t6890;
    let t22645 = t22642 * t22644;
    let t22646 = F::new(0.82246703342411321824e-2) * t22645;
    let t22656 = t6911 * t225;
    let t22666 = t214 * t1372;
    (t22643, t22645, t22646, t22656, t22666)
}
