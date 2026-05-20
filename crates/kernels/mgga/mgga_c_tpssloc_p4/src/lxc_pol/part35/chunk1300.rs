//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1300/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1300<F: Float>(t15501: F, t3500: F, t7337: F, t27710: F, t3: F, t24740: F, t5064: F, t2121: F, t3427: F, t8077: F, t24771: F, t7999: F) -> (F, F, F, F, F) {
    let t95627 = t3500 * t7337 * t15501;
    let t95648 = t27710 * t3;
    let t95687 = t5064 * t24740;
    let t95726 = t2121 * t3427 * t8077;
    let t95768 = t7999 * t24771;
    (t95627, t95648, t95687, t95726, t95768)
}
