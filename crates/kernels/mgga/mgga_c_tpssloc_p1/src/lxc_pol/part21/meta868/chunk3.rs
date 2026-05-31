//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3179/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3179<F: Float>(t15568: F, t5064: F, t1174: F, t18206: F, t44562: F, t1227: F, t13969: F, t18958: F, t11665: F, t11668: F, t11678: F, t11692: F, t15569: F, t15591: F, t15714: F, t18342: F, t18387: F, t3490: F, t3494: F, t3509: F, t3516: F, t3577: F, t3578: F, t3580: F, t44621: F, t4950: F, t5014: F, t52751: F, t52758: F, t52773: F, t53322: F, t5971: F, t5975: F, t63420: F) -> F {
    let t65884 = t5064 * t15568;
    let t65914 = t1174 * t44562 * t18206;
    let t65920 = t1227 * t13969 * t18958;
    let t65925 = t65884 * t3580 / F::cast_from(216.0_f64) - t11678 * t3578 * t5975 * t3509 / F::cast_from(1152.0_f64) + t11692 * t3578 * t5975 * t3516 / F::cast_from(2304.0_f64) - t11665 * t18387 / F::cast_from(1152.0_f64) - t3577 * t3578 * t5975 * t3494 / F::cast_from(2304.0_f64) - F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t15569 * t15714 + t15591 * t5014 / F::cast_from(768.0_f64) - F::cast_from(7.0_f64) / F::cast_from(972.0_f64) * t52751 - t53322 * t4950 / F::cast_from(1152.0_f64) + t52758 / F::cast_from(5184.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t11678 * t11668 * t5971 * t3509 - F::cast_from(7.0_f64) / F::cast_from(972.0_f64) * t65914 + F::cast_from(35.0_f64) / F::cast_from(972.0_f64) * t1174 * t44621 * t63420 - t65920 / F::cast_from(1728.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t3490 * t18342 - t52773 / F::cast_from(216.0_f64);
    t65925
}
