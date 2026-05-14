//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1305/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1305<F: Float>(t1729: F, t8547: F, t2715: F, t27754: F, t1464: F, t1726: F, t2785: F, t940: F, t2712: F, t9066: F, t2710: F, t19889: F, t19890: F, t219: F, t18170: F, t6171: F) -> (F, F, F, F, F, F, F) {
    let t64546 = t1729 * t8547;
    let t64548 = t64546 * t27754 * t2715;
    let t64550 = t1726 * t1464 * t2785;
    let t64557 = t64546 * t27754 * t940;
    let t64563 = t2712 * t9066;
    let t64565 = t1729 * t2710 * t64563 * t940;
    let t64573 = t940 * t19889;
    let t64590 = t19890 * t219;
    let t64613 = t6171 * t18170;
    (t64548, t64550, t64557, t64565, t64573, t64590, t64613)
}
