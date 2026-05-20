//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2216/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2216<F: Float>(t10469: F, t23470: F, t3: F, t82986: F, t23437: F, t4630: F, t25641: F, t82943: F, t1933: F, t1937: F, t3966: F, t14222: F, t1597: F, t1622: F, t23544: F, t23548: F, t25580: F, t25600: F, t25601: F, t25658: F, t3032: F, t3040: F, t3098: F, t360: F, t4579: F, t4636: F, t6722: F, t6729: F, t6735: F, t83071: F, t83075: F, t83215: F, t83220: F) -> (F, F) {
    let t88537 = t82986 * t3 * t23470 * t10469;
    let t88548 = t23437 * t4630 / F::new(216.0);
    let t88566 = F::cast_from(0.16149102437656156342e-2_f64) * t82943 * t25641;
    let t88569 = F::cast_from(0.20186378047070195428e-3_f64) * t1933 * t3966 * t1937;
    let t88570 = F::cast_from(0.10093189023535097714e-3_f64) * t88537 * t25658 * t3032 * t3040 * t360 + t83071 * t1622 / F::new(2304.0) + t23544 * t4636 / F::new(1152.0) - t88548 - t25580 * t3098 / F::new(1152.0) - F::cast_from(0.16149102437656156342e-2_f64) * t83075 - t83220 * t4579 / F::new(216.0) + F::cast_from(0.16149102437656156342e-2_f64) * t6722 * t25600 * t6735 - F::cast_from(0.20186378047070195428e-3_f64) * t1933 * t6729 * t1597 * t6735 - t83215 * t14222 / F::new(1152.0) - F::cast_from(0.10093189023535097714e-3_f64) * t25601 * t23548 - t88566 + t88569;
    (t88537, t88570)
}
