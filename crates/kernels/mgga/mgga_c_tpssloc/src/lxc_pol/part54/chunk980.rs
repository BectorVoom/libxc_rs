//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 980/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk980<F: Float>(t225: F, t7910: F, t26231: F, t26251: F, t26255: F, t26266: F, t22785: F, t22795: F, t26258: F, t26260: F, t26262: F, t26268: F, t26272: F, t26274: F, t26278: F, t22856: F, t22861: F, t24058: F, t24060: F, t24061: F, t26306: F, t26310: F, t26312: F, t26314: F, t26320: F, t26324: F) -> (F, F, F, F, F) {
    let t27009 = t7910 * t225;
    let t27012 = 7.0 / 1152.0 * t26231;
    let t27019 = 7.0 / 1152.0 * t26251;
    let t27022 = 7.0 / 288.0 * t26255;
    let t27027 = 7.0 / 72.0 * t26266;
    let t27032 = t27022 - t26258 / 192.0 - t26260 / 192.0 - t26262 / 192.0 + t22785 + 0.40372756094140390853e-3 * t22795 + t27027 + 0.16956557559538964158e-1 * t26268 + 0.40372756094140390853e-3 * t26272 - t26274 / 24.0 - 0.24223653656484234512e-2 * t26278;
    let t27049 = t26306 / 192.0 + t26310 / 384.0 - t26312 / 768.0 + t26314 / 192.0 + 0.67287926823567318088e-4 * t22856 + t24058 - t22861 + t24060 + t24061 + 0.80745512188280781706e-3 * t26320 - 0.40372756094140390853e-3 * t26324;
    (t27009, t27012, t27019, t27032, t27049)
}
