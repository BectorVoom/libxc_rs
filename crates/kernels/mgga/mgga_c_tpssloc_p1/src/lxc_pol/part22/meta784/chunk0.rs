//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2691/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2691<F: Float>(t20670: F, t225: F, t1834: F, t6414: F, t6387: F, t20553: F, t562: F, t20489: F, t16036: F, t16047: F, t16055: F, t1825: F, t19654: F, t19661: F, t19735: F, t19743: F, t19744: F, t19810: F, t20018: F, t20473: F, t20638: F, t5250: F, t5287: F, t5333: F, t5334: F, t5336: F, t5344: F, t54963: F, t57704: F, t6378: F, t74599: F) -> (F, F, F, F, F, F) {
    let t74930 = t20670 * t225;
    let t74937 = t1834 * t6414;
    let t74941 = t1834 * t6387;
    let t74949 = t562 * t20553;
    let t74967 = t562 * t20489;
    let t74996 = F::new(6.0) * t16036 * t20473 * t5334 - F::new(36.0) * t16047 * t19744 * t74967 - F::new(3.0) * t1825 * t5344 * t57704 + F::new(18.0) * t19735 * t19743 * t5334 - F::new(3.0) * t19743 * t5287 * t5344 + F::new(14.0) * t5250 * t5334 * t74967 + F::new(6.0) * t5333 * t5336 * t6378 + F::new(24.0) * t54963 * t74599 * t74967 + F::new(6.0) * t16055 * t20638 + F::new(6.0) * t19654 * t19661 - F::new(6.0) * t19810 * t20018;
    (t74930, t74937, t74941, t74949, t74967, t74996)
}
