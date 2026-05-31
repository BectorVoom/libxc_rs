//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1317/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1317<F: Float>(t20974: F, t23146: F, t105353: F, t105366: F, t105370: F, t105372: F, t105376: F, t87304: F, t87306: F, t87345: F, t98733: F, t98736: F, t98738: F, t98746: F, t98750: F, t98774: F, t98782: F, t98787: F, t98791: F, t98796: F, t98798: F) -> F {
    let t105381 = t23146 * t20974;
    let t105383 = -F::cast_from(0.20186378047070195427e-3_f64) * t105353 + F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t98733 - F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t87304 - F::cast_from(0.2034786907144675699e0_f64) * t87306 + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t98736 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t98738 + F::cast_from(0.42391393898847410397e-2_f64) * t98746 - F::cast_from(0.12111826828242117256e-2_f64) * t98750 - F::cast_from(0.42391393898847410397e-2_f64) * t98774 - F::cast_from(0.20186378047070195427e-3_f64) * t98782 + F::cast_from(0.10093189023535097714e-3_f64) * t98787 + F::cast_from(0.10093189023535097714e-3_f64) * t98791 - t105366 / F::cast_from(4.0_f64) - F::cast_from(0.67826230238155856634e-1_f64) * t105370 - t105372 / F::cast_from(48.0_f64) - F::cast_from(0.12111826828242117256e-2_f64) * t105376 - F::cast_from(119.0_f64) / F::cast_from(576.0_f64) * t87345 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t98796 + F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t98798 - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t105381;
    t105383
}
