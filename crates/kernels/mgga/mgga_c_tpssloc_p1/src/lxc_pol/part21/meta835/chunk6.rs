//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2968/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2968<F: Float>(t13969: F, t17713: F, t3130: F, t4649: F, t884: F, t1023: F, t10390: F, t10403: F, t10408: F, t1041: F, t14211: F, t17187: F, t17688: F, t17972: F, t18021: F, t3048: F, t3070: F, t3071: F, t3121: F, t3132: F, t4579: F, t4582: F, t47775: F, t48626: F, t48629: F, t48670: F, t48674: F, t50324: F, t5677: F, t61853: F, t61855: F) -> (F, F) {
    let t61866 = t3130 * t13969 * t17713;
    let t61871 = t884 * t4649;
    let t61876 = t50324 * t4579 / F::cast_from(1152.0_f64) + t10390 * t18021 / F::cast_from(2304.0_f64) + t3070 * t3071 * t17187 * t1023 / F::cast_from(2304.0_f64) + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t3070 * t10408 * t5677 * t3121 + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t10403 * t10408 * t5677 * t3132 + t61853 / F::cast_from(576.0_f64) - t1041 * t4582 * t47775 * t61855 / F::cast_from(192.0_f64) + F::cast_from(5.0_f64) / F::cast_from(216.0_f64) * t3048 * t17688 - t48626 / F::cast_from(864.0_f64) + F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t48629 + t48670 / F::cast_from(5184.0_f64) + t61866 / F::cast_from(1152.0_f64) - t3048 * t17972 / F::cast_from(72.0_f64) + t48674 / F::cast_from(7776.0_f64) + t10403 * t3071 * t14211 * t61871 / F::cast_from(576.0_f64);
    (t61871, t61876)
}
