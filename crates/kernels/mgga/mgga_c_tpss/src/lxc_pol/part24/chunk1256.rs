//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1256/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1256<F: Float>(t19889: F, t940: F, t19890: F, t219: F, t18170: F, t6171: F, t19898: F, t5638: F, t1482: F, t18178: F, t4016: F, t5640: F, t2715: F, t6167: F, t19956: F, t2814: F) -> (F, F, F, F, F, F, F, F) {
    let t64573 = t940 * t19889;
    let t64590 = t19890 * t219;
    let t64613 = t6171 * t18170;
    let t64645 = t19898 * t5638;
    let t64690 = t18178 * t1482;
    let t64694 = t5640 * t4016;
    let t64714 = t2715 * t6167;
    let t64731 = t19956 * t2814;
    (t64573, t64590, t64613, t64645, t64690, t64694, t64714, t64731)
}
