//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1122/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1122<F: Float>(t119862: F, t119867: F, t119874: F, t119877: F, t123091: F, t123093: F, t123095: F, t123097: F, t123112: F, t123113: F, t123115: F, t1458: F, t2314: F, t32572: F, t32609: F, t32674: F, t32676: F, t32679: F, t34203: F, t4034: F, t4077: F, t652: F) -> (F,) {
    let t125121 = -2.0 * t1458 * t32572 * t652 - 2.0 * t2314 * t34203 - 2.0 * t32609 * t4077 - 2.0 * t34203 * t4034 - t119862 - t119867 - t119874 + t119877 - 4.0 * t123091 - 4.0 * t123093 - 4.0 * t123095 - 4.0 * t123097 + 2.0 * t123112 - 4.0 * t123113 - 4.0 * t123115 - t32674 - t32676 - t32679;
    (t125121,)
}
