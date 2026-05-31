//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2164/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2164<F: Float>(t1022: F, t10482: F, t17670: F, t4582: F, t1539: F, t4650: F, t3071: F, t5867: F, t884: F, t10390: F, t1041: F, t10480: F, t10904: F, t13995: F, t14000: F, t14027: F, t17643: F, t17649: F, t17656: F, t17660: F, t17662: F, t17668: F, t3070: F, t4575: F, t5875: F, t5909: F) -> (F, F, F, F, F, F, F, F) {
    let t17671 = t10482 * t1022;
    let t17672 = t17670 * t17671;
    let t17673 = t4582 * t17672;
    let t17676 = t4650 * t1539;
    let t17677 = t3071 * t17676;
    let t17680 = t5867 * t884;
    let t17681 = t3071 * t17680;
    let t17684 = F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t1041 * t17643 + t13995 * t4575 / F::cast_from(2304.0_f64) - t3070 * t17649 / F::cast_from(2304.0_f64) + t10390 * t5909 / F::cast_from(2304.0_f64) - t17656 / F::cast_from(4608.0_f64) + t17660 / F::cast_from(6912.0_f64) + t17662 / F::cast_from(2304.0_f64) - t10904 * t5875 / F::cast_from(288.0_f64) + t17668 / F::cast_from(2304.0_f64) + t14000 + t10480 * t17673 / F::cast_from(512.0_f64) + t14027 + t3070 * t17677 / F::cast_from(2304.0_f64) + t3070 * t17681 / F::cast_from(4608.0_f64);
    (t17671, t17672, t17673, t17676, t17677, t17680, t17681, t17684)
}
