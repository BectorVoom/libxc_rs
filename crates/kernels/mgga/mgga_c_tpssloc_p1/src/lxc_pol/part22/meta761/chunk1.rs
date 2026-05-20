//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2563/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2563<F: Float>(t14850: F, t18677: F, t14838: F, t18680: F, t15207: F, t18640: F, t4802: F, t4824: F, t64103: F, t64292: F, t71793: F, t71795: F, t71797: F, t71800: F, t71803: F, t71806: F, t71809: F, t71811: F, t71814: F, t71817: F) -> (F, F, F) {
    let t71819 = F::new(18.0) * t14850 * t18677;
    let t71821 = F::new(12.0) * t14838 * t18680;
    let t71828 = t71793 - t71795 - t71797 - t71800 + t71803 + t71806 + t71809 - t71811 - t71814 - t71817 - t71819 + t71821 - F::new(6.0) * t64292 * t4802 + F::cast_from(0.96491876992155210402e2_f64) * t64103 * t4824 - F::new(6.0) * t15207 * t18640;
    (t71819, t71821, t71828)
}
