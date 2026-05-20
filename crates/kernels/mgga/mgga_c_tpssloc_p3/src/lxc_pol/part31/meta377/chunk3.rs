//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1331/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1331<F: Float>(t10143: F, t5664: F, t12895: F, t13121: F, t1484: F, t16697: F, t16699: F, t16700: F, t16703: F, t16705: F, t16707: F, t16708: F, t16709: F, t16712: F, t16715: F, t16719: F, t1877: F, t193: F, t2522: F, t262: F, t5527: F, t776: F, t868: F, t9853: F, t9859: F, t9894: F, t9907: F, t9921: F) -> F {
    let t17120 = t5664 * t10143;
    let t17131 = F::new(6.0) * t193 * t262 * t5527 * t776 + F::new(6.0) * t12895 * t1484 * t2522 + F::new(2.0) * t17120 * t1877 * t868 - t13121 - t16697 + t16699 - t16700 + t16703 + t16705 + t16707 - t16708 + t16709 - t16712 + t16715 + t16719 + t9853 + t9859 - t9894 + t9907 - t9921;
    t17131
}
