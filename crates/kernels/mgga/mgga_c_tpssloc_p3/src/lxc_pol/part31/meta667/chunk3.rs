//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1963/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1963<F: Float>(t1877: F, t2057: F, t24344: F, t2522: F, t26740: F, t26756: F, t28241: F, t28249: F, t28972: F, t4314: F, t46341: F, t5397: F, t7110: F, t7114: F, t7475: F, t7545: F, t84797: F, t92276: F, t98000: F, t98031: F, t98046: F, t98050: F, t98065: F, t98082: F, t98091: F, t98103: F) -> F {
    let t101283 = F::new(2.0) * t26756 * t98031 + F::new(2.0) * t26756 * t98065 + F::new(3.0) * t4314 * t7110 * t28241 + t1877 * t24344 * t98091 - F::new(3.0) * t84797 * t28249 + F::new(3.0) / F::new(2.0) * t2522 * t2057 * t98046 + F::new(3.0) / F::new(2.0) * t2522 * t2057 * t98050 + F::new(3.0) * t46341 * t28972 - t1877 * t7114 * t98082 / F::new(2.0) - F::new(3.0) * t26756 * t98000 + t26756 * t98103 + t1877 * t7110 * t5397 / F::new(2.0) - t1877 * t92276 * t7545 + F::new(3.0) * t2522 * t26740 * t7475;
    t101283
}
