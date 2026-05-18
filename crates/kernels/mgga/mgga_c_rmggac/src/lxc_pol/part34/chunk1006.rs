//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1006/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1006<F: Float>(t1525: F, t1971: F, t515: F, t698: F, t7230: F, t15504: F, t16043: F, t637: F, t8625: F, t71163: F, t8631: F, t72142: F) -> (F, F, F, F) {
    let t77572 = t7230 * t1971 * t515 * t698 * t1525;
    let t77573 = F::new(0.53205749866622299248e-5) * t77572;
    let t77574 = t16043 * t15504;
    let t77575 = F::new(0.42564599893297839398e-5) * t77574;
    let t77576 = t637 * t8625;
    let t77577 = t71163 * t77576;
    let t77578 = F::new(0.40911992481368012592e-1) * t77577;
    let t77579 = t637 * t8631;
    let t77580 = t72142 * t77579;
    (t77573, t77575, t77578, t77580)
}
