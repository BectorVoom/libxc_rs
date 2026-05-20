//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1514/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1514<F: Float>(t10255: F, t4531: F, t343: F, t4540: F, t984: F, t4546: F, t12606: F, t978: F, t977: F, t135: F, t340: F, t4548: F) -> (F, F, F, F) {
    let t13806 = t4531 * t10255;
    let t13812 = t4540 * t984 * t343;
    let t13813 = t4546 * t13812;
    let t13816 = t978 * t12606;
    let t13817 = t977 * t13816;
    let t13822 = t135 * t340;
    let t13823 = t13822 * t4548;
    (t13806, t13813, t13817, t13823)
}
