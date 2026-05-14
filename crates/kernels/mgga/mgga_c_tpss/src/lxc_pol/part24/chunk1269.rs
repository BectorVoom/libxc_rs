//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1269/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1269<F: Float>(t114: F, t61869: F, t61871: F, t65443: F, t65445: F, t67531: F, t68872: F, t68875: F, t68878: F, t68880: F, t68883: F, t68885: F, t485: F, t626: F, t4637: F, t5511: F, t19579: F, t19580: F, t51664: F) -> (F, F, F, F) {
    let t115 = 1.0 < t114;
    let t68887 = -t61869 - 11.0 / 9.0 * t61871 - t67531 - t65443 + t65445 - 2.0 / 3.0 * t68872 - 3.0 / 4.0 * t68875 + t68878 / 2.0 + t68880 / 3.0 + t68883 / 4.0 - t68885 / 8.0;
    let t68888 = piecewise3(t115, 0.0, t68887);
    let t68891 = 2.0 * t626 * t485 * t68888;
    let t68898 = t5511 * t4637;
    let t68905 = 2.0 * t19579 * t19580 * t51664;
    (t68888, t68891, t68898, t68905)
}
