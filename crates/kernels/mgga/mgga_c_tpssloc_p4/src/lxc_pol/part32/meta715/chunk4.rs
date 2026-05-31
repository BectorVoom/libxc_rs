//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2257/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2257<F: Float>(t16944: F, t221: F, t25154: F, t16851: F, t841: F, t87407: F, t81903: F, t87331: F, t87333: F, t87336: F, t87339: F, t87342: F, t87348: F, t87364: F, t87387: F, t87402: F, t92652: F, t98796: F, t98798: F, t98801: F, t98803: F, t98808: F) -> F {
    let t98811 = t25154 * t221 * t16944;
    let t98814 = t87407 * t841 * t16851;
    let t98816 = t87331 + t87333 - t87336 + t87339 + t87342 - t92652 - F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t98796 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t98798 - t87348 - F::cast_from(0.20186378047070195427e-3_f64) * t98801 - t98803 / F::cast_from(96.0_f64) - t87364 + F::cast_from(0.10093189023535097713e-3_f64) * t81903 - F::cast_from(0.63250651214153279005e-2_f64) * t87387 - t98808 / F::cast_from(4.0_f64) + t98811 / F::cast_from(8.0_f64) - F::cast_from(0.67826230238155856634e-1_f64) * t98814 - t87402;
    t98816
}
