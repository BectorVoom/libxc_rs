//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1089/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1089<F: Float>(t22635: F, t32704: F, t22633: F, t1807: F, t8470: F, t1799: F, t1998: F, t59: F, t6926: F, t1825: F, t6943: F, t6936: F) -> (F, F, F, F, F, F, F) {
    let t32705 = t22635 * t32704;
    let t32707 = F::new(0.3289868133696452873e-1) * t22633 * t32705;
    let t32708 = t1807 * t8470;
    let t32711 = t1998 * t59 * t1799;
    let t32712 = t6926 * t32711;
    let t32714 = t6943 * t1825;
    let t32715 = t6936 * t32714;
    (t32705, t32707, t32708, t32711, t32712, t32714, t32715)
}
