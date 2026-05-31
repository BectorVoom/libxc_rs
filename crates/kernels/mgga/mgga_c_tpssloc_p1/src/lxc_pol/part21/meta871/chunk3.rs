//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3203/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3203<F: Float>(t11697: F, t18386: F, t3577: F, t15608: F, t15740: F, t225: F, t65165: F, t1174: F, t6183: F, t698: F, t11665: F, t1216: F, t15470: F, t15474: F, t15569: F, t15700: F, t18241: F, t18383: F, t18965: F, t3578: F, t45119: F, t45134: F, t45266: F, t45296: F, t484: F, t488: F, t4954: F, t5012: F, t52893: F, t52897: F, t53322: F, t53515: F, t53519: F, t64874: F, t68: F) -> (F, F) {
    let t66646 = t3577 * t11697 * t18386;
    let t66648 = t15740 * t15608;
    let t66662 = t65165 * t225;
    let t66668 = t1174 * t698 * t6183;
    let t66670 = t15569 * t15470 / F::cast_from(216.0_f64) + t15569 * t15474 / F::cast_from(432.0_f64) - t11665 * t18383 / F::cast_from(2304.0_f64) - t3577 * t3578 * t18241 * t1216 / F::cast_from(2304.0_f64) - t45266 / F::cast_from(6912.0_f64) + t53515 / F::cast_from(5184.0_f64) + t53519 / F::cast_from(5184.0_f64) - t66646 / F::cast_from(1728.0_f64) - t66648 / F::cast_from(1728.0_f64) - t52893 * t3578 * t64874 / F::cast_from(192.0_f64) - t53322 * t4954 / F::cast_from(1152.0_f64) + t45119 * t52897 * t15700 * t5012 / F::cast_from(768.0_f64) + t45134 * t18965 / F::cast_from(2304.0_f64) - t45296 / F::cast_from(7776.0_f64) + t66662 * t68 * t484 * t488 / F::cast_from(3072.0_f64) + t66668 / F::cast_from(1296.0_f64);
    (t66662, t66670)
}
