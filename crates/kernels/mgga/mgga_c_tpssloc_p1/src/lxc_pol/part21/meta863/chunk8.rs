//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3149/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3149<F: Float>(t19211: F, t225: F, t3507: F, t6238: F, t11914: F, t1244: F, t1246: F, t14997: F, t15022: F, t15023: F, t15027: F, t15239: F, t15245: F, t15430: F, t15771: F, t15777: F, t1734: F, t1751: F, t1755: F, t19138: F, t19166: F, t19190: F, t3493: F, t3604: F, t3624: F, t3625: F, t45326: F, t475: F, t5064: F, t5072: F, t53592: F, t6252: F, t6260: F, t6739: F) -> (F, F, F) {
    let t65208 = t19211 * t225;
    let t65221 = t6238 * t3507;
    let t65249 = t11914 * t6252 * t6739 * t3493 * t475 + F::cast_from(2.0_f64) * t1244 * t1751 * t15239 * t1246 - t3624 * t65221 * t3625 - F::cast_from(2.0_f64) * t3624 * t1755 * t3625 * t15239 - F::cast_from(2.0_f64) * t15245 * t15023 + F::cast_from(8.0_f64) * t15027 * t14997 - t3624 * t6260 * t15022 + F::cast_from(2.0_f64) * t1244 * t15771 * t1734 * t1246 + F::cast_from(2.0_f64) * t3604 * t19190 + F::cast_from(4.0_f64) * t5064 * t15777 + F::cast_from(2.0_f64) * t53592 * t15430 - F::cast_from(4.0_f64) * t3624 * t5072 * t19138 + F::cast_from(12.0_f64) * t45326 * t19166;
    (t65208, t65221, t65249)
}
