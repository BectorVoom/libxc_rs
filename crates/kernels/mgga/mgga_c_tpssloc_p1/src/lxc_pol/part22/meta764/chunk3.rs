//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2583/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2583<F: Float>(t1155: F, t1164: F, t21906: F, t43689: F, t43692: F, t18276: F, t4869: F, t1238: F, t1251: F, t14972: F, t1751: F, t1761: F, t18571: F, t19209: F, t19219: F, t19234: F, t19249: F, t22004: F, t22393: F, t27784: F, t3487: F, t3598: F, t4940: F, t4945: F, t498: F, t5060: F, t5089: F, t53677: F, t6238: F, t6268: F, t64595: F, t65203: F, t66845: F, t66860: F) -> (F, F, F) {
    let t72104 = F::cast_from(0.91082604192152556044e5_f64) * t1164 * t43689 * t21906 * t43692 * t1155;
    let t72106 = F::cast_from(0.30762056574649219972e4_f64) * t4869 * t18276;
    let t72138 = F::cast_from(2.0_f64) * t1238 * t1251 * t22393 * t3598 + F::cast_from(3.0_f64) * t1751 * t18571 * t498 - F::cast_from(18.0_f64) * t19219 * t27784 * t53677 + F::cast_from(3.0_f64) * t4940 * t498 * t6238 - F::cast_from(3.0_f64) * t14972 * t6268 - F::cast_from(3.0_f64) * t1761 * t64595 - F::cast_from(6.0_f64) * t1761 * t65203 - F::cast_from(3.0_f64) * t1761 * t66845 - F::cast_from(3.0_f64) * t1761 * t66860 - F::cast_from(3.0_f64) * t19209 * t4945 - F::cast_from(6.0_f64) * t19234 * t5089 + F::cast_from(6.0_f64) * t19249 * t5060 + F::cast_from(6.0_f64) * t22004 * t3487;
    (t72104, t72106, t72138)
}
