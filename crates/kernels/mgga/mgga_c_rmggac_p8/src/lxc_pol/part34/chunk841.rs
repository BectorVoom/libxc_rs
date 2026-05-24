//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 841/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk841<F: Float>(t14286: F, t570: F, t262: F, t8620: F, t14125: F, t68871: F, t8456: F, t11670: F, t14236: F, t3369: F, t7834: F, t2144: F, t2816: F) -> (F, F, F, F, F, F) {
    let t75086 = t14286 * t570;
    let t75087 = t262 * t75086;
    let t75088 = t8620 * t75087;
    let t75092 = t68871 * t14125 * t8456;
    let t75096 = t14236 * t3369 * t7834 * t11670;
    let t75098 = t2144 * t2816;
    (t75086, t75087, t75088, t75092, t75096, t75098)
}
