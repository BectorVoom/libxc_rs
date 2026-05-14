//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 855/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk855<F: Float>(t2144: F, t3351: F, t498: F, t7231: F, t9540: F, t3352: F, t9555: F, t1971: F, t7190: F, t9558: F, t7262: F, t9541: F, t15624: F, t515: F, t7230: F, t321: F) -> (F, F, F, F, F, F) {
    let t77732 = t3351 * t7231 * t2144 * t9540 * t498;
    let t77733 = 0.12769379967989351819e-4 * t77732;
    let t77736 = t3351 * t3352 * t2144 * t9555;
    let t77737 = 0.38308139903968055457e-4 * t77736;
    let t77740 = t3351 * t1971 * t7190 * t9558;
    let t77741 = 0.51077519871957407276e-4 * t77740;
    let t77744 = t3351 * t1971 * t7262 * t9541;
    let t77745 = 0.25538759935978703638e-4 * t77744;
    let t77749 = t7230 * t7231 * t515 * t15624 * t498;
    let t77750 = 0.53205749866622299248e-5 * t77749;
    let t77754 = t7230 * t3352 * t515 * t15624 * t321;
    (t77733, t77737, t77741, t77745, t77750, t77754)
}
