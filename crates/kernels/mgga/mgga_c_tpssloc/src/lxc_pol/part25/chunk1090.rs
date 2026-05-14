//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1090/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1090<F: Float>(t1081: F, t1877: F, t2057: F, t2068: F, t23781: F, t23789: F, t23796: F, t23807: F, t24191: F, t24335: F, t2522: F, t26563: F, t26756: F, t3231: F, t4314: F, t6841: F, t7110: F, t7114: F, t82320: F, t83559: F, t83585: F, t83592: F, t83596: F, t83624: F, t83645: F, t83651: F, t84766: F, t84797: F, t84800: F) -> (F,) {
    let t85337 = 3.0 * t26756 * t83645 + 3.0 * t82320 * t2068 + 9.0 / 2.0 * t2522 * t2057 * t83596 + 9.0 / 2.0 * t2522 * t7110 * t23796 + 9.0 / 2.0 * t2522 * t2057 * t83592 + 9.0 / 2.0 * t2522 * t24335 * t6841 + 3.0 * t1877 * t84800 * t23807 + 3.0 / 2.0 * t1877 * t7110 * t3231 - 9.0 * t84797 * t23789 + 9.0 * t4314 * t7110 * t23781 - 3.0 * t1877 * t84766 * t83585 - 9.0 / 2.0 * t24191 * t83651 - t1877 * t7114 * t83559 / 2.0 + 3.0 / 2.0 * t1877 * t24335 * t1081 - 9.0 * t26563 * t83624;
    (t85337,)
}
