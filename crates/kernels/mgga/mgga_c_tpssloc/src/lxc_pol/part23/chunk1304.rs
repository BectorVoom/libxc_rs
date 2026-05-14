//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1304/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1304<F: Float>(t1670: F, t3313: F, t71701: F, t11275: F, t18265: F, t6020: F, t6267: F, t15376: F, t15395: F, t18409: F, t18416: F, t18427: F, t18469: F, t22063: F, t22066: F, t3447: F, t4919: F, t52100: F, t64644: F, t73188: F, t73199: F, t73225: F, t73272: F, t73496: F, t78035: F) -> (F, F, F, F) {
    let t78370 = 0.64327917994770140268e2 * t3313 * t71701 * t1670;
    let t78373 = 0.3103560775156404018e4 * t11275 * t18265 * t6020;
    let t78379 = t6267 * t6267;
    let t78423 = -0.59259259259259259256e-2 * t73188 + 0.22222222222222222221e-2 * t73199 + 0.66666666666666666664e-2 * t3447 * t4919 * t73225 - 0.22222222222222222222e-2 * t3447 * t64644 * t18469 + 0.16666666666666666666e-2 * t3447 * t18416 * t18409 + 0.33333333333333333332e-2 * t3447 * t18416 * t18427 - 0.11851851851851851852e-1 * t15376 * t22063 + 0.11851851851851851852e-1 * t15376 * t22066 - 0.51851851851851851851e-2 * t3447 * t15395 * t78035 + 0.34567901234567901234e-2 * t3447 * t52100 * t73496 - 0.39506172839506172838e-2 * t73272;
    (t78370, t78373, t78379, t78423)
}
