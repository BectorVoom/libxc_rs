//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2697/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2697<F: Float>(t1388: F, t6330: F, t6463: F, t1307: F, t15899: F, t20563: F, t3918: F, t3919: F, t39529: F, t39539: F, t39549: F, t5126: F, t5160: F, t5161: F, t74476: F, t74477: F, t74478: F, t74479: F) -> F {
    let t75203 = t6330 * t1388;
    let t75210 = t6463 * t1388;
    let t75214 = t6463 * t1307;
    let t75218 = F::new(6.0) * t15899 * t5160 * t75210 + F::new(18.0) * t20563 * t3919 * t5126 - F::new(9.0) * t3918 * t5161 * t75214 - F::new(18.0) * t5126 * t5161 * t75203 - t39529 + t39539 + t39549 - t74476 - t74477 - t74478 - t74479;
    t75218
}
