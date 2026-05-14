//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1258/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1258<F: Float>(t106895: F, t106899: F, t106901: F, t106905: F, t106919: F, t106961: F, t106964: F, t106968: F, t106974: F, t106978: F, t107492: F, t1774: F, t1849: F, t19451: F, t1976: F, t20293: F, t20296: F, t27993: F, t28020: F, t28852: F, t4028: F, t5457: F, t5493: F, t574: F, t652: F, t7458: F, t7472: F, t7670: F) -> (F,) {
    let t107493 = -6.0 * t5493 * t652 * t7670 + t106961 * t574 - 3.0 * t1774 * t27993 + 3.0 * t1849 * t28020 - 6.0 * t19451 * t7472 - t1976 * t20293 - 6.0 * t1976 * t20296 - 6.0 * t28852 * t4028 - 6.0 * t28852 * t7458 - 6.0 * t5457 * t7670 - t106895 - t106899 - t106901 - t106905 - t106919 - t106964 + t106968 - t106974 + t106978 + t107492;
    (t107493,)
}
