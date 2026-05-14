//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 570/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk570<F: Float>(t2675: F, t949: F, t242: F, t946: F, t943: F, t956: F, t938: F, t941: F, t589: F, t924: F, t140: F, t930: F, t925: F, t265: F, t836: F, t2459: F) -> (F, F, F, F, F, F) {
    let t2676 = t2675 * t949;
    let t2677 = t242 * t2676;
    let t2678 = t946 * t2677;
    let t2680 = t956 * t943;
    let t2682 = t938 * t941 * t2680;
    let t2685 = t589 * t924;
    let t2689 = t140 * t930;
    let t2690 = t925 * t2689;
    let t2697 = 1.0 / t265 / t836;
    let t2698 = t2697 * t2459;
    (t2678, t2682, t2685, t2690, t2697, t2698)
}
