//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1998/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1998<F: Float>(t13191: F, t13196: F, t1484: F, t1530: F, t1877: F, t2057: F, t2379: F, t24335: F, t24344: F, t2522: F, t25374: F, t2553: F, t26744: F, t2745: F, t2749: F, t4314: F, t57893: F, t58009: F, t58071: F, t7114: F, t7845: F, t84766: F, t84791: F, t84800: F, t86713: F, t86717: F, t868: F, t86815: F, t92276: F, t93000: F) -> F {
    let t93099 = F::new(12.0) * t13191 * t2057 * t4314 + F::new(6.0) * t13196 * t2057 * t4314 + F::new(3.0) * t1484 * t24335 * t2522 - t1530 * t1877 * t84791 + F::new(4.0) * t1877 * t24344 * t58009 + F::new(2.0) * t1877 * t24344 * t86713 + F::new(4.0) * t1877 * t25374 * t84800 - t1877 * t26744 * t2745 + F::new(2.0) * t1877 * t2749 * t93000 - F::new(6.0) * t1877 * t84766 * t86717 - F::new(2.0) * t1877 * t868 * t92276 + F::new(6.0) * t2379 * t4314 * t7845 + F::new(3.0) * t2522 * t2553 * t7845 - F::new(6.0) * t2522 * t57893 * t7114 - F::new(6.0) * t2522 * t58071 * t7114 - F::new(3.0) * t2522 * t7114 * t86815;
    t93099
}
