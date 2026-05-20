//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2581/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2581<F: Float>(t18911: F, t4869: F, t18918: F, t1164: F, t4858: F, t6105: F, t1147: F, t1156: F, t71530: F, t22229: F, t3411: F, t22233: F) -> (F, F, F, F, F, F) {
    let t72081 = F::cast_from(0.51947577317044391276e2_f64) * t4869 * t18911;
    let t72083 = F::cast_from(0.70178683471615754484e1_f64) * t4869 * t18918;
    let t72086 = F::cast_from(0.10526802520742363173e2_f64) * t1164 * t6105 * t4858;
    let t72094 = F::cast_from(0.5848223622634646207e0_f64) * t1164 * t1147 * t71530 * t1156;
    let t72096 = F::cast_from(0.10389515463408878255e3_f64) * t3411 * t22229;
    let t72098 = F::cast_from(0.5848223622634646207e0_f64) * t3411 * t22233;
    (t72081, t72083, t72086, t72094, t72096, t72098)
}
