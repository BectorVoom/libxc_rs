//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2590/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2590<F: Float>(t19026: F, t4997: F, t18975: F, t5005: F, t11719: F, t22307: F, t248: F, t3570: F, t11668: F, t1213: F, t1214: F, t1737: F, t19002: F, t3577: F, t4724: F, t475: F, t52879: F, t6219: F, t65479: F, t65482: F, t65485: F, t65506: F, t65957: F, t72181: F, t72183: F, t72217: F) -> F {
    let t72223 = t19026 * t4997;
    let t72225 = t5005 * t18975;
    let t72229 = t11719 * t248 * t3570 * t22307;
    let t72233 = t72181 / F::new(1536.0) - t72183 / F::new(2304.0) + t65957 * t1737 / F::new(1024.0) - t65479 / F::new(1152.0) + F::new(5.0) / F::new(4608.0) * t3577 * t11668 * t6219 * t4724 + t65482 / F::new(1152.0) - t65485 / F::new(576.0) - t65506 / F::new(576.0) + t1213 * t248 * t1214 * t72217 * t475 / F::new(3072.0) + F::new(19.0) / F::new(864.0) * t72223 + F::new(5.0) / F::new(6912.0) * t72225 + t72229 / F::new(768.0) - t52879 * t19002 / F::new(384.0);
    t72233
}
