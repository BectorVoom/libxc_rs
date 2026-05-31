//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2275/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2275<F: Float>(t97989: F, t98039: F, t98090: F, t99067: F, t16558: F, t3: F, t25365: F, t57911: F, t10143: F, t1484: F, t25374: F, t16596: F, t16944: F, t16949: F, t1877: F, t1915: F, t193: F, t202: F, t22959: F, t23290: F, t23295: F, t25013: F, t2522: F, t25354: F, t25358: F, t28248: F, t4255: F, t4314: F, t5544: F, t6666: F, t6670: F, t67128: F, t7541: F, t82312: F, t870: F, t97999: F, t98003: F, t98007: F, t98011: F, t99042: F) -> (F, F, F) {
    let t99069 = t97989 + t98039 + t98090 + t99067;
    let t99767 = t3 * t16558;
    let t100562 = t57911 * t25365;
    let t100572 = t10143 * t1484 * t25374;
    let t100578 = -F::cast_from(6.0_f64) * t1877 * t82312 * t97999 - F::cast_from(6.0_f64) * t2522 * t25358 * t16596 + F::cast_from(12.0_f64) * t4314 * t1915 * t16944 + F::cast_from(12.0_f64) * t4314 * t7541 * t4255 - F::cast_from(6.0_f64) * t2522 * t6670 * t98007 + F::cast_from(3.0_f64) * t2522 * t6666 * t5544 - F::cast_from(3.0_f64) * t2522 * t6670 * t98011 + F::cast_from(6.0_f64) * t2522 * t23295 * t98003 - F::cast_from(6.0_f64) * t2522 * t25358 * t25365 + F::cast_from(6.0_f64) * t2522 * t25354 * t1484 - F::cast_from(6.0_f64) * t4314 * t6670 * t67128 - F::cast_from(12.0_f64) * t25013 * t100562 + t193 * t202 * t99042 * t870 + F::cast_from(6.0_f64) * t4314 * t1915 * t16949 + F::cast_from(12.0_f64) * t22959 * t100572 - F::cast_from(6.0_f64) * t2522 * t23290 * t28248;
    (t99069, t99767, t100578)
}
