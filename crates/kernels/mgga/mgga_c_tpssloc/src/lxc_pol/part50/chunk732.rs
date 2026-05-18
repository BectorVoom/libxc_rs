//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 732/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk732<F: Float>(t1987: F, t794: F, t6897: F, t1372: F, t225: F, t567: F, t214: F, t1985: F, t1377: F) -> (F, F, F, F, F, F) {
    let t6898 = t794 * t1987;
    let t6899 = t6897 * t6898;
    let t6900 = F::new(0.41123351671205660912e-2) * t6899;
    let t6902 = t1372 * t225 * t567;
    let t6903 = t214 * t6902;
    let t6904 = t1985 * t6903;
    let t6906 = t225 * t1377;
    (t6898, t6900, t6902, t6903, t6904, t6906)
}
