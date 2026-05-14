//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 922/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk922<F: Float>(t21256: F, t21363: F, t300: F, t21348: F, t4483: F, t5804: F, t17954: F, t4475: F, t959: F, t4488: F, t5791: F, t1637: F, t5950: F, t11094: F, t17202: F, t193: F, t21093: F, t21097: F, t21099: F, t21103: F, t21105: F, t21107: F, t336: F, t4700: F) -> (F, F, F, F, F, F, F) {
    let t21365 = t300 * (t21256 + t21363);
    let t21367 = 0.19751673498613801407e-1 * t300 * t21348;
    let t21369 = 0.35089341735807877242e1 * t4483 * t5804;
    let t21370 = t17954 * t4475;
    let t21372 = 0.51947577317044391277e2 * t959 * t21370;
    let t21373 = t4488 * t5791;
    let t21375 = 0.35089341735807877242e1 * t959 * t21373;
    let t21376 = t5950 * t1637;
    let t21381 = 2.0 * t11094 * t193 * t21376 * t336 - 3.0 * t1637 * t17202 * t4700 - t21093 + t21097 - t21099 - t21103 - t21105 - t21107 + t21365 + t21367 + t21369 - t21372 + t21375;
    (t21365, t21367, t21369, t21372, t21375, t21376, t21381)
}
