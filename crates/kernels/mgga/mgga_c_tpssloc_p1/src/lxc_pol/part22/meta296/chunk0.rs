//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1458/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1458<F: Float>(t13847: F, t2990: F, t2986: F, t2987: F, t4540: F, t2989: F, t3966: F, t2960: F, t4506: F, t10224: F, t1592: F, t973: F) -> (F, F, F, F, F, F) {
    let t13848 = t13847 * t2990;
    let t13850 = F::cast_from(0.18518518518518518518e-3_f64) * t2986 * t13848;
    let t13851 = t2987 * t4540;
    let t13861 = t2989 * t3966;
    let t13893 = F::cast_from(0.49382716049382716048e-3_f64) * t2960 * t4506;
    let t13895 = t10224 * t1592;
    let t13896 = t973 * t13895;
    (t13850, t13851, t13861, t13893, t13895, t13896)
}
