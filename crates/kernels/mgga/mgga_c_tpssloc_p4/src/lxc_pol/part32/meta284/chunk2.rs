//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1288/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1288<F: Float>(t1458: F, t7266: F, t7675: F, t7678: F, t7680: F, t7983: F, t113: F, t1442: F, t1459: F, t1774: F, t1849: F, t2114: F, t2165: F, t2167: F, t510: F, t574: F, t652: F, t7457: F, t7460: F, t7463: F, t7470: F, t7686: F, t7690: F, t7755: F, t7757: F, t7989: F, t8103: F) -> (F, F) {
    let t8107 = F::cast_from(2.0_f64) * t1458 * t7266 + t7675 + t7678 + t7680 + t7983;
    let t8110 = -t113 * t8103 - t1442 * t2165 - F::cast_from(2.0_f64) * t1459 * t7266 - t1774 * t2114 + t1849 * t2167 - t510 * t7983 + t574 * t8107 - F::cast_from(2.0_f64) * t652 * t7989 - t7457 - t7460 - t7463 - t7470 + t7686 + t7690 + t7755 - t7757;
    (t8107, t8110)
}
