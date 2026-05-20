//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2585/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2585<F: Float>(t52367: F, t11549: F, t1174: F, t44586: F, t44589: F, t44592: F, t44595: F, t44602: F, t44628: F, t44631: F, t44635: F, t44638: F, t44641: F, t457: F, t460: F, t4889: F, t52327: F, t52345: F, t52355: F, t52357: F, t52362: F, t52364: F, t974: F) -> F {
    let t52368 = F::cast_from(0.18518518518518518518e-3_f64) * t52367;
    let t52374 = -F::cast_from(0.37037037037037037036e-3_f64) * t44586 + F::cast_from(0.27777777777777777777e-3_f64) * t44589 - F::cast_from(0.55555555555555555554e-3_f64) * t44592 + F::cast_from(0.37037037037037037036e-3_f64) * t44595 + F::cast_from(0.55555555555555555554e-3_f64) * t44602 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t974 * t457 * (t52327 + t52345) * t460 + t52355 - F::cast_from(0.27777777777777777777e-3_f64) * t52357 + F::cast_from(0.23045267489711934156e-2_f64) * t4889 * t11549 - F::cast_from(0.83333333333333333331e-3_f64) * t52362 - F::cast_from(0.98765432098765432097e-3_f64) * t52364 + t52368 - F::cast_from(0.83333333333333333331e-3_f64) * t44628 - F::cast_from(0.9259259259259259259e-4_f64) * t44631 - F::cast_from(0.3086419753086419753e-3_f64) * t44635 + F::cast_from(0.18518518518518518518e-3_f64) * t44638 + F::cast_from(0.37037037037037037036e-3_f64) * t44641;
    t52374
}
