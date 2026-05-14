//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1173/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1173<F: Float>(t20565: F, t3866: F, t1827: F, t57056: F, t20492: F, t39944: F, t16288: F, t6417: F, t12385: F, t20497: F, t20433: F, t16336: F, t6431: F, t1831: F, t57021: F, t53945: F, t6396: F) -> (F, F, F, F, F, F, F, F, F) {
    let t74191 = t3866 * t20565;
    let t74212 = t57056 * t1827;
    let t74214 = t39944 * t20492;
    let t74217 = t16288 * t6417;
    let t74228 = t12385 * t20497;
    let t74256 = t3866 * t20433;
    let t74258 = t16336 * t6431;
    let t74260 = t57021 * t1831;
    let t74274 = t53945 * t6396;
    (t74191, t74212, t74214, t74217, t74228, t74256, t74258, t74260, t74274)
}
