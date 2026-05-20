//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1534/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1534<F: Float>(t1041: F, t14084: F, t14085: F, t14117: F, t14508: F, t14511: F, t1622: F, t17734: F, t17738: F, t17878: F, t17885: F, t17890: F, t3048: F, t3117: F, t3130: F, t378: F, t4596: F, t4600: F, t4636: F, t4644: F, t5857: F, t5861: F, t973: F) -> F {
    let t17900 = t14084 + t14508 * t4596 / F::new(768.0) - t14511 * t4600 / F::new(1536.0) + t3130 * t17734 / F::new(768.0) + t973 * t17738 / F::new(288.0) + t17878 * t378 / F::new(3072.0) - F::new(5.0) / F::new(2592.0) * t3048 * t5861 + F::new(5.0) / F::new(20736.0) * t17885 + t3117 * t5857 / F::new(4608.0) + t1041 * t17890 / F::new(4608.0) + F::new(5.0) / F::new(13824.0) * t3117 * t5861 + t14085 * t1622 / F::new(2304.0) + t4644 * t4636 / F::new(2304.0) - t14117 / F::new(6912.0);
    t17900
}
