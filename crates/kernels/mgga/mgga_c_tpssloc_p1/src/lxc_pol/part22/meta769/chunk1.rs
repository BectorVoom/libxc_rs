//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2611/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2611<F: Float>(t1213: F, t22244: F, t248: F, t3570: F, t1227: F, t21758: F, t45268: F, t11692: F, t11697: F, t22283: F, t1216: F, t15498: F, t15569: F, t1653: F, t18360: F, t18584: F, t18941: F, t19056: F, t22301: F, t22309: F, t22314: F, t3515: F, t3577: F, t3578: F, t44858: F, t44896: F, t44965: F, t45119: F, t4582: F, t5012: F, t52897: F, t53000: F, t6203: F, t72767: F) -> F {
    let t72849 = t1213 * t248 * t3570 * t22244;
    let t72857 = t1227 * t248 * t45268 * t21758;
    let t72864 = t11692 * t11697 * t22283;
    let t72878 = t53000 - F::cast_from(5.0_f64) / F::cast_from(864.0_f64) * t15498 * t6203 + t44896 * t22309 / F::cast_from(512.0_f64) + t72849 / F::cast_from(4608.0_f64) - t44858 * t22314 / F::cast_from(512.0_f64) + t44965 * t22301 / F::cast_from(3072.0_f64) - F::cast_from(5.0_f64) / F::cast_from(7776.0_f64) * t72857 - t3577 * t3578 * t18941 * t1653 / F::cast_from(1536.0_f64) + t72864 / F::cast_from(2304.0_f64) + t15569 * t18584 / F::cast_from(144.0_f64) + t15569 * t18360 / F::cast_from(144.0_f64) + t45119 * t52897 * t72767 * t1216 / F::cast_from(1024.0_f64) - t3515 * t4582 * t19056 * t5012 / F::cast_from(1024.0_f64);
    t72878
}
