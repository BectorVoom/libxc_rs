//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1334/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1334<F: Float>(t81954: F, t244: F, t6546: F, t2606: F, t1878: F, t845: F, t10033: F, t2230: F, t23076: F, t213: F, t23080: F, t200: F, t23075: F, t598: F) -> (F, F, F, F, F) {
    let t81955 = F::cast_from(0.33643963411783659044e-4_f64) * t81954;
    let t81956 = t6546 * t244;
    let t81957 = t81956 * t2606;
    let t81959 = t1878 * t845;
    let t81960 = t81959 * t10033;
    let t81962 = t2230 * t23076;
    let t81963 = t81962 * t213;
    let t81964 = t81963 * t23080;
    let t81968 = t598 / t23075 / t200;
    (t81955, t81957, t81960, t81964, t81968)
}
