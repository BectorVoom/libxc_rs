//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2606/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2606<F: Float>(t14725: F, t17635: F, t18329: F, t4889: F, t18324: F, t1174: F, t135: F, t22136: F, t18346: F, t18580: F, t19019: F, t3440: F, t45128: F, t5024: F, t52873: F, t52893: F, t66001: F, t66015: F, t66024: F, t66027: F, t71138: F) -> (F, F) {
    let t72688 = t14725 * t17635;
    let t72703 = t4889 * t18329;
    let t72705 = t4889 * t18324;
    let t72708 = t1174 * t135 * t22136;
    let t72712 = -F::cast_from(5.0_f64) / F::cast_from(1728.0_f64) * t52893 * t45128 * t72688 - t66001 / F::cast_from(144.0_f64) - t4889 * t19019 / F::cast_from(27.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4889 * t18580 + t1174 * t3440 * t71138 / F::cast_from(216.0_f64) + t66015 / F::cast_from(216.0_f64) + t52873 + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t66024 + F::cast_from(5.0_f64) / F::cast_from(1152.0_f64) * t66027 + t72703 / F::cast_from(108.0_f64) + t72705 / F::cast_from(54.0_f64) + t72708 / F::cast_from(108.0_f64) - F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t5024 * t18346;
    (t72688, t72712)
}
