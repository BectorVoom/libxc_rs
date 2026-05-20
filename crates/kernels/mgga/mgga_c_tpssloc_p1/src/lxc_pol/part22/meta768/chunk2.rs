//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2604/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2604<F: Float>(t15503: F, t18356: F, t18975: F, t5024: F, t1174: F, t21749: F, t3431: F, t11738: F, t15569: F, t15740: F, t1735: F, t18225: F, t18300: F, t18321: F, t18387: F, t18969: F, t19068: F, t3577: F, t3578: F, t4582: F, t4950: F, t4954: F, t4969: F, t4980: F, t5012: F, t65541: F, t65815: F, t65935: F) -> F {
    let t72632 = t15503 * t18356;
    let t72634 = t5024 * t18975;
    let t72648 = t1174 * t3431 * t21749;
    let t72654 = t15569 * t18387 / F::new(144.0) - t65815 * t4954 / F::new(1536.0) - t15740 * t18969 / F::new(1536.0) - t65815 * t4950 / F::new(1536.0) - t72632 / F::new(144.0) - F::new(5.0) / F::new(1296.0) * t72634 - F::new(5.0) / F::new(20736.0) * t65935 + F::new(19.0) / F::new(288.0) * t65541 * t4980 - t3577 * t3578 * t1735 * t18225 / F::new(384.0) + t11738 * t4582 * t18300 * t5012 / F::new(1024.0) - t72648 / F::new(144.0) - F::new(11.0) / F::new(54.0) * t18321 * t4969 - F::new(5.0) / F::new(864.0) * t5024 * t19068;
    t72654
}
