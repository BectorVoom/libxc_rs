//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1750/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1750<F: Float>(t22833: F, t3809: F, t2002: F, t3773: F, t559: F, t1878: F, t557: F, t3766: F, t556: F, t598: F, t213: F, t1998: F, t236: F, t3734: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22834 = t22833 * t3809;
    let t22836 = t3773 * t2002;
    let t22837 = t22836 * t559;
    let t22839 = t1878 * t557;
    let t22840 = t22839 * t3766;
    let t22842 = t556 * t556;
    let t22843 = F::new(1.0) / t22842;
    let t22844 = t598 * t22843;
    let t22845 = t22844 * t213;
    let t22847 = t1998 * t236 * t3734;
    (t22834, t22836, t22837, t22839, t22840, t22842, t22843, t22844, t22845, t22847)
}
