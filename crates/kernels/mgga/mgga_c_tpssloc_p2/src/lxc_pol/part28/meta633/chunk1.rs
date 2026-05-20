//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2002/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2002<F: Float>(t1877: F, t2057: F, t24191: F, t24339: F, t2522: F, t25905: F, t25921: F, t25930: F, t25934: F, t26740: F, t26756: F, t6841: F, t7110: F, t7114: F, t84797: F, t89850: F, t89888: F, t89892: F, t89911: F, t89917: F, t89978: F, t92356: F, t92359: F, t92362: F, t92364: F) -> F {
    let t93211 = F::new(2.0) * t26756 * t89850 - t1877 * t7114 * t89978 / F::new(2.0) - t92356 - t1877 * t24339 * t25934 + F::new(3.0) / F::new(2.0) * t2522 * t2057 * t89888 + F::new(6.0) * t24191 * t89917 + F::new(3.0) / F::new(2.0) * t2522 * t2057 * t89911 + F::new(3.0) * t2522 * t2057 * t89892 + F::new(3.0) * t2522 * t7110 * t25905 - t1877 * t24339 * t25930 + t92359 - F::new(3.0) * t84797 * t25921 + F::new(3.0) * t2522 * t26740 * t6841 - t92362 + t92364;
    t93211
}
