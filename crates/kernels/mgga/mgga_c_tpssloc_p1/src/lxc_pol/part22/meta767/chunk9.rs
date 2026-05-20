//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2601/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2601<F: Float>(t11697: F, t22287: F, t3577: F, t15569: F, t18371: F, t1090: F, t1216: F, t15737: F, t18303: F, t18307: F, t18364: F, t18948: F, t19051: F, t21769: F, t22244: F, t3578: F, t4950: F, t4954: F, t4989: F, t52680: F, t53083: F, t53336: F, t65803: F, t66622: F) -> F {
    let t72530 = t3577 * t11697 * t22287;
    let t72542 = t15569 * t18371;
    let t72552 = -F::new(5.0) / F::new(864.0) * t15569 * t18364 - t3577 * t3578 * t21769 * t1216 / F::new(768.0) - t72530 / F::new(1152.0) - t53336 * t18303 / F::new(32.0) + t53083 * t18307 / F::new(32.0) + t65803 / F::new(108.0) - t52680 / F::new(5184.0) + F::new(5.0) / F::new(4608.0) * t19051 * t4989 + t15737 * t18948 / F::new(256.0) + t72542 / F::new(216.0) - F::new(19.0) / F::new(864.0) * t66622 * t4950 - t3577 * t3578 * t22244 * t1090 / F::new(4608.0) - F::new(19.0) / F::new(864.0) * t66622 * t4954;
    t72552
}
