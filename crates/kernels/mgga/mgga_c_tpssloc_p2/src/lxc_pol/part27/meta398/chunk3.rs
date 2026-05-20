//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1645/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1645<F: Float>(t15501: F, t3503: F, t3500: F, t1210: F, t11665: F, t1218: F, t1232: F, t15470: F, t15474: F, t15478: F, t15484: F, t15488: F, t15490: F, t15494: F, t15495: F, t15498: F, t3511: F, t3518: F, t3527: F, t3577: F, t3587: F, t4954: F, t5005: F, t5024: F) -> F {
    let t15502 = t3503 * t15501;
    let t15503 = t3500 * t15502;
    let t15506 = t1210 * t15501;
    let t15507 = t3500 * t15506;
    let t15512 = -t11665 * t4954 / F::new(2304.0) - t3577 * t15470 / F::new(2304.0) - t3577 * t15474 / F::new(4608.0) - t3577 * t15478 / F::new(2304.0) + F::new(5.0) / F::new(13824.0) * t5005 * t3587 - t15484 - t15488 + t15490 + t15494 - t15495 * t1218 / F::new(288.0) + t15498 * t1232 / F::new(432.0) - t15503 * t3511 / F::new(288.0) + t15507 * t3518 / F::new(576.0) + t5024 * t3527 / F::new(864.0);
    t15512
}
