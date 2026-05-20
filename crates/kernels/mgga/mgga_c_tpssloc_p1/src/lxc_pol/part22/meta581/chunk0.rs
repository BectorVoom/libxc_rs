//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2090/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2090<F: Float>(t11789: F, t820: F, t11737: F, t44857: F, t11647: F, t1203: F, t204: F, t486: F, t1213: F, t1216: F, t248: F, t11716: F, t44833: F, t44834: F) -> (F, F, F, F, F, F) {
    let t44951 = t820 * t11789;
    let t44965 = t44857 * t11737;
    let t45002 = t1203 * t11647;
    let t45017 = t204 * t486;
    let t45020 = t1213 * t248 * t45017 * t1216;
    let t45030 = t44833 * t11716 * t44834;
    (t44951, t44965, t45002, t45017, t45020, t45030)
}
