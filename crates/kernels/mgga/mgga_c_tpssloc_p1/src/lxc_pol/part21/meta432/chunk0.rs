//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1966/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1966<F: Float>(t15338: F, t3451: F, t3447: F, t14818: F, t14781: F, t14710: F, t11211: F, t11213: F, t11215: F, t11217: F, t11487: F, t14713: F, t14766: F, t14779: F, t14784: F, t14787: F, t14790: F, t14793: F, t14796: F, t14799: F) -> (F, F, F, F, F, F) {
    let t15339 = t15338 * t3451;
    let t15341 = F::cast_from(0.18518518518518518518e-3_f64) * t3447 * t15339;
    let t15347 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t14818;
    let t15348 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t14781;
    let t15349 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t14710;
    let t15357 = t11487 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t11211 - t11213 / F::cast_from(27.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t11215 + t11217 / F::cast_from(9.0_f64) - F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t14766 - t15347 + t15348 + t15349 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t14779 + t14790 / F::cast_from(3.0_f64) + t14784 / F::cast_from(9.0_f64) + t14787 / F::cast_from(18.0_f64) - t14799 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t14793 - t14796 / F::cast_from(3.0_f64) - t14713 / F::cast_from(6.0_f64);
    (t15339, t15341, t15347, t15348, t15349, t15357)
}
