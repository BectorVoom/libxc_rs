//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2463/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2463<F: Float>(t1174: F, t3477: F, t698: F, t43776: F, t1186: F, t2402: F, t457: F, t625: F, t221: F, t456: F, t461: F, t1184: F, t15394: F) -> (F, F, F, F, F, F) {
    let t44439 = t1174 * t698 * t3477;
    let t44466 = F::new(220.0) / F::new(81.0) * t43776;
    let t44478 = t1174 * t2402 * t1186;
    let t44483 = t625 * t457;
    let t44487 = F::cast_from(0.82304526748971193413e-3_f64) * t456 * t221 * t44483 * t461;
    let t44504 = t15394 * t1184;
    (t44439, t44466, t44478, t44483, t44487, t44504)
}
