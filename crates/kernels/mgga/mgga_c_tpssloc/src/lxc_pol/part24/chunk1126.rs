//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1126/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1126<F: Float>(t2770: F, t381: F, t254: F, t382: F, t10164: F, t1955: F, t343: F, t6690: F, t28: F, t870: F, t10143: F, t1868: F, t671: F, t1982: F, t8944: F, t12461: F, t2018: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25721 = t381 * t2770;
    let t25757 = t382 * t254;
    let t25758 = t10164 * t1955;
    let t25796 = t343 * t381;
    let t25797 = t25796 * t6690;
    let t25891 = t870 * t28;
    let t25927 = t10143 * t28;
    let t26103 = t1868 * t671;
    let t26161 = t1982 * t8944;
    let t26162 = t2018 * t12461;
    (t25721, t25757, t25758, t25797, t25891, t25927, t26103, t26161, t26162)
}
