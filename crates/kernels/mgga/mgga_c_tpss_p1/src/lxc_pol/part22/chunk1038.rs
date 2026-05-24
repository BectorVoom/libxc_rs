//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1038/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1038<F: Float>(t11231: F, t3908: F, t912: F, t2596: F, t3907: F, t10954: F, t10956: F, t10963: F, t10965: F, t10968: F, t10970: F, t10972: F, t11103: F, t11123: F, t11146: F, t11149: F, t11155: F, t11160: F, t11211: F, t11215: F, t11218: F, t11221: F) -> (F, F, F) {
    let t11232 = t11231 * t3908;
    let t11234 = F::cast_from(0.34631718211362927518e2_f64) * t912 * t11232;
    let t11235 = t3907 * t2596;
    let t11237 = F::cast_from(0.35089341735807877242e1_f64) * t912 * t11235;
    let t11238 = t10954 - t10956 + t10963 + t10965 + t10968 + t10970 + t10972 + t11103 + t11123 - t11146 - t11149 + t11155 - t11160 - t11211 + t11215 - t11218 - t11221;
    (t11234, t11237, t11238)
}
