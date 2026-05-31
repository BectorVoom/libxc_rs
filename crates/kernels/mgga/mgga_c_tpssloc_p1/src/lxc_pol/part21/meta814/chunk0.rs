//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2869/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2869<F: Float>(t13655: F, t4354: F, t41811: F, t5695: F, t4471: F, t17488: F, t892: F, t914: F, t10771: F, t10811: F, t14271: F, t14328: F, t14460: F, t14466: F, t17547: F, t17554: F, t2861: F, t2862: F, t2880: F, t2886: F, t2905: F, t42154: F, t42226: F, t42228: F, t4437: F, t49263: F, t5742: F, t5759: F, t59941: F, t59958: F, t59961: F, t59962: F, t59966: F, t59968: F, t933: F, t951: F) -> (F, F, F, F, F) {
    let t59970 = F::cast_from(2.0_f64) * t4354 * t13655;
    let t59972 = F::cast_from(2.0_f64) * t41811 * t5695;
    let t59975 = t4471 * t4471;
    let t59979 = t17488 * t892;
    let t59981 = F::cast_from(2.0_f64) * t59979 * t914;
    let t59982 = -F::cast_from(0.24828486201251232145e5_f64) * t42154 * t17554 * t2862 - F::cast_from(2.0_f64) * t2861 * t5759 * t2880 - F::cast_from(0.19298375398431042081e3_f64) * t10771 * t17547 * t2862 + F::cast_from(0.32163958997385070134e2_f64) * t2886 * t17547 * t2880 + F::cast_from(0.2069040516770936012e4_f64) * t10811 * t59941 * t2862 + F::cast_from(0.64327917994770140268e2_f64) * t2886 * t4437 * t14328 + F::cast_from(0.2069040516770936012e4_f64) * t10811 * t17554 * t2880 + F::cast_from(0.19964560303604640732e6_f64) * t42226 * t5742 * t42228 * t2862 - F::cast_from(0.4155806185363551302e3_f64) * t49263 * t14460 - t59958 - t59961 + F::cast_from(2.0_f64) * t59962 * t933 - t59966 - t59968 - t59970 + t59972 + F::cast_from(12.0_f64) * t14271 * t14466 - F::cast_from(0.23392894490538584828e1_f64) * t2905 * t59975 * t951 - t59981;
    (t59970, t59972, t59975, t59981, t59982)
}
