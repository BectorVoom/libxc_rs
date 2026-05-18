//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1246/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1246<F: Float>(t15806: F, t15833: F, t11947: F, t1763: F, t1256: F, t14963: F, t14969: F, t14971: F, t15038: F, t15040: F, t15043: F, t15046: F, t15048: F, t15050: F, t15053: F, t15056: F, t15059: F, t15063: F, t15066: F, t15070: F, t15235: F, t15237: F, t193: F, t336: F, t3633: F, t3637: F, t4700: F, t5095: F) -> F {
    let t15834 = t15806 + t15833;
    let t15838 = t1763 * t11947;
    let t15842 = t1256 * t15834 * t193 * t336 + F::new(2.0) * t15838 * t3637 * t4700 - t3633 * t4700 * t5095 + t14963 - t14969 - t14971 - t15038 - t15040 - t15043 + t15046 - t15048 + t15050 - t15053 - t15056 - t15059 + t15063 + t15066 + t15070 + t15235 + t15237;
    t15842
}
