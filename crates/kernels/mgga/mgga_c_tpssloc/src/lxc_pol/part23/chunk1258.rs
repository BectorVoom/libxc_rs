//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1258/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1258<F: Float>(t300: F, t77343: F, t77370: F, t77390: F, t77471: F, t10629: F, t2932: F, t76637: F, t959: F, t2929: F, t77139: F, t77153: F, t77157: F, t77159: F, t77224: F, t77226: F, t77229: F, t77232: F, t77236: F, t77470: F) -> (F, F, F, F) {
    let t77474 = t300 * (t77343 + t77370 + t77390 + t77471);
    let t77478 = 0.6233709278045326953e3 * t959 * t10629 * t76637 * t2932;
    let t77482 = 0.51947577317044391277e2 * t959 * t2929 * t77139 * t2932;
    let t77483 = -t77153 + t77157 + t77159 - t77224 + t77226 - t77229 - t77232 + t77236 + t77474 - t77478 - t77470 - t77482;
    (t77474, t77478, t77482, t77483)
}
