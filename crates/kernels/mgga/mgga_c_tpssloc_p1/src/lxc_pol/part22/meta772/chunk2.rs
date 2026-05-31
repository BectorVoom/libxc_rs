//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2634/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2634<F: Float>(t225: F, t73575: F, t11665: F, t11668: F, t11678: F, t1215: F, t15659: F, t19083: F, t22162: F, t3577: F, t3578: F, t45296: F, t484: F, t488: F, t4965: F, t4974: F, t5012: F, t52893: F, t53516: F, t53520: F, t5975: F, t5979: F, t6164: F, t66646: F, t66648: F, t66668: F, t68: F, t73138: F, t73142: F) -> (F, F) {
    let t73576 = t73575 * t225;
    let t73587 = -t11678 * t3578 * t15659 * t5975 * t1215 / F::cast_from(384.0_f64) - t3577 * t3578 * t5012 * t5979 / F::cast_from(1536.0_f64) + t19083 * t4974 / F::cast_from(72.0_f64) + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t52893 * t11668 * t73138 - F::cast_from(77.0_f64) / F::cast_from(486.0_f64) * t73142 + F::cast_from(19.0_f64) / F::cast_from(576.0_f64) * t4965 * t6164 * t488 + t73576 * t68 * t484 * t488 / F::cast_from(3072.0_f64) + t53516 + t53520 - t66646 / F::cast_from(1152.0_f64) - t66648 / F::cast_from(1152.0_f64) - t11665 * t22162 / F::cast_from(1536.0_f64) - t45296 / F::cast_from(15552.0_f64) + t66668 / F::cast_from(432.0_f64);
    (t73576, t73587)
}
