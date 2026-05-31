//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2353/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2353<F: Float>(t1395: F, t8119: F, t1858: F, t7415: F, t27930: F, t576: F, t112: F, t27907: F, t2169: F, t2319: F, t1458: F, t16538: F, t2363: F, t24969: F, t24972: F, t27921: F, t4072: F, t577: F, t671: F, t85423: F, t86582: F, t86606: F, t86610: F, t86612: F, t86614: F, t86616: F, t86619: F, t86622: F, t86625: F, t86629: F, t96277: F) -> (F, F, F, F) {
    let t96300 = F::cast_from(2.0_f64) * t1395 * t8119;
    let t96303 = F::cast_from(2.0_f64) * t7415 * t1858;
    let t96308 = F::cast_from(2.0_f64) * t576 * t27930;
    let t96311 = t27907 * t112;
    let t96316 = t2169 * t2319;
    let t96327 = F::cast_from(27.0_f64) * t96311 * t671 + F::cast_from(0.135e2_f64) * t27921 * t2363 + F::cast_from(27.0_f64) * t96316 * t1458 + F::cast_from(0.45e1_f64) * t96277 * t577 + t86582 + t86606 + F::cast_from(0.135e2_f64) * t85423 * t1458 + F::cast_from(27.0_f64) * t24969 * t4072 + t86610 + t86612 + t86614 + t86616 + t86619 + t86622 + t86625 + t86629 + F::cast_from(54.0_f64) * t24972 * t16538;
    (t96300, t96303, t96308, t96327)
}
