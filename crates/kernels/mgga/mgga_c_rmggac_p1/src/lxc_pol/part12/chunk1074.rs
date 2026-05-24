//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1074/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1074<F: Float>(t2049: F, t35688: F, t39116: F, t7760: F, t1982: F, t7428: F, t8602: F, t8608: F, t1627: F, t2124: F, t36624: F, t36976: F, t36984: F, t4041: F, t42142: F, t42145: F, t42149: F, t42152: F, t42156: F, t42159: F, t42162: F, t42167: F, t42170: F, t4999: F, t530: F, t668: F, t72: F, t8824: F, t903: F) -> F {
    let t42174 = t35688 * t2049 * t39116 * t7760;
    let t42177 = t8602 * t7428 * t1982;
    let t42178 = F::cast_from(0.19863479950205658386e-4_f64) * t42177;
    let t42180 = t8608 * t7428 * t1982;
    let t42181 = F::cast_from(0.19863479950205658386e-4_f64) * t42180;
    let t42186 = -F::cast_from(0.11971293719990017331e-4_f64) * t42142 - t42145 + F::cast_from(0.15961724959986689774e-4_f64) * t42149 - t42152 + F::cast_from(0.35922725105591425692e0_f64) * t903 * t2124 * t1627 - F::cast_from(0.14967802127329760705e-1_f64) * t42156 + F::cast_from(0.21819729323396273384e0_f64) * t36976 - t36984 - F::cast_from(0.17961362552795712846e0_f64) * t42159 - F::cast_from(0.5987120850931904282e-1_f64) * t42162 - F::new(0.2363e1) * t530 * t36624 - t42167 - F::cast_from(0.72042316457491791906e-3_f64) * t42170 + F::cast_from(0.10248087766267884742e-3_f64) * t42174 - t42178 - t42181 + t72 * t4999 * t668 + F::cast_from(0.11974241701863808564e0_f64) * t4041 * t8824;
    t42186
}
