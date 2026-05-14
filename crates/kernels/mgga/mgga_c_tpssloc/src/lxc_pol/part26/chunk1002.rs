//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1002/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1002<F: Float>(t1369: F, t22788: F, t6597: F, t6924: F, t281: F, t1307: F, t1361: F, t22690: F, t547: F, t6546: F, t1329: F, t3770: F, t6916: F, t22754: F, t22757: F, t22762: F, t22767: F, t22768: F, t22771: F, t22774: F, t22777: F, t22780: F, t22785: F, t22786: F) -> (F, F, F, F, F) {
    let t22789 = t22788 * t1369;
    let t22791 = t6597 * t6924;
    let t22792 = t22791 * t281;
    let t22794 = t22690 * t1361 * t1307;
    let t22795 = t22792 * t22794;
    let t22797 = t6546 * t547;
    let t22798 = t22797 * t1329;
    let t22799 = 7.0 / 72.0 * t22798;
    let t22800 = t6916 * t3770;
    let t22802 = -t22754 / 1536.0 - t22757 / 768.0 + t22762 / 768.0 + t22767 - t22768 / 1536.0 - 0.20186378047070195427e-3 * t22771 - 0.20186378047070195427e-3 * t22774 + 0.40372756094140390854e-3 * t22777 + 0.28260929265898273598e-2 * t22780 + t22785 - t22786 / 384.0 - t22789 / 192.0 + 0.40372756094140390854e-3 * t22795 + t22799 - t22800 / 48.0;
    (t22791, t22792, t22794, t22797, t22802)
}
