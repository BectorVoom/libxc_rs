//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1044/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1044<F: Float>(t24574: F, t32544: F, t85660: F, t8872: F, t225: F, t32452: F, t32422: F, t24826: F, t32466: F, t1089: F, t2144: F, t7327: F, t2121: F, t3427: F, t8891: F, t1170: F, t32469: F) -> (F, F, F, F, F, F, F, F) {
    let t118059 = t24574 * t32544;
    let t118067 = 0.36554090374405031922e-2 * t85660 * t8872;
    let t118084 = t32452 * t225;
    let t118097 = t32422 * t225;
    let t118111 = t24826 * t32466;
    let t118136 = t7327 * t2144 * t1089;
    let t118142 = 0.36554090374405031922e-2 * t2121 * t3427 * t8891;
    let t118157 = t2121 * t1170 * t32469;
    (t118059, t118067, t118084, t118097, t118111, t118136, t118142, t118157)
}
