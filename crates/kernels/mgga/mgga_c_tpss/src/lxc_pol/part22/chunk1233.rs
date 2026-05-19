//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1233/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1233<F: Float>(t18704: F, t19009: F, t3: F, t1799: F, t2061: F, t116: F, t5815: F, t645: F, t2105: F, t5953: F, t117: F, t18627: F, param_d: F) -> (F, F, F, F, F, F, F, F) {
    let t19010 = t18704 + t19009;
    let t19011 = t3 * t19010;
    let t19023 = param_d * t19010;
    let t19037 = t2061 * t1799;
    let t19040 = t116 * t5815;
    let t19041 = t19040 * t645;
    let t19044 = t5953 * t2105;
    let t19047 = t117 * t18627;
    (t19010, t19011, t19023, t19037, t19040, t19041, t19044, t19047)
}
