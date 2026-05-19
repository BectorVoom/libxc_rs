//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1181/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1181<F: Float>(t31611: F, t6907: F, t1985: F, t6883: F, t8631: F, t2085: F, t552: F, t1307: F, t6637: F, t6888: F, t794: F, t8630: F) -> (F, F, F, F, F, F, F, F) {
    let t31612 = t31611 * t6907;
    let t31613 = t1985 * t31612;
    let t31616 = t6883 * t8631;
    let t31617 = F::cast_from(0.19190897446562641759e-1_f64) * t31616;
    let t31618 = t552 * t2085;
    let t31619 = t31618 * t1307;
    let t31620 = t6637 * t31619;
    let t31621 = t6888 * t31620;
    let t31623 = t794 * t8630;
    (t31612, t31613, t31617, t31618, t31619, t31620, t31621, t31623)
}
