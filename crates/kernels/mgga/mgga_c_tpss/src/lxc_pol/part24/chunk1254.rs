//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1254/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1254<F: Float>(t11460: F, t5620: F, t11687: F, t5610: F, t18107: F, t3941: F, t11548: F, t11691: F, t11701: F, t11584: F, t18069: F, t11506: F, t18094: F, t18110: F, t3916: F, t11640: F, t5605: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t64403 = 5.0 / 5184.0 * t5620 * t11460;
    let t64420 = t5610 * t11687;
    let t64427 = t18107 * t3941 / 216.0;
    let t64430 = t5620 * t11548 / 1728.0;
    let t64433 = t5620 * t11691;
    let t64436 = t5610 * t11701 / 1152.0;
    let t64447 = t18069 * t11584 / 1728.0;
    let t64455 = t18094 * t11506 / 576.0;
    let t64477 = t18110 * t3916 / 162.0;
    let t64478 = t5605 * t11640;
    (t64403, t64420, t64427, t64430, t64433, t64436, t64447, t64455, t64477, t64478)
}
