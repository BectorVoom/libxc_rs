//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1250/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1250<F: Float>(t77220: F, t942: F, t951: F, t959: F, t13520: F, t21253: F, t10661: F, t76644: F, t913: F, t2792: F, t76998: F, t10632: F, t41825: F, t76637: F, t5742: F, t48103: F, t68442: F, t68444: F, t68446: F, t68448: F, t68452: F, t68454: F, t68494: F, t68498: F, t68500: F, t77028: F, t77030: F, t77032: F, t77034: F) -> (F, F, F, F, F, F, F) {
    let t77224 = 0.5848223622634646207e0 * t959 * t942 * t77220 * t951;
    let t77226 = 24.0 * t13520 * t21253;
    let t77229 = 24.0 * t10661 * t76644 * t913;
    let t77232 = 6.0 * t2792 * t76998 * t913;
    let t77236 = 0.12304822629859687989e5 * t959 * t41825 * t76637 * t10632;
    let t77239 = t5742 * t5742;
    let t77257 = 0.41318e1 * t68442 + 0.68863333333333333332e0 * t68444 + 0.76514814814814814814e0 * t68446 - 0.27545333333333333332e1 * t68448 - 0.166712e1 * t68452 + 0.27785333333333333333e0 * t68454 + 0.12349037037037037037e1 * t48103 + 0.13772666666666666667e1 * t68494 - 0.41318e1 * t68498 + 0.158837625e2 * t77028 - 0.705945e1 * t77030 - 0.94674375e0 * t77032 + 0.1262325e1 * t77034 + 0.12349037037037037037e0 * t68500;
    (t77224, t77226, t77229, t77232, t77236, t77239, t77257)
}
