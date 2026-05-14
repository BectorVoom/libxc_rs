//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 962/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk962<F: Float>(t10255: F, t4531: F, t343: F, t4540: F, t984: F, t4546: F, t12606: F, t978: F, t977: F, t135: F, t340: F, t4548: F, t973: F, t2970: F, t4522: F, t6733: F, t884: F) -> (F, F, F, F, F, F) {
    let t13806 = t4531 * t10255;
    let t13812 = t4540 * t984 * t343;
    let t13813 = t4546 * t13812;
    let t13816 = t978 * t12606;
    let t13817 = t977 * t13816;
    let t13822 = t135 * t340;
    let t13823 = t13822 * t4548;
    let t13825 = 0.55555555555555555554e-3 * t973 * t13823;
    let t13828 = t2970 * t4522;
    let t13830 = 0.18518518518518518518e-3 * t973 * t13828;
    let t13831 = t6733 * t884;
    (t13806, t13813, t13817, t13825, t13830, t13831)
}
