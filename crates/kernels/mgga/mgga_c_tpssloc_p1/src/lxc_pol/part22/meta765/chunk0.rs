//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2584/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2584<F: Float>(t1653: F, t5011: F, t19080: F, t4997: F, t1215: F, t5398: F, t11668: F, t11678: F, t11692: F, t15569: F, t15594: F, t15659: F, t1735: F, t18236: F, t18395: F, t19016: F, t22185: F, t27524: F, t3490: F, t3577: F, t3578: F, t45119: F, t4723: F, t4729: F, t475: F, t52813: F, t5971: F, t6203: F, t6230: F, t6232: F, t65424: F, t65444: F, t66388: F) -> (F, F) {
    let t72146 = t1653 * t5011;
    let t72161 = t19080 * t4997;
    let t72164 = t5398 * t1215;
    let t72180 = t52813 * t6232 / F::new(192.0) + t65424 / F::new(1536.0) + F::new(5.0) / F::new(4608.0) * t15594 * t6203 + F::new(5.0) / F::new(2304.0) * t3490 * t22185 + t11692 * t3578 * t1735 * t72146 / F::new(768.0) - t45119 * t3578 * t66388 * t18395 / F::new(1536.0) + t11692 * t3578 * t6230 * t4729 / F::new(768.0) - F::new(5.0) / F::new(432.0) * t15569 * t19016 - t72161 / F::new(144.0) + t65444 / F::new(432.0) + F::new(5.0) / F::new(4608.0) * t3577 * t11668 * t4723 * t72164 * t475 - t3577 * t3578 * t18236 * t27524 * t475 / F::new(768.0) + F::new(5.0) / F::new(2304.0) * t11678 * t11668 * t15659 * t5971 * t1215;
    (t72146, t72180)
}
