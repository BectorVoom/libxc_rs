//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1438/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1438<F: Float>(t11190: F, t6020: F, t6024: F, t1670: F, t21810: F, t3264: F, t3313: F, t71701: F, t11275: F, t18265: F, t6267: F, t15376: F, t15395: F, t18409: F, t18416: F, t18427: F, t18469: F, t22063: F, t22066: F, t3447: F, t4919: F, t52100: F, t64644: F, t73188: F, t73199: F, t73225: F, t73272: F, t73496: F, t78035: F) -> (F, F, F, F, F, F) {
    let t78364 = F::cast_from(0.57895126195293126241e3_f64) * t11190 * t6024 * t6020;
    let t78367 = F::new(8.0) * t3264 * t21810 * t1670;
    let t78370 = F::cast_from(0.64327917994770140268e2_f64) * t3313 * t71701 * t1670;
    let t78373 = F::cast_from(0.3103560775156404018e4_f64) * t11275 * t18265 * t6020;
    let t78379 = t6267 * t6267;
    let t78423 = -F::cast_from(0.59259259259259259256e-2_f64) * t73188 + F::cast_from(0.22222222222222222221e-2_f64) * t73199 + F::cast_from(0.66666666666666666664e-2_f64) * t3447 * t4919 * t73225 - F::cast_from(0.22222222222222222222e-2_f64) * t3447 * t64644 * t18469 + F::cast_from(0.16666666666666666666e-2_f64) * t3447 * t18416 * t18409 + F::cast_from(0.33333333333333333332e-2_f64) * t3447 * t18416 * t18427 - F::cast_from(0.11851851851851851852e-1_f64) * t15376 * t22063 + F::cast_from(0.11851851851851851852e-1_f64) * t15376 * t22066 - F::cast_from(0.51851851851851851851e-2_f64) * t3447 * t15395 * t78035 + F::cast_from(0.34567901234567901234e-2_f64) * t3447 * t52100 * t73496 - F::cast_from(0.39506172839506172838e-2_f64) * t73272;
    (t78364, t78367, t78370, t78373, t78379, t78423)
}
