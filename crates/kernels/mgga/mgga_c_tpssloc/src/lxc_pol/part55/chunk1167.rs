//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1167/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1167<F: Float>(t24574: F, t32511: F, t32544: F, t85660: F, t8872: F, t225: F, t32452: F, t32422: F, t24826: F, t32466: F, t1089: F, t2144: F, t7327: F) -> (F, F, F, F, F, F, F) {
    let t118052 = t24574 * t32511;
    let t118059 = t24574 * t32544;
    let t118067 = F::cast_from(0.36554090374405031922e-2_f64) * t85660 * t8872;
    let t118084 = t32452 * t225;
    let t118097 = t32422 * t225;
    let t118111 = t24826 * t32466;
    let t118136 = t7327 * t2144 * t1089;
    (t118052, t118059, t118067, t118084, t118097, t118111, t118136)
}
