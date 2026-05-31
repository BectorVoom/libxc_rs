//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2501/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2501<F: Float>(t67059: F, t18255: F, t51667: F, t18259: F, t50819: F, t22408: F, t3640: F, t1164: F, t15218: F, t18279: F, t18910: F, t18274: F, t51651: F) -> (F, F, F, F, F, F, F) {
    let t71090 = -t67059;
    let t71095 = F::cast_from(18.0_f64) * t51667 * t18255;
    let t71097 = F::cast_from(0.2894756309764656312e3_f64) * t50819 * t18259;
    let t71101 = t22408 * t3640;
    let t71106 = F::cast_from(0.31168546390226634765e3_f64) * t1164 * t18279 * t15218;
    let t71109 = F::cast_from(0.51947577317044391277e2_f64) * t1164 * t18910 * t15218;
    let t71112 = F::cast_from(0.30762056574649219974e4_f64) * t1164 * t18274 * t51651;
    (t71090, t71095, t71097, t71101, t71106, t71109, t71112)
}
