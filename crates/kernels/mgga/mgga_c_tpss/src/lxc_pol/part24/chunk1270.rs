//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1270/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1270<F: Float>(t21253: F, t5710: F, t19602: F, t6243: F, t1163: F, t13133: F, t13554: F, t19448: F, t21021: F, t3493: F, t485: F, t6117: F, t68848: F, t68850: F, t68853: F, t68857: F, t68859: F, t68861: F, t68863: F, t68865: F, t68867: F, t68870: F, t68891: F, t68898: F, t68905: F) -> (F,) {
    let t68907 = 3.0 * t21253 * t5710;
    let t68909 = 2.0 * t6243 * t19602;
    let t68910 = -2.0 * t1163 * t21021 - 4.0 * t13133 * t6117 - 4.0 * t13554 * t6117 - 4.0 * t19448 * t3493 - 2.0 * t485 * t68898 - t68848 - t68850 - t68853 + t68857 - t68859 - t68861 - t68863 + t68865 - t68867 + t68870 - t68891 + t68905 + t68907 - t68909;
    (t68910,)
}
