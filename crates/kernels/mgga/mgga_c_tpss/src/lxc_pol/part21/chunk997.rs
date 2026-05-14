//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 997/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk997<F: Float>(t3678: F, t8313: F, t2175: F, t2370: F, t3676: F, t3628: F, t3629: F, t8306: F, t8307: F, t10777: F, t10779: F, t10782: F, t10786: F, t10790: F, t10795: F, t10799: F, t2173: F, t3626: F, t8289: F, t8293: F, t8314: F) -> (F, F, F, F) {
    let t10803 = 7.0 / 576.0 * t8313 * t3678;
    let t10805 = t2175 * t3676 * t2370;
    let t10809 = t3628 * t3629 * t2370;
    let t10813 = t8306 * t3629 * t8307;
    let t10816 = 7.0 / 4608.0 * t8289 - 7.0 / 2304.0 * t8293 - 7.0 / 576.0 * t8314 - t10777 - t10779 * t10782 / 512.0 + t3626 * t10786 / 512.0 - t3626 * t10790 / 384.0 + t2173 * t10795 / 384.0 + t2173 * t10799 / 768.0 - t10803 + t2173 * t10805 / 768.0 - t2173 * t10809 / 3072.0 - 5.0 / 768.0 * t2173 * t10813;
    (t10805, t10809, t10813, t10816)
}
